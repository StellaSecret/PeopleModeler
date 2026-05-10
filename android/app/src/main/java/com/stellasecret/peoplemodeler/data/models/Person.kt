package com.stellasecret.peoplemodeler.data.models

import androidx.room.Entity
import androidx.room.PrimaryKey
import androidx.room.TypeConverter
import androidx.room.TypeConverters
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import java.util.UUID

// ─── Enums ───────────────────────────────────────────────

enum class MotivationType(val label: String, val emoji: String) {
    POWER("Pouvoir", "👑"),
    ACHIEVEMENT("Accomplissement", "🏆"),
    AFFILIATION("Appartenance", "🤝"),
    SECURITY("Sécurité", "🛡️"),
    AUTONOMY("Autonomie", "🦅"),
    RECOGNITION("Reconnaissance", "⭐"),
    LEARNING("Apprentissage", "📚"),
    HELPING("Aider les autres", "❤️")
}

enum class BiasType(val label: String, val emoji: String) {
    CONFIRMATION("Confirmation", "🔄"),
    ANCHORING("Ancrage", "⚓"),
    AVAILABILITY("Disponibilité", "📱"),
    SUNK_COST("Coût irrécupérable", "💸"),
    DUNNING_KRUGER("Dunning-Kruger", "🎭"),
    LOSS_AVERSION("Aversion aux pertes", "😰"),
    SOCIAL_PROOF("Preuve sociale", "👥"),
    AUTHORITY("Autorité", "🎖️"),
    RECENCY("Récence", "⏰"),
    IN_GROUP("Endogroupe", "🏠")
}

enum class BehaviorTrigger(val label: String) {
    STRESS("Sous stress"),
    CONFLICT("En conflit"),
    SUCCESS("En réussite"),
    UNCERTAINTY("Dans l'incertitude"),
    RECOGNITION("Cherchant reconnaissance"),
    THREATENED("Se sentant menacé")
}

// ─── Core Models ─────────────────────────────────────────

data class Motivation(
    val type: MotivationType,
    val intensity: Int, // 1–10
    val notes: String = ""
)

data class Bias(
    val type: BiasType,
    val intensity: Int, // 1–10
    val evidence: String = ""
)

data class BehavioralPattern(
    val trigger: BehaviorTrigger,
    val predictedBehavior: String,
    val confidence: Int // 1–10
)

data class Prediction(
    val id: String = UUID.randomUUID().toString(),
    val personId: String,
    val context: String,
    val predictedOutcome: String,
    val actualOutcome: String? = null,
    val accuracy: Int? = null, // 1–10 après feedback
    val createdAt: Long = System.currentTimeMillis(),
    val resolvedAt: Long? = null
)

// ─── Main Entity ──────────────────────────────────────────

@Entity(tableName = "persons")
@TypeConverters(PersonConverters::class)
data class Person(
    @PrimaryKey val id: String = UUID.randomUUID().toString(),
    val name: String,
    val role: String = "",
    val context: String = "", // Ex: collègue, client, partenaire
    val avatarEmoji: String = "🧑",

    // Psycho profile
    val motivations: List<Motivation> = emptyList(),
    val biases: List<Bias> = emptyList(),
    val behavioralPatterns: List<BehavioralPattern> = emptyList(),

    // Big Five (OCEAN) 1–10
    val openness: Int = 5,
    val conscientiousness: Int = 5,
    val extraversion: Int = 5,
    val agreeableness: Int = 5,
    val neuroticism: Int = 5,

    // Meta
    val tags: List<String> = emptyList(),
    val notes: String = "",
    val createdAt: Long = System.currentTimeMillis(),
    val updatedAt: Long = System.currentTimeMillis()
) {
    val predictionAccuracy: Double
        get() = 0.0 // Calculé depuis les prédictions associées

    val topMotivation: MotivationType?
        get() = motivations.maxByOrNull { it.intensity }?.type

    val topBias: BiasType?
        get() = biases.maxByOrNull { it.intensity }?.type
}

// ─── Room Type Converters ─────────────────────────────────

class PersonConverters {
    private val gson = Gson()

    @TypeConverter fun motivationsToJson(v: List<Motivation>) = gson.toJson(v)
    @TypeConverter fun jsonToMotivations(v: String): List<Motivation> =
        gson.fromJson(v, object : TypeToken<List<Motivation>>() {}.type) ?: emptyList()

    @TypeConverter fun biasesToJson(v: List<Bias>) = gson.toJson(v)
    @TypeConverter fun jsonToBiases(v: String): List<Bias> =
        gson.fromJson(v, object : TypeToken<List<Bias>>() {}.type) ?: emptyList()

    @TypeConverter fun patternsToJson(v: List<BehavioralPattern>) = gson.toJson(v)
    @TypeConverter fun jsonToPatterns(v: String): List<BehavioralPattern> =
        gson.fromJson(v, object : TypeToken<List<BehavioralPattern>>() {}.type) ?: emptyList()

    @TypeConverter fun tagsToJson(v: List<String>) = gson.toJson(v)
    @TypeConverter fun jsonToTags(v: String): List<String> =
        gson.fromJson(v, object : TypeToken<List<String>>() {}.type) ?: emptyList()
}
