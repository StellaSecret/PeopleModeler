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

    // App Data folder — visible only by this app, not browsable by user
    // Pas besoin de permission Drive complète, juste drive.appdata
    private val BACKUP_FILENAME = "people_modeler_backup.json"
    private val MIME_JSON = "application/json"
    private val FOLDER_APPDATA = "appDataFolder"

    // ── Build Drive service ───────────────────────────────

    private fun buildDriveService(): Drive? {
        val credential = authManager.getDriveCredential() ?: return null
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
                ?: return@withContext SyncResult.Error("Impossible de créer le service Drive")

            val payload = BackupPayload(
                version = BACKUP_VERSION,
                timestamp = System.currentTimeMillis(),
                persons = persons
            )
            val json = gson.toJson(payload)
            val content = ByteArrayContent(MIME_JSON, json.toByteArray(Charsets.UTF_8))

            // Check if backup already exists
            val existingId = findBackupFileId(drive)

            if (existingId != null) {
                // Update existing file
                drive.files().update(existingId, null, content).execute()
                Log.i(TAG, "✅ Backup mis à jour (${persons.size} profils)")
            } else {
                // Create new file in App Data folder
                val fileMetadata = File().apply {
                    name = BACKUP_FILENAME
                    parents = listOf(FOLDER_APPDATA)
                }
                drive.files().create(fileMetadata, content)
                    .setFields("id, name, modifiedTime")
                    .execute()
                Log.i(TAG, "✅ Backup créé (${persons.size} profils)")
            }

            SyncResult.Success(
                "Sauvegarde réussie — ${persons.size} profil(s) sur Google Drive",
                persons.size
            )
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
                ?: return@withContext Pair(SyncResult.Error("Impossible de créer le service Drive"), null)

            val fileId = findBackupFileId(drive)
                ?: return@withContext Pair(
                    SyncResult.Error("Aucune sauvegarde trouvée sur Drive"),
                    null
                )

            val outputStream = ByteArrayOutputStream()
            drive.files().get(fileId).executeMediaAndDownloadTo(outputStream)
            val json = outputStream.toString(Charsets.UTF_8.name())

            val payload = gson.fromJson(json, BackupPayload::class.java)
            val persons = payload.persons

            Log.i(TAG, "✅ Restore réussi (${persons.size} profils, v${payload.version})")
            Pair(
                SyncResult.Success(
                    "Restauration réussie — ${persons.size} profil(s) importés",
                    persons.size
                ),
                persons
            )
        } catch (e: Exception) {
            Log.e(TAG, "❌ Restore failed", e)
            Pair(SyncResult.Error("Échec de la restauration : ${e.message}", e), null)
        }
    }

    // ── Get backup info ───────────────────────────────────

    suspend fun getBackupInfo(): BackupInfo? = withContext(Dispatchers.IO) {
        if (!authManager.isSignedIn) return@withContext null
        return@withContext try {
            val drive = buildDriveService() ?: return@withContext null
            val fileId = findBackupFileId(drive) ?: return@withContext null
            val file = drive.files().get(fileId)
                .setFields("id, name, modifiedTime, size")
                .execute()
            BackupInfo(
                fileId = file.id,
                modifiedTime = file.modifiedTime?.value ?: 0L,
                sizeBytes = file.getSize() ?: 0L,
            )
        } catch (e: Exception) {
            Log.e(TAG, "getBackupInfo failed", e)
            null
        }
    }

    // ── Internal helpers ──────────────────────────────────

    private fun findBackupFileId(drive: Drive): String? {
        val result = drive.files().list()
            .setSpaces(FOLDER_APPDATA)
            .setQ("name = '$BACKUP_FILENAME'")
            .setFields("files(id, name)")
            .execute()
        return result.files?.firstOrNull()?.id
    }

    // ── Data models ───────────────────────────────────────

    data class BackupPayload(
        val version: Int,
        val timestamp: Long,
        val persons: List<Person>,
    )

    data class BackupInfo(
        val fileId: String,
        val modifiedTime: Long,
        val sizeBytes: Long,
    )

    companion object {
        private const val TAG = "DriveSync"
        private const val BACKUP_VERSION = 1
    }
}
