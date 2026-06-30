package com.stellasecret.peoplemodeler.ui

import android.content.Context
import android.content.res.Configuration
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.navigation.NavController
import androidx.navigation.fragment.NavHostFragment
import androidx.navigation.ui.setupWithNavController
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.databinding.ActivityMainBinding
import java.util.Locale

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding
    private lateinit var navController: NavController

    override fun attachBaseContext(newBase: Context) {
        val lang =
            newBase
                .getSharedPreferences("prefs", MODE_PRIVATE)
                .getString("lang", "fr") ?: "fr"
        val locale = Locale(lang)
        Locale.setDefault(locale)
        val config = Configuration(newBase.resources.configuration)
        config.setLocale(locale)
        super.attachBaseContext(newBase.createConfigurationContext(config))
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        val navHostFragment =
            supportFragmentManager
                .findFragmentById(R.id.nav_host_fragment) as NavHostFragment
        navController = navHostFragment.navController

        binding.bottomNav.setupWithNavController(navController)

        navController.addOnDestinationChangedListener { _, destination, _ ->
            val isChild =
                destination.id == R.id.personDetailFragment ||
                    destination.id == R.id.personEditFragment
            binding.btnBack.visibility = if (isChild) android.view.View.VISIBLE else android.view.View.GONE

            binding.appTitle.text =
                when (destination.id) {
                    R.id.peopleListFragment -> "🧩 People"
                    R.id.predictionsFragment -> "🔮 Prédictions"
                    R.id.insightsFragment -> "📊 Insights"
                    else -> "People Modeler"
                }
        }

        binding.btnBack.setOnClickListener { navController.navigateUp() }

        val prefs = getSharedPreferences("prefs", MODE_PRIVATE)
        binding.btnLang.text = prefs.getString("lang", "fr")?.uppercase() ?: "FR"
        binding.btnLang.setOnClickListener {
            val current = prefs.getString("lang", "fr") ?: "fr"
            val next = if (current == "fr") "en" else "fr"
            prefs.edit().putString("lang", next).apply()
            recreate()
        }
    }

    override fun onSupportNavigateUp(): Boolean = navController.navigateUp() || super.onSupportNavigateUp()
}
