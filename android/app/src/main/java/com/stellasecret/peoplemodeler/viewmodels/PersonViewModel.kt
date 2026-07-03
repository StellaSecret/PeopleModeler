package com.stellasecret.peoplemodeler.viewmodels

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.asLiveData
import androidx.lifecycle.viewModelScope
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.core.PeopleModeler
import com.stellasecret.peoplemodeler.data.models.BehaviorTrigger
import com.stellasecret.peoplemodeler.data.models.BiasType
import com.stellasecret.peoplemodeler.data.models.MotivationType
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.data.repository.AppDatabase
import com.stellasecret.peoplemodeler.data.repository.PersonRepository
import com.stellasecret.peoplemodeler.data.repository.PredictionEntity
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.launch
import org.json.JSONObject

@OptIn(ExperimentalCoroutinesApi::class)
class PersonViewModel(
    application: Application,
) : AndroidViewModel(application) {
    private val repo = PersonRepository(AppDatabase.getInstance(application))

    val allPersons = repo.allPersons.asLiveData()

    private val searchQuery = MutableStateFlow("")
    val searchResults =
        searchQuery
            .flatMapLatest { query ->
                if (query.isBlank()) {
                    repo.allPersons
                } else {
                    repo.searchPersons(query)
                }
            }.asLiveData()

    fun search(query: String) {
        searchQuery.value = query
    }

    private val _currentPerson = MutableLiveData<Person?>()
    val currentPerson: LiveData<Person?> = _currentPerson

    fun loadPerson(id: String) =
        viewModelScope.launch {
            _currentPerson.value = repo.getPersonById(id)
        }

    fun savePerson(person: Person) =
        viewModelScope.launch {
            repo.savePerson(person)
            _currentPerson.value = person
        }

    fun deletePerson(person: Person) =
        viewModelScope.launch {
            repo.deletePerson(person)
            if (_currentPerson.value?.id == person.id) _currentPerson.value = null
        }

    fun getPredictions(personId: String) = repo.getPredictionsForPerson(personId).asLiveData()

    val pendingPredictions = repo.getPendingPredictions().asLiveData()

    fun deletePrediction(prediction: PredictionEntity) =
        viewModelScope.launch {
            repo.deletePrediction(prediction)
        }

    fun addPrediction(
        personId: String,
        context: String,
        predicted: String,
    ) = viewModelScope.launch {
        val entity =
            if (PeopleModeler.isAvailable) {
                try {
                    val json = PeopleModeler.createPrediction(personId, context, predicted)
                    val obj = JSONObject(json)
                    PredictionEntity(
                        id = obj.getString("id"),
                        personId = obj.getString("person_id"),
                        context = obj.getString("context"),
                        predictedOutcome = obj.getString("predicted_outcome"),
                        createdAt = obj.optLong("created_at", System.currentTimeMillis()),
                    )
                } catch (_: Exception) {
                    fallbackPrediction(personId, context, predicted)
                }
            } else {
                fallbackPrediction(personId, context, predicted)
            }
        repo.savePrediction(entity)
    }

    private fun fallbackPrediction(
        personId: String,
        context: String,
        predicted: String,
    ) = PredictionEntity(
        id =
            java.util.UUID
                .randomUUID()
                .toString(),
        personId = personId,
        context = context,
        predictedOutcome = predicted,
    )

    fun resolvePrediction(
        prediction: PredictionEntity,
        actual: String,
        accuracy: Int,
    ) = viewModelScope.launch {
        val entity =
            if (PeopleModeler.isAvailable) {
                try {
                    val input =
                        JSONObject()
                            .apply {
                                put("id", prediction.id)
                                put("person_id", prediction.personId)
                                put("context", prediction.context)
                                put("predicted_outcome", prediction.predictedOutcome)
                                put("actual_outcome", JSONObject.NULL)
                                put("accuracy", JSONObject.NULL)
                                put("created_at", prediction.createdAt)
                                put("resolved_at", JSONObject.NULL)
                                put("resolved", false)
                            }.toString()
                    val json = PeopleModeler.resolvePrediction(input, actual, accuracy)
                    val obj = JSONObject(json)
                    prediction.copy(
                        actualOutcome = obj.optString("actual_outcome", actual),
                        accuracy = obj.optInt("accuracy", accuracy),
                        resolvedAt = obj.optLong("resolved_at", System.currentTimeMillis()),
                    )
                } catch (_: Exception) {
                    fallbackResolve(prediction, actual, accuracy)
                }
            } else {
                fallbackResolve(prediction, actual, accuracy)
            }
        repo.savePrediction(entity)
    }

    private fun fallbackResolve(
        prediction: PredictionEntity,
        actual: String,
        accuracy: Int,
    ) = prediction.copy(
        actualOutcome = actual,
        accuracy = accuracy,
        resolvedAt = System.currentTimeMillis(),
    )

    fun generateBehavioralInsight(
        person: Person,
        trigger: BehaviorTrigger,
    ): String {
        val ctx = getApplication<Application>()
        val topMot = person.motivations.maxByOrNull { it.intensity }
        val topBias = person.biases.maxByOrNull { it.intensity }
        return buildString {
            append(ctx.getString(R.string.insight_header_format, ctx.getString(trigger.labelResId), person.name))
            topMot?.let { append(ctx.getString(R.string.insight_motivation_line, ctx.getString(it.type.labelResId), it.type.emoji)) }
            topBias?.let { append(ctx.getString(R.string.insight_bias_line, ctx.getString(it.type.labelResId), it.type.emoji)) }
            when (trigger) {
                BehaviorTrigger.STRESS -> {
                    if (person.neuroticism >= 7) append(ctx.getString(R.string.insight_stress_bullet_n_high))
                    if (person.extraversion >= 7) append(ctx.getString(R.string.insight_stress_bullet_e_high))
                    if (person.extraversion <= 4) append(ctx.getString(R.string.insight_stress_bullet_e_low))
                    if (person.conscientiousness >= 7) append(ctx.getString(R.string.insight_stress_bullet_c_high))
                    if (topMot?.type == MotivationType.POWER) append(ctx.getString(R.string.insight_stress_bullet_power))
                    if (topMot?.type == MotivationType.SECURITY) append(ctx.getString(R.string.insight_stress_bullet_security))
                    if (topBias !=
                        null
                    ) {
                        append(ctx.getString(R.string.insight_stress_bullet_top_bias, ctx.getString(topBias.type.labelResId)))
                    }
                    val strategy =
                        when {
                            person.agreeableness >= 7 -> ctx.getString(R.string.insight_stress_strategy_high_a)
                            person.agreeableness <= 3 -> ctx.getString(R.string.insight_stress_strategy_low_a)
                            else -> ctx.getString(R.string.insight_stress_strategy_mid_a)
                        }
                    append(ctx.getString(R.string.insight_strategy_suffix, strategy))
                }

                BehaviorTrigger.CONFLICT -> {
                    if (person.agreeableness <= 4) append(ctx.getString(R.string.insight_conflict_bullet_a_low))
                    if (person.agreeableness >= 7) append(ctx.getString(R.string.insight_conflict_bullet_a_high))
                    if (person.neuroticism >= 7) append(ctx.getString(R.string.insight_conflict_bullet_n_high))
                    if (person.extraversion >= 7) append(ctx.getString(R.string.insight_conflict_bullet_e_high))
                    if (topMot?.type == MotivationType.POWER) append(ctx.getString(R.string.insight_conflict_bullet_power))
                    if (topMot?.type == MotivationType.AFFILIATION) append(ctx.getString(R.string.insight_conflict_bullet_affiliation))
                    val lossBias = person.biases.find { it.type == BiasType.LOSS_AVERSION }
                    if (lossBias?.intensity != null &&
                        lossBias.intensity >= 6
                    ) {
                        append(ctx.getString(R.string.insight_conflict_bullet_loss_aversion))
                    }
                    val strategy =
                        when {
                            person.agreeableness >= 7 -> ctx.getString(R.string.insight_conflict_strategy_high_a)
                            person.agreeableness <= 3 -> ctx.getString(R.string.insight_conflict_strategy_low_a)
                            else -> ctx.getString(R.string.insight_conflict_strategy_mid_a)
                        }
                    append(ctx.getString(R.string.insight_strategy_suffix, strategy))
                }

                BehaviorTrigger.SUCCESS -> {
                    val recMot = person.motivations.find { it.type == MotivationType.RECOGNITION }
                    if (recMot?.intensity != null &&
                        recMot.intensity >= 7
                    ) {
                        append(ctx.getString(R.string.insight_success_bullet_recognition_high))
                    }
                    val powMot = person.motivations.find { it.type == MotivationType.POWER }
                    if (powMot?.intensity != null &&
                        powMot.intensity >= 7
                    ) {
                        append(ctx.getString(R.string.insight_success_bullet_power_high))
                    }
                    if (person.openness >= 7) append(ctx.getString(R.string.insight_success_bullet_o_high))
                    if (person.conscientiousness >= 7) append(ctx.getString(R.string.insight_success_bullet_c_high))
                    val dkBias = person.biases.find { it.type == BiasType.DUNNING_KRUGER }
                    if (dkBias?.intensity != null && dkBias.intensity >= 6) append(ctx.getString(R.string.insight_success_bullet_dk))
                    val successStrat =
                        when {
                            person.conscientiousness >= 7 -> ctx.getString(R.string.insight_success_strategy_high_c)
                            person.conscientiousness <= 3 -> ctx.getString(R.string.insight_success_strategy_low_c)
                            else -> ctx.getString(R.string.insight_success_strategy_mid_c)
                        }
                    append(ctx.getString(R.string.insight_strategy_suffix, successStrat))
                }

                BehaviorTrigger.UNCERTAINTY -> {
                    if (person.neuroticism >= 7) append(ctx.getString(R.string.insight_uncertainty_bullet_n_high))
                    if (person.neuroticism <= 3) append(ctx.getString(R.string.insight_uncertainty_bullet_n_low))
                    if (person.openness >= 7) append(ctx.getString(R.string.insight_uncertainty_bullet_o_high))
                    if (person.openness <= 4) append(ctx.getString(R.string.insight_uncertainty_bullet_o_low))
                    val secMot = person.motivations.find { it.type == MotivationType.SECURITY }
                    if (secMot?.intensity != null &&
                        secMot.intensity >= 7
                    ) {
                        append(ctx.getString(R.string.insight_uncertainty_bullet_security_high))
                    }
                    val ancBias = person.biases.find { it.type == BiasType.ANCHORING }
                    if (ancBias?.intensity != null &&
                        ancBias.intensity >= 6
                    ) {
                        append(ctx.getString(R.string.insight_uncertainty_bullet_anchoring))
                    }
                    val uncertStrat =
                        when {
                            person.neuroticism >= 7 -> ctx.getString(R.string.insight_uncertainty_strategy_high_n)
                            person.neuroticism <= 3 -> ctx.getString(R.string.insight_uncertainty_strategy_low_n)
                            else -> ctx.getString(R.string.insight_uncertainty_strategy_mid_n)
                        }
                    append(ctx.getString(R.string.insight_strategy_suffix, uncertStrat))
                }

                BehaviorTrigger.RECOGNITION -> {
                    val recMot = person.motivations.find { it.type == MotivationType.RECOGNITION }
                    if (recMot != null) {
                        when {
                            recMot.intensity >= 8 -> append(ctx.getString(R.string.insight_recognition_bullet_intensity_high))
                            recMot.intensity >= 5 -> append(ctx.getString(R.string.insight_recognition_bullet_intensity_mid))
                            else -> append(ctx.getString(R.string.insight_recognition_bullet_intensity_low))
                        }
                    }
                    if (person.extraversion >= 7) append(ctx.getString(R.string.insight_recognition_bullet_e_high))
                    if (person.extraversion <= 4) append(ctx.getString(R.string.insight_recognition_bullet_e_low))
                    val spBias = person.biases.find { it.type == BiasType.SOCIAL_PROOF }
                    if (spBias?.intensity != null &&
                        spBias.intensity >= 6
                    ) {
                        append(ctx.getString(R.string.insight_recognition_bullet_social_proof))
                    }
                    val recStrat =
                        when {
                            person.openness >= 7 -> ctx.getString(R.string.insight_recognition_strategy_high_o)
                            person.openness <= 3 -> ctx.getString(R.string.insight_recognition_strategy_low_o)
                            else -> ctx.getString(R.string.insight_recognition_strategy_mid_o)
                        }
                    append(ctx.getString(R.string.insight_strategy_suffix, recStrat))
                }

                BehaviorTrigger.THREATENED -> {
                    val powMot = person.motivations.find { it.type == MotivationType.POWER }
                    if (powMot?.intensity != null && powMot.intensity >= 7) append(ctx.getString(R.string.insight_threat_bullet_power_high))
                    if (person.agreeableness <= 4) append(ctx.getString(R.string.insight_threat_bullet_a_low))
                    if (person.agreeableness >= 7) append(ctx.getString(R.string.insight_threat_bullet_a_high))
                    if (person.neuroticism >= 7) append(ctx.getString(R.string.insight_threat_bullet_n_high))
                    val laBias = person.biases.find { it.type == BiasType.LOSS_AVERSION }
                    if (laBias?.intensity != null &&
                        laBias.intensity >= 6
                    ) {
                        append(ctx.getString(R.string.insight_threat_bullet_loss_aversion))
                    }
                    val confBias = person.biases.find { it.type == BiasType.CONFIRMATION }
                    if (confBias?.intensity != null &&
                        confBias.intensity >= 6
                    ) {
                        append(ctx.getString(R.string.insight_threat_bullet_confirmation))
                    }
                    val threatStrat =
                        when {
                            person.extraversion >= 7 -> ctx.getString(R.string.insight_threat_strategy_high_e)
                            person.extraversion <= 3 -> ctx.getString(R.string.insight_threat_strategy_low_e)
                            else -> ctx.getString(R.string.insight_threat_strategy_mid_e)
                        }
                    append(ctx.getString(R.string.insight_strategy_suffix, threatStrat))
                }
            }
            person.behavioralPatterns
                .find { it.trigger == trigger }
                ?.let { append(ctx.getString(R.string.insight_observed_pattern, it.predictedBehavior)) }
        }
    }
}
