package com.stellasecret.peoplemodeler.sync

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.viewModelScope
import com.stellasecret.peoplemodeler.data.repository.AppDatabase
import com.stellasecret.peoplemodeler.data.repository.PersonRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class SyncViewModel(
    application: Application,
) : AndroidViewModel(application) {
    val authManager = GoogleAuthManager(application)
    private val driveSync = DriveSync(application, authManager)
    private val repo = PersonRepository(AppDatabase.getInstance(application))

    val authState = authManager.authState

    private val _syncState = MutableStateFlow<SyncUiState>(SyncUiState.Idle)
    val syncState: StateFlow<SyncUiState> = _syncState.asStateFlow()

    private val _backupInfo = MutableStateFlow<DriveSync.BackupInfo?>(null)
    val backupInfo: StateFlow<DriveSync.BackupInfo?> = _backupInfo.asStateFlow()

    // ── Actions ────────────────────────────────────────────

    fun signOut() =
        viewModelScope.launch {
            authManager.signOut()
            _syncState.value = SyncUiState.Idle
            _backupInfo.value = null
        }

    fun backup() =
        viewModelScope.launch {
            _syncState.value = SyncUiState.Loading("Sauvegarde en cours…")
            val persons = repo.getAllPersonsOnce()
            _syncState.value =
                when (val result = driveSync.backup(persons)) {
                    is SyncResult.Success -> SyncUiState.Done(result.message)
                    is SyncResult.Error -> SyncUiState.Failure(result.message)
                    is SyncResult.NotAuthenticated -> SyncUiState.Failure("Non connecté")
                }
            refreshBackupInfo()
        }

    suspend fun countPersons() = repo.countPersons()

    fun restore(overwrite: Boolean = false) =
        viewModelScope.launch {
            _syncState.value = SyncUiState.Loading("Restauration en cours…")
            val (result, persons) = driveSync.restore()
            when (result) {
                is SyncResult.Success -> {
                    if (overwrite) repo.deleteAllPersons()
                    persons?.forEach { repo.savePerson(it) }
                    _syncState.value = SyncUiState.Done(result.message)
                }

                is SyncResult.Error -> {
                    _syncState.value = SyncUiState.Failure(result.message)
                }

                is SyncResult.NotAuthenticated -> {
                    _syncState.value = SyncUiState.Failure("Non connecté")
                }
            }
        }

    fun refreshBackupInfo() =
        viewModelScope.launch {
            _backupInfo.value = driveSync.getBackupInfo()
        }

    fun resetSyncState() {
        _syncState.value = SyncUiState.Idle
    }
}

// ─── UI State ─────────────────────────────────────────────

sealed class SyncUiState {
    object Idle : SyncUiState()

    data class Loading(
        val message: String,
    ) : SyncUiState()

    data class Done(
        val message: String,
    ) : SyncUiState()

    data class Failure(
        val message: String,
    ) : SyncUiState()
}
