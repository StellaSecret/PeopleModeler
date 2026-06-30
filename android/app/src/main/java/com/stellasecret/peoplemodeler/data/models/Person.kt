package com.stellasecret.peoplemodeler.data.models

import androidx.room.Entity
import androidx.room.PrimaryKey
import androidx.room.TypeConverter
import androidx.room.TypeConverters
import com.google.gson.Gson
import com.google.gson.reflect.TypeToken
import com.stellasecret.peoplemodeler.R
import java.util.UUID

// ─── Enums ───────────────────────────────────────────────

enum class MotivationType(
    val labelResId: Int,
    val emoji: String,
    val descResId: Int,
) {
    POWER(R.string.mot_power, "👑", R.string.mot_power_desc),
    ACHIEVEMENT(R.string.mot_achievement, "🏆", R.string.mot_achievement_desc),
    AFFILIATION(R.string.mot_affiliation, "🤝", R.string.mot_affiliation_desc),
    SECURITY(R.string.mot_security, "🛡️", R.string.mot_security_desc),
    AUTONOMY(R.string.mot_autonomy, "🦅", R.string.mot_autonomy_desc),
    RECOGNITION(R.string.mot_recognition, "⭐", R.string.mot_recognition_desc),
    LEARNING(R.string.mot_learning, "📚", R.string.mot_learning_desc),
    HELPING(R.string.mot_helping, "❤️", R.string.mot_helping_desc),
}

enum class BiasType(
    val labelResId: Int,
    val emoji: String,
    val descResId: Int,
) {
    CONFIRMATION(R.string.bias_confirmation, "🔄", R.string.bias_confirmation_desc),
    ANCHORING(R.string.bias_anchoring, "⚓", R.string.bias_anchoring_desc),
    AVAILABILITY(R.string.bias_availability, "📱", R.string.bias_availability_desc),
    SUNK_COST(R.string.bias_sunk_cost, "💸", R.string.bias_sunk_cost_desc),
    DUNNING_KRUGER(R.string.bias_dunning_kruger, "🎭", R.string.bias_dunning_kruger_desc),
    LOSS_AVERSION(R.string.bias_loss_aversion, "😰", R.string.bias_loss_aversion_desc),
    SOCIAL_PROOF(R.string.bias_social_proof, "👥", R.string.bias_social_proof_desc),
    AUTHORITY(R.string.bias_authority, "🎖️", R.string.bias_authority_desc),
    RECENCY(R.string.bias_recency, "⏰", R.string.bias_recency_desc),
    IN_GROUP(R.string.bias_in_group, "🏠", R.string.bias_in_group_desc),
}

enum class BehaviorTrigger(
    val labelResId: Int,
) {
    STRESS(R.string.trigger_stress),
    CONFLICT(R.string.trigger_conflict),
    SUCCESS(R.string.trigger_success),
    UNCERTAINTY(R.string.trigger_uncertainty),
    RECOGNITION(R.string.trigger_recognition),
    THREATENED(R.string.trigger_threatened),
}

// ─── Core Models ─────────────────────────────────────────

data class Motivation(
    val type: MotivationType,
    val intensity: Int, // 1–10
    val notes: String = "",
)

data class Bias(
    val type: BiasType,
    val intensity: Int, // 1–10
    val evidence: String = "",
)

data class BehavioralPattern(
    val trigger: BehaviorTrigger,
    val predictedBehavior: String,
    val confidence: Int, // 1–10
)

data class Prediction(
    val id: String = UUID.randomUUID().toString(),
    val personId: String,
    val context: String,
    val predictedOutcome: String,
    val actualOutcome: String? = null,
    val accuracy: Int? = null, // 1–10 après feedback
    val createdAt: Long = System.currentTimeMillis(),
    val resolvedAt: Long? = null,
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
    val updatedAt: Long = System.currentTimeMillis(),
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
        gson.fromJson(
            v,
            object : TypeToken<List<Motivation>>() {}.type,
        ) ?: emptyList()

    @TypeConverter fun biasesToJson(v: List<Bias>) = gson.toJson(v)

    @TypeConverter fun jsonToBiases(v: String): List<Bias> = gson.fromJson(v, object : TypeToken<List<Bias>>() {}.type) ?: emptyList()

    @TypeConverter fun patternsToJson(v: List<BehavioralPattern>) = gson.toJson(v)

    @TypeConverter fun jsonToPatterns(v: String): List<BehavioralPattern> =
        gson.fromJson(v, object : TypeToken<List<BehavioralPattern>>() {}.type) ?: emptyList()

    @TypeConverter fun tagsToJson(v: List<String>) = gson.toJson(v)

    @TypeConverter fun jsonToTags(v: String): List<String> = gson.fromJson(v, object : TypeToken<List<String>>() {}.type) ?: emptyList()
}
