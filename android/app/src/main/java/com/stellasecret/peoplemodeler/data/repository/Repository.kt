package com.stellasecret.peoplemodeler.data.repository

import android.content.Context
import androidx.room.Dao
import androidx.room.Database
import androidx.room.Delete
import androidx.room.Entity
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.PrimaryKey
import androidx.room.Query
import androidx.room.Room
import androidx.room.RoomDatabase
import androidx.room.TypeConverters
import androidx.room.Update
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.data.models.PersonConverters
import kotlinx.coroutines.flow.Flow

// ─── DAOs ─────────────────────────────────────────────────

@Dao
interface PersonDao {
    @Query("SELECT * FROM persons ORDER BY updatedAt DESC")
    fun getAllPersons(): Flow<List<Person>>

    @Query("SELECT * FROM persons ORDER BY updatedAt DESC")
    suspend fun getAllPersonsOnce(): List<Person>

    @Query("SELECT * FROM persons WHERE id = :id")
    suspend fun getPersonById(id: String): Person?

    @Query("SELECT * FROM persons WHERE name LIKE '%' || :query || '%' OR tags LIKE '%' || :query || '%'")
    fun searchPersons(query: String): Flow<List<Person>>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertPerson(person: Person)

    @Update
    suspend fun updatePerson(person: Person)

    @Delete
    suspend fun deletePerson(person: Person)
}

@Dao
interface PredictionDao {
    @Query("SELECT * FROM predictions WHERE personId = :personId ORDER BY createdAt DESC")
    fun getPredictionsForPerson(personId: String): Flow<List<PredictionEntity>>

    @Query("SELECT * FROM predictions WHERE actualOutcome IS NULL ORDER BY createdAt DESC")
    fun getPendingPredictions(): Flow<List<PredictionEntity>>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insertPrediction(prediction: PredictionEntity)

    @Delete
    suspend fun deletePrediction(prediction: PredictionEntity)

    @Query("SELECT AVG(accuracy) FROM predictions WHERE personId = :personId AND accuracy IS NOT NULL")
    suspend fun averageAccuracyForPerson(personId: String): Double?
}

// ─── Room Entity for Prediction ───────────────────────────

@Entity(tableName = "predictions")
data class PredictionEntity(
    @PrimaryKey val id: String,
    val personId: String,
    val context: String,
    val predictedOutcome: String,
    val actualOutcome: String? = null,
    val accuracy: Int? = null,
    val createdAt: Long = System.currentTimeMillis(),
    val resolvedAt: Long? = null,
)

// ─── Database ─────────────────────────────────────────────

@Database(
    entities = [Person::class, PredictionEntity::class],
    version = 1,
    exportSchema = false,
)
@TypeConverters(PersonConverters::class)
abstract class AppDatabase : RoomDatabase() {
    abstract fun personDao(): PersonDao

    abstract fun predictionDao(): PredictionDao

    companion object {
        @Volatile private var instance: AppDatabase? = null

        fun getInstance(context: Context): AppDatabase =
            instance ?: synchronized(this) {
                Room
                    .databaseBuilder(
                        context.applicationContext,
                        AppDatabase::class.java,
                        "people_modeler.db",
                    ).build()
                    .also { instance = it }
            }
    }
}

// ─── Repository ───────────────────────────────────────────

class PersonRepository(
    private val db: AppDatabase,
) {
    val allPersons = db.personDao().getAllPersons()

    suspend fun getAllPersonsOnce() = db.personDao().getAllPersonsOnce()

    fun searchPersons(query: String) = db.personDao().searchPersons(query)

    suspend fun getPersonById(id: String) = db.personDao().getPersonById(id)

    suspend fun savePerson(person: Person) {
        db.personDao().insertPerson(person.copy(updatedAt = System.currentTimeMillis()))
    }

    suspend fun deletePerson(person: Person) = db.personDao().deletePerson(person)

    fun getPredictionsForPerson(personId: String) = db.predictionDao().getPredictionsForPerson(personId)

    fun getPendingPredictions() = db.predictionDao().getPendingPredictions()

    suspend fun savePrediction(prediction: PredictionEntity) = db.predictionDao().insertPrediction(prediction)

    suspend fun getAverageAccuracy(personId: String) = db.predictionDao().averageAccuracyForPerson(personId)
}
