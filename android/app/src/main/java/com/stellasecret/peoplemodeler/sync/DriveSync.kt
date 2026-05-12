package com.stellasecret.peoplemodeler.sync

import com.stellasecret.peoplemodeler.R

import android.content.Context
import android.util.Log
import com.google.gson.Gson
import com.stellasecret.peoplemodeler.data.models.Person
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import org.json.JSONObject
import java.io.BufferedReader
import java.io.InputStreamReader
import java.io.OutputStreamWriter
import java.net.HttpURLConnection
import java.net.URL

// ─── Sync State ───────────────────────────────────────────

sealed class SyncResult {
    data class Success(val message: String, val count: Int = 0) : SyncResult()
    data class Error(val message: String, val cause: Exception? = null) : SyncResult()
    object NotAuthenticated : SyncResult()
}

// ─── DriveSync — REST API directe avec idToken ────────────
//
// On utilise l'API Drive REST directement via HttpURLConnection
// au lieu de GoogleAccountCredential (qui nécessite un compte
// Android enregistré dans AccountManager — incompatible avec
// le flux Credential Manager moderne).
//
// Auth : on échange l'idToken contre un access_token via
// le endpoint token de Google, puis on appelle Drive REST.

class DriveSync(
    private val context: Context,
    private val authManager: GoogleAuthManager,
) {
    private val gson = Gson()

    private val BACKUP_FILENAME = "people_modeler_backup.json"
    private val FOLDER_APPDATA  = "appDataFolder"
    private val DRIVE_FILES_URL = "https://www.googleapis.com/drive/v3/files"
    private val DRIVE_UPLOAD_URL = "https://www.googleapis.com/upload/drive/v3/files"

    // ── Get access token from idToken ─────────────────────
    // On utilise le flux "urn:ietf:params:oauth:grant-type:jwt-bearer"
    // pour obtenir un access_token depuis l'idToken

    private suspend fun getAccessToken(): String? = withContext(Dispatchers.IO) {
        val idToken = authManager.currentIdToken ?: return@withContext null
        val clientId = context.getString(R.string.google_client_id)

        return@withContext try {
            val url = URL("https://oauth2.googleapis.com/token")
            val conn = url.openConnection() as HttpURLConnection
            conn.requestMethod = "POST"
            conn.doOutput = true
            conn.setRequestProperty("Content-Type", "application/x-www-form-urlencoded")

            val body = "grant_type=urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Ajwt-bearer" +
                "&assertion=${idToken}" +
                "&client_id=${clientId}" +
                "&scope=https%3A%2F%2Fwww.googleapis.com%2Fauth%2Fdrive.appdata"

            OutputStreamWriter(conn.outputStream).use { it.write(body) }

            val response = BufferedReader(InputStreamReader(conn.inputStream)).readText()
            val json = JSONObject(response)
            val token = json.optString("access_token", null)
            Log.i(TAG, if (token != null) "✅ Access token obtenu" else "❌ Pas d'access_token dans la réponse")
            token
        } catch (e: Exception) {
            Log.e(TAG, "❌ getAccessToken failed: ${e.message}", e)
            // Fallback: use idToken directly as Bearer (works for some Drive operations)
            // This won't work for drive.appdata but helps diagnose
            null
        }
    }

    // ── Backup ────────────────────────────────────────────

    suspend fun backup(persons: List<Person>): SyncResult = withContext(Dispatchers.IO) {
        if (!authManager.isSignedIn) return@withContext SyncResult.NotAuthenticated

        val accessToken = getAccessToken()
            ?: return@withContext SyncResult.Error(
                "Impossible d'obtenir un access token. " +
                "Vérifiez que le scope 'drive.appdata' est activé dans Google Cloud Console."
            )

        return@withContext try {
            val payload = gson.toJson(
                BackupPayload(
                    version   = BACKUP_VERSION,
                    timestamp = System.currentTimeMillis(),
                    persons   = persons
                )
            )

            val existingId = findBackupFileId(accessToken)

            if (existingId != null) {
                updateFile(accessToken, existingId, payload)
                Log.i(TAG, "✅ Backup mis à jour (${persons.size} profils)")
            } else {
                createFile(accessToken, payload)
                Log.i(TAG, "✅ Backup créé (${persons.size} profils)")
            }

            SyncResult.Success("Sauvegarde réussie — ${persons.size} profil(s) sur Google Drive", persons.size)
        } catch (e: Exception) {
            Log.e(TAG, "❌ Backup failed", e)
            SyncResult.Error("Échec de la sauvegarde : ${e.message}", e)
        }
    }

    // ── Restore ───────────────────────────────────────────

    suspend fun restore(): Pair<SyncResult, List<Person>?> = withContext(Dispatchers.IO) {
        if (!authManager.isSignedIn) return@withContext Pair(SyncResult.NotAuthenticated, null)

        val accessToken = getAccessToken()
            ?: return@withContext Pair(SyncResult.Error("Impossible d'obtenir un access token"), null)

        return@withContext try {
            val fileId = findBackupFileId(accessToken)
                ?: return@withContext Pair(SyncResult.Error("Aucune sauvegarde trouvée sur Drive"), null)

            val content = downloadFile(accessToken, fileId)
            val payload = gson.fromJson(content, BackupPayload::class.java)

            Log.i(TAG, "✅ Restore réussi (${payload.persons.size} profils, v${payload.version})")
            Pair(
                SyncResult.Success("Restauration réussie — ${payload.persons.size} profil(s) importés", payload.persons.size),
                payload.persons
            )
        } catch (e: Exception) {
            Log.e(TAG, "❌ Restore failed", e)
            Pair(SyncResult.Error("Échec de la restauration : ${e.message}", e), null)
        }
    }

    // ── Get backup info ───────────────────────────────────

    suspend fun getBackupInfo(): BackupInfo? = withContext(Dispatchers.IO) {
        if (!authManager.isSignedIn) return@withContext null
        val accessToken = getAccessToken() ?: return@withContext null
        return@withContext try {
            val fileId = findBackupFileId(accessToken) ?: return@withContext null
            val url = URL("$DRIVE_FILES_URL/$fileId?fields=id,name,modifiedTime,size")
            val conn = url.openConnection() as HttpURLConnection
            conn.setRequestProperty("Authorization", "Bearer $accessToken")
            val json = JSONObject(BufferedReader(InputStreamReader(conn.inputStream)).readText())
            BackupInfo(
                fileId       = json.getString("id"),
                modifiedTime = parseRfc3339(json.optString("modifiedTime")),
                sizeBytes    = json.optLong("size", 0L),
            )
        } catch (e: Exception) {
            Log.e(TAG, "getBackupInfo failed", e)
            null
        }
    }

    // ── Internal REST helpers ─────────────────────────────

    private fun findBackupFileId(accessToken: String): String? {
        val q = "name='$BACKUP_FILENAME'"
        val url = URL("$DRIVE_FILES_URL?spaces=$FOLDER_APPDATA&q=${java.net.URLEncoder.encode(q, "UTF-8")}&fields=files(id,name)")
        val conn = url.openConnection() as HttpURLConnection
        conn.setRequestProperty("Authorization", "Bearer $accessToken")
        val response = BufferedReader(InputStreamReader(conn.inputStream)).readText()
        val files = JSONObject(response).optJSONArray("files")
        return if (files != null && files.length() > 0) files.getJSONObject(0).getString("id") else null
    }

    private fun createFile(accessToken: String, content: String) {
        // Multipart upload
        val boundary = "==boundary=="
        val url = URL("$DRIVE_UPLOAD_URL?uploadType=multipart")
        val conn = url.openConnection() as HttpURLConnection
        conn.requestMethod = "POST"
        conn.doOutput = true
        conn.setRequestProperty("Authorization", "Bearer $accessToken")
        conn.setRequestProperty("Content-Type", "multipart/related; boundary=$boundary")

        val metadata = """{"name":"$BACKUP_FILENAME","parents":["$FOLDER_APPDATA"]}"""
        val body = "--$boundary\r\nContent-Type: application/json; charset=UTF-8\r\n\r\n$metadata\r\n" +
            "--$boundary\r\nContent-Type: application/json\r\n\r\n$content\r\n--$boundary--"
        OutputStreamWriter(conn.outputStream).use { it.write(body) }

        val code = conn.responseCode
        if (code !in 200..299) throw Exception("createFile HTTP $code")
    }

    private fun updateFile(accessToken: String, fileId: String, content: String) {
        val url = URL("$DRIVE_UPLOAD_URL/$fileId?uploadType=media")
        val conn = url.openConnection() as HttpURLConnection
        conn.requestMethod = "PATCH"
        conn.doOutput = true
        conn.setRequestProperty("Authorization", "Bearer $accessToken")
        conn.setRequestProperty("Content-Type", "application/json")
        OutputStreamWriter(conn.outputStream).use { it.write(content) }
        val code = conn.responseCode
        if (code !in 200..299) throw Exception("updateFile HTTP $code")
    }

    private fun downloadFile(accessToken: String, fileId: String): String {
        val url = URL("$DRIVE_FILES_URL/$fileId?alt=media")
        val conn = url.openConnection() as HttpURLConnection
        conn.setRequestProperty("Authorization", "Bearer $accessToken")
        return BufferedReader(InputStreamReader(conn.inputStream)).readText()
    }

    private fun parseRfc3339(s: String): Long {
        return try { java.text.SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss.SSS'Z'", java.util.Locale.US).parse(s)?.time ?: 0L }
        catch (e: Exception) { 0L }
    }

    // ── Data models ───────────────────────────────────────

    data class BackupPayload(val version: Int, val timestamp: Long, val persons: List<Person>)
    data class BackupInfo(val fileId: String, val modifiedTime: Long, val sizeBytes: Long)

    companion object {
        private const val TAG = "DriveSync"
        private const val BACKUP_VERSION = 1
    }
}
