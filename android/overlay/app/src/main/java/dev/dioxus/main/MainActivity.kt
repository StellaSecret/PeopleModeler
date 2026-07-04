package dev.dioxus.main

import android.content.Intent
import android.os.Bundle
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import com.stellasecret.peoplemodeler.GoogleDriveHelper

typealias BuildConfig = com.stellasecret.peoplemodeler.BuildConfig

class MainActivity : WryActivity() {
    private lateinit var signInLauncher: ActivityResultLauncher<Intent>

    override fun onCreate(savedInstanceState: Bundle?) {
        signInLauncher =
            registerForActivityResult(
                ActivityResultContracts.StartActivityForResult(),
            ) { result ->
                GoogleDriveHelper.handleSignInResult(result.data)
            }
        super.onCreate(savedInstanceState)
        GoogleDriveHelper.init(this, signInLauncher)
    }
}
