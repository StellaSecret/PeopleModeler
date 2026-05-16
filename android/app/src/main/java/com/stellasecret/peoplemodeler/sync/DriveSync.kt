package com.stellasecret.peoplemodeler.sync

import android.content.Context
import android.util.Log
import com.google.api.client.http.ByteArrayContent
import com.google.api.client.json.gson.GsonFactory
import com.google.api.services.drive.Drive
import com.google.api.services.drive.model.File
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import com.stellasecret.peoplemodeler.data.models.Person
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.ByteArrayOutputStream

// ─── Sync State ───────────────────────────────────────────

sealed class SyncResult {
    data class Success(val message: String, val count: Int = 0) : SyncResult()
    data class Error(val message: String, val cause: Exception? = null) : SyncResult()
    object NotAuthenticated : SyncResult()
}

// ─── DriveSync ────────────────────────────────────────────

class DriveSync(
    private val context: Context,
    private val authManager: GoogleAuthManager,
) {
    private val gson = Gson()

    private val BACKUP_FILENAME = "people_modeler_backup.json"
    private val MIME_JSON       = "application/json"
    private val FOLDER_APPDATA  = "appDataFolder"

    // ── Build Drive service via GoogleAccountCredential ───
    // GoogleSignIn already obtained drive.appdata scope —
    // GoogleAccountCredential uses the token from GMS directly.

    private fun buildDriveService(): Drive? {
        val credential = authManager.getDriveCredential() ?: run {
            Log.w(TAG, "⚠️ No Drive credential — user not signed in or scope missing")
            return null
        }
        return Drive.Builder(
            com.google.api.client.http.javanet.NetHttpTransport(),
            GsonFactory.getDefaultInstance(),
            credential
        )
            .setApplicationName("People Modeler")
            .build()
    }

    // ── Backup ────────────────────────────────────────────

    suspend fun backup(persons: List<Person>): SyncResult = withContext(Dispatchers.IO) {
        if (!authManager.isSignedIn) return@withContext SyncResult.NotAuthenticated

        return@withContext try {
            val drive = buildDriveService()
                ?: return@withContext SyncResult.Error("Non connecté ou scope Drive manquant")

            val payload = BackupPayload(BACKUP_VERSION, System.currentTimeMillis(), persons)
            val json    = gson.toJson(payload)
            val content = ByteArrayContent(MIME_JSON, json.toByteArray(Charsets.UTF_8))

            val existingId = findBackupFileId(drive)
            if (existingId != null) {
                drive.files().update(existingId, null, content).execute()
                Log.i(TAG, "✅ Backup mis à jour (${persons.size} profils)")
            } else {
                val meta = File().apply {
                    name    = BACKUP_FILENAME
                    parents = listOf(FOLDER_APPDATA)
                }
                drive.files().create(meta, content).setFields("id,name").execute()
                Log.i(TAG, "✅ Backup créé (${persons.size} profils)")
            }

            SyncResult.Success("Sauvegarde réussie — ${persons.size} profil(s)", persons.size)
        } catch (e: Exception) {
            Log.e(TAG, "❌ Backup failed", e)
            SyncResult.Error("Échec de la sauvegarde : ${e.message}", e)
        }
    }

    // ── Restore ───────────────────────────────────────────

    suspend fun restore(): Pair<SyncResult, List<Person>?> = withContext(Dispatchers.IO) {
        if (!authManager.isSignedIn) return@withContext Pair(SyncResult.NotAuthenticated, null)

        return@withContext try {
            val drive = buildDriveService()
                ?: return@withContext Pair(SyncResult.Error("Non connecté ou scope Drive manquant"), null)

            val fileId = findBackupFileId(drive)
                ?: return@withContext Pair(SyncResult.Error("Aucune sauvegarde trouvée sur Drive"), null)

            val out = ByteArrayOutputStream()
            drive.files().get(fileId).executeMediaAndDownloadTo(out)
            val json = out.toString(Charsets.UTF_8.name())
            val type = object : TypeToken<BackupPayload>() {}.type
            val payload: BackupPayload = gson.fromJson(json, type)

            Log.i(TAG, "✅ Restore réussi (${payload.persons.size} profils, v${payload.version})")
            Pair(
                SyncResult.Success("Restauration réussie — ${payload.persons.size} profil(s)", payload.persons.size),
                payload.persons
            )
        } catch (e: Exception) {
            Log.e(TAG, "❌ Restore failed", e)
            Pair(SyncResult.Error("Échec : ${e.message}", e), null)
        }
    }

    // ── Backup info ───────────────────────────────────────

    suspend fun getBackupInfo(): BackupInfo? = withContext(Dispatchers.IO) {
        if (!authManager.isSignedIn) return@withContext null
        return@withContext try {
            val drive  = buildDriveService() ?: return@withContext null
            val fileId = findBackupFileId(drive) ?: return@withContext null
            val file   = drive.files().get(fileId)
                .setFields("id,name,modifiedTime,size")
                .execute()
            BackupInfo(
                fileId       = file.id,
                modifiedTime = file.modifiedTime?.value ?: 0L,
                sizeBytes    = file.getSize() ?: 0L,
            )
        } catch (e: Exception) {
            Log.e(TAG, "getBackupInfo failed: ${e.message}")
            null
        }
    }

    // ── Internal ──────────────────────────────────────────

    private fun findBackupFileId(drive: Drive): String? {
        val result = drive.files().list()
            .setSpaces(FOLDER_APPDATA)
            .setQ("name = '$BACKUP_FILENAME'")
            .setFields("files(id,name)")
            .execute()
        return result.files?.firstOrNull()?.id
    }

    // ── Models ────────────────────────────────────────────

    data class BackupPayload(val version: Int, val timestamp: Long, val persons: List<Person>)
    data class BackupInfo(val fileId: String, val modifiedTime: Long, val sizeBytes: Long)

    companion object {
        private const val TAG = "DriveSync"
        private const val BACKUP_VERSION = 1
    }
}
