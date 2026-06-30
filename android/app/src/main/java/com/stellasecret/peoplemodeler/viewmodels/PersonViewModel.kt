package com.stellasecret.peoplemodeler.viewmodels

import android.app.Application
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.asLiveData
import androidx.lifecycle.viewModelScope
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.data.models.BehaviorTrigger
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.data.repository.AppDatabase
import com.stellasecret.peoplemodeler.data.repository.PersonRepository
import com.stellasecret.peoplemodeler.data.repository.PredictionEntity
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.launch

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
        repo.savePrediction(
            PredictionEntity(
                id =
                    java.util.UUID
                        .randomUUID()
                        .toString(),
                personId = personId,
                context = context,
                predictedOutcome = predicted,
            ),
        )
    }

    fun resolvePrediction(
        prediction: PredictionEntity,
        actual: String,
        accuracy: Int,
    ) = viewModelScope.launch {
        repo.savePrediction(
            prediction.copy(
                actualOutcome = actual,
                accuracy = accuracy,
                resolvedAt = System.currentTimeMillis(),
            ),
        )
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
