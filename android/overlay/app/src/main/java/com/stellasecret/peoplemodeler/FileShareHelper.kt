package com.stellasecret.peoplemodeler

import android.app.Activity
import android.content.Intent
import android.net.Uri
import android.util.Log
import androidx.activity.result.ActivityResultLauncher
import java.io.File

private const val TAG = "FileShareHelper"

object FileShareHelper {
    private var activity: Activity? = null
    private var importLauncher: ActivityResultLauncher<Intent>? = null

    fun init(
        act: Activity,
        launcher: ActivityResultLauncher<Intent>,
    ) {
        activity = act
        importLauncher = launcher
        Log.d(TAG, "init: calling nativeInit")
        nativeInit()
    }

    private external fun nativeInit()

    fun launchExport(json: String) {
        Log.d(TAG, "launchExport: sharing JSON backup")
        val intent =
            Intent(Intent.ACTION_SEND).apply {
                type = "text/plain"
                putExtra(Intent.EXTRA_TEXT, json)
                putExtra(Intent.EXTRA_SUBJECT, "PeopleModeler Backup")
            }
        activity?.startActivity(Intent.createChooser(intent, "Export Backup"))
    }

    fun launchImport() {
        Log.d(TAG, "launchImport: opening file picker")
        val intent =
            Intent(Intent.ACTION_OPEN_DOCUMENT).apply {
                addCategory(Intent.CATEGORY_OPENABLE)
                type = "application/json"
            }
        importLauncher?.launch(intent)
    }

    fun handleImportResult(data: Intent?) {
        val uri: Uri? = data?.data
        if (uri == null) {
            Log.e(TAG, "handleImportResult: no data URI")
            return
        }
        Log.d(TAG, "handleImportResult: uri=$uri")
        try {
            val ctx =
                activity ?: run {
                    Log.e(TAG, "handleImportResult: activity is null")
                    return
                }
            val input = ctx.contentResolver.openInputStream(uri)
            val text = input?.bufferedReader()?.readText() ?: ""
            input?.close()
            Log.d(TAG, "handleImportResult: read ${text.length} chars")
            val f = File(ctx.filesDir, ".pm_import_data")
            f.parentFile?.mkdirs()
            f.writeText(text)
            nativeOnImportReady()
        } catch (e: Exception) {
            Log.e(TAG, "handleImportResult: ${e.message}")
        }
    }

    private external fun nativeOnImportReady()
}
