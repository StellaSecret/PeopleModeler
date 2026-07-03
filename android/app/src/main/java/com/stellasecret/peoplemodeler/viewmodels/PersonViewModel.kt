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

    fun addPrediction(
        personId: String,
        context: String,
        predicted: String,
    ) = viewModelScope.launch {
        val entity =
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
                PredictionEntity(
                    id =
                        java.util.UUID
                            .randomUUID()
                            .toString(),
                    personId = personId,
                    context = context,
                    predictedOutcome = predicted,
                )
            }
        repo.savePrediction(entity)
    }

    fun resolvePrediction(
        prediction: PredictionEntity,
        actual: String,
        accuracy: Int,
    ) = viewModelScope.launch {
        val entity =
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
                prediction.copy(
                    actualOutcome = actual,
                    accuracy = accuracy,
                    resolvedAt = System.currentTimeMillis(),
                )
            }
        repo.savePrediction(entity)
    }

    fun generateBehavioralInsight(
        person: Person,
        trigger: BehaviorTrigger,
    ): String {
        val ctx = getApplication<Application>()
        val topMotivation = person.motivations.maxByOrNull { it.intensity }
        val topBias = person.biases.maxByOrNull { it.intensity }
        return buildString {
            append(ctx.getString(R.string.insight_header_format, ctx.getString(trigger.labelResId), person.name))
            topMotivation?.let { append(ctx.getString(R.string.insight_motivation_line, ctx.getString(it.type.labelResId), it.type.emoji)) }
            topBias?.let { append(ctx.getString(R.string.insight_bias_line, ctx.getString(it.type.labelResId), it.type.emoji)) }
            if (person.neuroticism > 7) append(ctx.getString(R.string.insight_neuroticism_line))
            if (person.conscientiousness > 7) append(ctx.getString(R.string.insight_conscientiousness_line))
            if (person.agreeableness > 7) append(ctx.getString(R.string.insight_agreeableness_line))
            if (person.extraversion > 7) append(ctx.getString(R.string.insight_extraversion_line))
            person.behavioralPatterns
                .find { it.trigger == trigger }
                ?.let { append(ctx.getString(R.string.insight_observed_pattern, it.predictedBehavior)) }
        }
    }
}
