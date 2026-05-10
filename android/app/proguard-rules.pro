# ── People Modeler — ProGuard Rules ──────────────────────

# Kotlin
-keep class kotlin.** { *; }
-keep class kotlinx.** { *; }
-dontwarn kotlin.**

# AndroidX / Jetpack
-keep class androidx.** { *; }
-dontwarn androidx.**

# Room — garder les entités et DAOs
-keep class com.peoplemodeler.data.models.** { *; }
-keep class com.peoplemodeler.data.repository.** { *; }
-keepclassmembers class * {
    @androidx.room.* <fields>;
    @androidx.room.* <methods>;
}

# Gson — garder les classes sérialisées
-keep class com.google.gson.** { *; }
-keepattributes Signature
-keepattributes *Annotation*
-keep class * implements com.google.gson.TypeAdapterFactory
-keep class * implements com.google.gson.JsonSerializer
-keep class * implements com.google.gson.JsonDeserializer

# Google API Client / Drive
-keep class com.google.api.** { *; }
-keep class com.google.api.client.** { *; }
-dontwarn com.google.api.client.**
-dontwarn com.google.api.**
-keep class com.google.apis.** { *; }

# Google Auth / Identity
-keep class com.google.android.gms.** { *; }
-dontwarn com.google.android.gms.**
-keep class com.google.android.libraries.identity.** { *; }
-dontwarn com.google.android.libraries.identity.**
-keep class androidx.credentials.** { *; }
-dontwarn androidx.credentials.**

# Apache HTTP (dépendance transitive Drive SDK)
-dontwarn org.apache.http.**
-dontwarn android.net.http.**

# MPAndroidChart
-keep class com.github.mikephil.charting.** { *; }
-dontwarn com.github.mikephil.charting.**

# Navigation Component
-keep class androidx.navigation.** { *; }

# Coroutines
-keepclassmembernames class kotlinx.** {
    volatile <fields>;
}
-dontwarn kotlinx.coroutines.**

# ViewModel
-keep class * extends androidx.lifecycle.ViewModel { *; }
-keep class * extends androidx.lifecycle.AndroidViewModel { *; }

# Garder les noms des Fragments pour la navigation
-keep class com.peoplemodeler.ui.** extends androidx.fragment.app.Fragment { *; }
-keep class com.peoplemodeler.sync.SyncFragment { *; }
