package com.stellasecret.peoplemodeler.sync

import android.app.Activity
import android.content.Intent
import android.os.Bundle
import android.view.*
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.fragment.app.Fragment
import androidx.fragment.app.viewModels
import androidx.lifecycle.lifecycleScope
import com.google.android.material.dialog.MaterialAlertDialogBuilder
import com.google.android.material.snackbar.Snackbar
import com.stellasecret.peoplemodeler.databinding.FragmentSyncBinding
import kotlinx.coroutines.launch
import java.text.SimpleDateFormat
import java.util.*

class SyncFragment : Fragment() {

    private var _binding: FragmentSyncBinding? = null
    private val binding get() = _binding!!
    private val viewModel: SyncViewModel by viewModels()

    // ActivityResultLauncher for GoogleSignIn intent
    private lateinit var signInLauncher: ActivityResultLauncher<Intent>

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // Register before onCreateView
        signInLauncher = registerForActivityResult(
            ActivityResultContracts.StartActivityForResult()
        ) { result ->
            val data = result.data
            val resultCode = result.resultCode
            if (resultCode == Activity.RESULT_OK) {
                val signInResult = viewModel.authManager.handleSignInResult(data)
                signInResult.onFailure { e ->
                    showSnackbar("Connexion échouée : ${e.message}")
                }
            } else {
                showSnackbar("Connexion annulée")
            }
        }
    }

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?, state: Bundle?
    ): View {
        _binding = FragmentSyncBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        setupObservers()
        setupClickListeners()
    }

    // ── Observers ─────────────────────────────────────────

    private fun setupObservers() {
        viewLifecycleOwner.lifecycleScope.launch {
            viewModel.authState.collect { state ->
                when (state) {
                    is AuthState.SignedOut -> renderSignedOut()
                    is AuthState.SignedIn  -> renderSignedIn(state)
                    is AuthState.Error     -> {
                        renderSignedOut()
                        showSnackbar("Erreur : ${state.message}")
                    }
                }
            }
        }

        viewLifecycleOwner.lifecycleScope.launch {
            viewModel.syncState.collect { state ->
                when (state) {
                    is SyncUiState.Idle    -> {
                        binding.progressBar.visibility = View.GONE
                        binding.textSyncStatus.text = ""
                    }
                    is SyncUiState.Loading -> {
                        binding.progressBar.visibility = View.VISIBLE
                        binding.textSyncStatus.text = state.message
                        binding.btnBackup.isEnabled  = false
                        binding.btnRestore.isEnabled = false
                    }
                    is SyncUiState.Done    -> {
                        binding.progressBar.visibility = View.GONE
                        binding.textSyncStatus.text = state.message
                        binding.btnBackup.isEnabled  = true
                        binding.btnRestore.isEnabled = true
                        showSnackbar("✅ ${state.message}")
                        viewModel.resetSyncState()
                    }
                    is SyncUiState.Failure -> {
                        binding.progressBar.visibility = View.GONE
                        binding.textSyncStatus.text = "❌ ${state.message}"
                        binding.btnBackup.isEnabled  = true
                        binding.btnRestore.isEnabled = true
                        showSnackbar("❌ ${state.message}")
                        viewModel.resetSyncState()
                    }
                }
            }
        }

        viewLifecycleOwner.lifecycleScope.launch {
            viewModel.backupInfo.collect { info ->
                if (info != null) {
                    val date = SimpleDateFormat("dd/MM/yyyy HH:mm", Locale.FRANCE)
                        .format(Date(info.modifiedTime))
                    val size = "%.1f KB".format(info.sizeBytes / 1024.0)
                    binding.textBackupInfo.text = "Dernière sauvegarde : $date ($size)"
                    binding.textBackupInfo.visibility = View.VISIBLE
                } else {
                    binding.textBackupInfo.text = "Aucune sauvegarde sur Drive"
                    binding.textBackupInfo.visibility = View.VISIBLE
                }
            }
        }
    }

    // ── Click listeners ───────────────────────────────────

    private fun setupClickListeners() {
        binding.btnSignIn.setOnClickListener {
            // Launch GoogleSignIn intent — handles scope request automatically
            val intent = viewModel.authManager.getSignInIntent()
            signInLauncher.launch(intent)
        }

        binding.btnSignOut.setOnClickListener {
            MaterialAlertDialogBuilder(requireContext())
                .setTitle("Se déconnecter ?")
                .setMessage("Vos données locales sont conservées.")
                .setPositiveButton("Se déconnecter") { _, _ ->
                    lifecycleScope.launch { viewModel.signOut() }
                }
                .setNegativeButton("Annuler", null)
                .show()
        }

        binding.btnBackup.setOnClickListener {
            MaterialAlertDialogBuilder(requireContext())
                .setTitle("💾 Sauvegarder sur Drive")
                .setMessage("Vos profils seront sauvegardés dans votre Google Drive personnel (dossier app privé).")
                .setPositiveButton("Sauvegarder") { _, _ -> viewModel.backup() }
                .setNegativeButton("Annuler", null)
                .show()
        }

        binding.btnRestore.setOnClickListener {
            MaterialAlertDialogBuilder(requireContext())
                .setTitle("⬇️ Restaurer depuis Drive")
                .setMessage("Les profils de votre sauvegarde Drive seront fusionnés avec vos données locales.")
                .setPositiveButton("Restaurer") { _, _ -> viewModel.restore() }
                .setNegativeButton("Annuler", null)
                .show()
        }
    }

    // ── Render helpers ────────────────────────────────────

    private fun renderSignedOut() {
        binding.groupSignedOut.visibility = View.VISIBLE
        binding.groupSignedIn.visibility  = View.GONE
    }

    private fun renderSignedIn(state: AuthState.SignedIn) {
        binding.groupSignedOut.visibility = View.GONE
        binding.groupSignedIn.visibility  = View.VISIBLE
        binding.textAccountEmail.text     = state.email
        binding.textAccountName.text      = state.displayName
        viewModel.refreshBackupInfo()
    }

    private fun showSnackbar(message: String) {
        Snackbar.make(binding.root, message, Snackbar.LENGTH_LONG).show()
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
