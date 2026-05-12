package com.stellasecret.peoplemodeler.sync

import com.stellasecret.peoplemodeler.R

import android.content.Context
import android.util.Log
import androidx.credentials.CredentialManager
import androidx.credentials.CustomCredential
import androidx.credentials.GetCredentialRequest
import androidx.credentials.exceptions.GetCredentialException
import com.google.android.libraries.identity.googleid.GetGoogleIdOption
import com.google.android.libraries.identity.googleid.GoogleIdTokenCredential
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow

// ─── Auth State ───────────────────────────────────────────

sealed class AuthState {
    object SignedOut : AuthState()
    data class SignedIn(
        val email: String,
        val displayName: String,
        val idToken: String,
        // Access token obtenu via authorization code exchange
        // Null tant qu'on utilise uniquement l'idToken
        val accessToken: String? = null,
    ) : AuthState()
    data class Error(val message: String) : AuthState()
}

// ─── GoogleAuthManager ────────────────────────────────────

class GoogleAuthManager(private val context: Context) {

    private val _authState = MutableStateFlow<AuthState>(AuthState.SignedOut)
    val authState: StateFlow<AuthState> = _authState

    val isSignedIn: Boolean
        get() = _authState.value is AuthState.SignedIn

    val currentEmail: String?
        get() = (_authState.value as? AuthState.SignedIn)?.email

    val currentIdToken: String?
        get() = (_authState.value as? AuthState.SignedIn)?.idToken

    private val credentialManager = CredentialManager.create(context)

    init { restoreSession() }

    // ── Sign In ───────────────────────────────────────────

    suspend fun signIn(activityContext: Context): Result<AuthState.SignedIn> {
        return try {
            val googleIdOption = GetGoogleIdOption.Builder()
                .setFilterByAuthorizedAccounts(false)
                .setServerClientId(context.getString(R.string.google_client_id))
                .setAutoSelectEnabled(false)
                .build()

            val request = GetCredentialRequest.Builder()
                .addCredentialOption(googleIdOption)
                .build()

            val result = credentialManager.getCredential(
                request = request,
                context = activityContext,
            )

            val credential = result.credential
            if (credential is CustomCredential &&
                credential.type == GoogleIdTokenCredential.TYPE_GOOGLE_ID_TOKEN_CREDENTIAL
            ) {
                val googleIdToken = GoogleIdTokenCredential.createFrom(credential.data)
                val signedIn = AuthState.SignedIn(
                    email       = googleIdToken.id,
                    displayName = googleIdToken.displayName ?: googleIdToken.id,
                    idToken     = googleIdToken.idToken,
                )
                _authState.value = signedIn
                saveSession(signedIn)
                Log.i(TAG, "✅ Signed in: ${signedIn.email}")
                Result.success(signedIn)
            } else {
                Result.failure(Exception("Type de credential non supporté"))
            }
        } catch (e: GetCredentialException) {
            val error = "Échec de la connexion Google : ${e.message}"
            _authState.value = AuthState.Error(error)
            Log.e(TAG, error, e)
            Result.failure(e)
        }
    }

    // ── Sign Out ──────────────────────────────────────────

    fun signOut() {
        _authState.value = AuthState.SignedOut
        clearSession()
        Log.i(TAG, "✅ Signed out")
    }

    // ── Session persistence ───────────────────────────────

    private fun saveSession(state: AuthState.SignedIn) {
        context.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit()
            .putString(KEY_EMAIL, state.email)
            .putString(KEY_NAME, state.displayName)
            .putString(KEY_TOKEN, state.idToken)
            .apply()
    }

    private fun restoreSession() {
        val prefs = context.getSharedPreferences(PREFS, Context.MODE_PRIVATE)
        val email = prefs.getString(KEY_EMAIL, null)
        val name  = prefs.getString(KEY_NAME, null)
        val token = prefs.getString(KEY_TOKEN, null)
        if (email != null && name != null && token != null) {
            _authState.value = AuthState.SignedIn(email, name, token)
            Log.i(TAG, "✅ Session restaurée : $email")
        }
    }

    private fun clearSession() {
        context.getSharedPreferences(PREFS, Context.MODE_PRIVATE).edit().clear().apply()
    }

    companion object {
        private const val TAG    = "GoogleAuthManager"
        private const val PREFS  = "pm_auth_prefs"
        private const val KEY_EMAIL = "email"
        private const val KEY_NAME  = "display_name"
        private const val KEY_TOKEN = "id_token"
    }
}
