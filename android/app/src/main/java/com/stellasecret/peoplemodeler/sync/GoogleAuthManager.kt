package com.stellasecret.peoplemodeler.sync

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.util.Log
import com.google.android.gms.auth.api.signin.GoogleSignIn
import com.google.android.gms.auth.api.signin.GoogleSignInAccount
import com.google.android.gms.auth.api.signin.GoogleSignInClient
import com.google.android.gms.auth.api.signin.GoogleSignInOptions
import com.google.android.gms.common.api.ApiException
import com.google.android.gms.common.api.Scope
import com.google.api.client.googleapis.extensions.android.gms.auth.GoogleAccountCredential
import com.google.api.client.util.ExponentialBackOff
import com.google.api.services.drive.DriveScopes
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.tasks.await

// ─── Auth State ───────────────────────────────────────────

sealed class AuthState {
    object SignedOut : AuthState()

    data class SignedIn(
        val email: String,
        val displayName: String,
    ) : AuthState()

    data class Error(
        val message: String,
    ) : AuthState()
}

// ─── GoogleAuthManager ────────────────────────────────────
// Uses GoogleSignIn (not Credential Manager) to properly request
// drive.appdata scope alongside the user identity.

class GoogleAuthManager(
    private val context: Context,
) {
    companion object {
        private const val TAG = "GoogleAuthManager"
        const val RC_SIGN_IN = 9001
    }

    private val _authState = MutableStateFlow<AuthState>(AuthState.SignedOut)
    val authState: StateFlow<AuthState> = _authState

    val isSignedIn: Boolean
        get() = _authState.value is AuthState.SignedIn

    val currentEmail: String?
        get() = (_authState.value as? AuthState.SignedIn)?.email

    // ── GoogleSignIn client with drive.appdata scope ───────
    private val gso =
        GoogleSignInOptions
            .Builder(GoogleSignInOptions.DEFAULT_SIGN_IN)
            .requestEmail()
            .requestScopes(Scope(DriveScopes.DRIVE_APPDATA))
            .build()

    private val signInClient: GoogleSignInClient =
        GoogleSignIn.getClient(context, gso)

    init {
        // Restore session from last signed-in account
        val lastAccount = GoogleSignIn.getLastSignedInAccount(context)
        if (lastAccount != null && lastAccount.email != null) {
            _authState.value =
                AuthState.SignedIn(
                    email = lastAccount.email!!,
                    displayName = lastAccount.displayName ?: lastAccount.email!!,
                )
            Log.i(TAG, "✅ Session restaurée : ${lastAccount.email}")
        }
    }

    // ── Sign In Intent — launch from Activity ─────────────
    // Call this from SyncFragment, then handle result in onActivityResult

    fun getSignInIntent(): Intent = signInClient.signInIntent

    // ── Handle result from onActivityResult ───────────────

    fun handleSignInResult(data: Intent?): Result<AuthState.SignedIn> {
        return try {
            val task = GoogleSignIn.getSignedInAccountFromIntent(data)
            val account = task.getResult(ApiException::class.java)
            if (account?.email == null) {
                return Result.failure(Exception("Email null après connexion"))
            }
            val signedIn =
                AuthState.SignedIn(
                    email = account.email!!,
                    displayName = account.displayName ?: account.email!!,
                )
            _authState.value = signedIn
            Log.i(TAG, "✅ Signed in: ${account.email}")
            Result.success(signedIn)
        } catch (e: ApiException) {
            val msg = "Échec connexion Google (code ${e.statusCode}): ${e.message}"
            _authState.value = AuthState.Error(msg)
            Log.e(TAG, msg, e)
            Result.failure(e)
        }
    }

    // ── Silent sign-in (token refresh) ────────────────────

    suspend fun silentSignIn(): Boolean =
        try {
            val account = signInClient.silentSignIn().await()
            if (account?.email != null) {
                _authState.value =
                    AuthState.SignedIn(
                        email = account.email!!,
                        displayName = account.displayName ?: account.email!!,
                    )
                true
            } else {
                false
            }
        } catch (e: Exception) {
            Log.w(TAG, "Silent sign-in failed: ${e.message}")
            false
        }

    // ── Sign Out ──────────────────────────────────────────

    suspend fun signOut() {
        try {
            signInClient.signOut().await()
        } catch (e: Exception) {
            Log.w(TAG, "Sign out error: ${e.message}")
        }
        _authState.value = AuthState.SignedOut
        Log.i(TAG, "✅ Signed out")
    }

    // ── Drive Credential ──────────────────────────────────
    // Returns a credential usable with the Drive REST API client.
    // GoogleSignIn ensures the drive.appdata scope was granted.

    fun getDriveCredential(): GoogleAccountCredential? {
        val account = GoogleSignIn.getLastSignedInAccount(context) ?: return null
        if (account.email == null) return null

        // Verify drive.appdata scope was granted
        val hasScope = GoogleSignIn.hasPermissions(account, Scope(DriveScopes.DRIVE_APPDATA))
        if (!hasScope) {
            Log.w(TAG, "⚠️ drive.appdata scope not granted")
            return null
        }

        return GoogleAccountCredential
            .usingOAuth2(context, listOf(DriveScopes.DRIVE_APPDATA))
            .setBackOff(ExponentialBackOff())
            .also { it.selectedAccount = account.account }
    }
}
