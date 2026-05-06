package com.peoplemodeler.viewmodels

import android.app.Application
import androidx.lifecycle.*
import com.peoplemodeler.data.models.*
import com.peoplemodeler.data.repository.AppDatabase
import com.peoplemodeler.data.repository.PersonRepository
import com.peoplemodeler.data.repository.PredictionEntity
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.flatMapLatest
import kotlinx.coroutines.launch

@OptIn(ExperimentalCoroutinesApi::class)
class PersonViewModel(application: Application) : AndroidViewModel(application) {

    private val repo = PersonRepository(AppDatabase.getInstance(application))

    val allPersons = repo.allPersons.asLiveData()

    private val _searchQuery = MutableStateFlow("")
    val searchResults = _searchQuery.flatMapLatest { query ->
        if (query.isBlank()) repo.allPersons
        else repo.searchPersons(query)
    }.asLiveData()

    fun search(query: String) { _searchQuery.value = query }

    private val _currentPerson = MutableLiveData<Person?>()
    val currentPerson: LiveData<Person?> = _currentPerson

    fun loadPerson(id: String) = viewModelScope.launch {
        _currentPerson.value = repo.getPersonById(id)
    }

    fun savePerson(person: Person) = viewModelScope.launch {
        repo.savePerson(person)
        _currentPerson.value = person
    }

    fun deletePerson(person: Person) = viewModelScope.launch {
        repo.deletePerson(person)
        if (_currentPerson.value?.id == person.id) _currentPerson.value = null
    }

    fun getPredictions(personId: String) =
        repo.getPredictionsForPerson(personId).asLiveData()

    val pendingPredictions = repo.getPendingPredictions().asLiveData()

    fun addPrediction(personId: String, context: String, predicted: String) =
        viewModelScope.launch {
            repo.savePrediction(
                PredictionEntity(
                    id = java.util.UUID.randomUUID().toString(),
                    personId = personId,
                    context = context,
                    predictedOutcome = predicted
                )
            )
        }

    fun resolvePrediction(prediction: PredictionEntity, actual: String, accuracy: Int) =
        viewModelScope.launch {
            repo.savePrediction(
                prediction.copy(
                    actualOutcome = actual,
                    accuracy = accuracy,
                    resolvedAt = System.currentTimeMillis()
                )
            )
        }

    fun generateBehavioralInsight(person: Person, trigger: BehaviorTrigger): String {
        val topMotivation = person.motivations.maxByOrNull { it.intensity }
        val topBias = person.biases.maxByOrNull { it.intensity }
        return buildString {
            append("Sous '${trigger.label}', ${person.name} est susceptible de :\n\n")
            topMotivation?.let { append("• Chercher à satisfaire : ${it.type.label} ${it.type.emoji}\n") }
            topBias?.let { append("• Être influencé par : ${it.type.label} ${it.type.emoji}\n") }
            if (person.neuroticism > 7) append("• Réagir de façon émotionnelle\n")
            if (person.conscientiousness > 7) append("• Chercher à contrôler et planifier\n")
            if (person.agreeableness > 7) append("• Éviter le conflit, rechercher l'harmonie\n")
            if (person.extraversion > 7) append("• Exprimer verbalement ses préoccupations\n")
            person.behavioralPatterns
                .find { it.trigger == trigger }
                ?.let { append("\n📌 Comportement observé : ${it.predictedBehavior}") }
        }
    }
}
