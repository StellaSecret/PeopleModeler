package com.stellasecret.peoplemodeler

import com.google.gson.Gson
import com.stellasecret.peoplemodeler.data.models.Bias
import com.stellasecret.peoplemodeler.data.models.BiasType
import com.stellasecret.peoplemodeler.data.models.Motivation
import com.stellasecret.peoplemodeler.data.models.MotivationType
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.sync.DriveSync
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test

class DriveSyncTest {
    private val gson = Gson()

    private val samplePerson =
        Person(
            id = "p1",
            name = "Test User",
            role = "Dev",
            context = "Work",
            avatarEmoji = "🧑",
            motivations =
                listOf(
                    Motivation(MotivationType.POWER, 8, "Veut diriger"),
                    Motivation(MotivationType.LEARNING, 6),
                ),
            biases =
                listOf(
                    Bias(BiasType.CONFIRMATION, 7, "Ignore les avis contraires"),
                    Bias(BiasType.ANCHORING, 5),
                ),
            openness = 7,
            conscientiousness = 8,
            extraversion = 6,
            agreeableness = 5,
            neuroticism = 4,
            tags = listOf("tech", "manager"),
            notes = "Some notes",
        )

    @Test
    fun `BackupPayload se sérialise et se désérialise avec Gson`() {
        val payload = DriveSync.BackupPayload(1, 1000L, listOf(samplePerson))
        val json = gson.toJson(payload)
        val restored: DriveSync.BackupPayload = gson.fromJson(json, DriveSync.BackupPayload::class.java)

        assertEquals(1, restored.version)
        assertEquals(1000L, restored.timestamp)
        assertEquals(1, restored.persons.size)
    }

    @Test
    fun `BackupPayload préserve les champs Person`() {
        val payload = DriveSync.BackupPayload(1, 1000L, listOf(samplePerson))
        val json = gson.toJson(payload)
        val restored: DriveSync.BackupPayload = gson.fromJson(json, DriveSync.BackupPayload::class.java)
        val p = restored.persons.first()

        assertEquals(samplePerson.id, p.id)
        assertEquals(samplePerson.name, p.name)
        assertEquals(samplePerson.role, p.role)
        assertEquals(samplePerson.context, p.context)
        assertEquals(samplePerson.avatarEmoji, p.avatarEmoji)
        assertEquals(samplePerson.openness, p.openness)
        assertEquals(samplePerson.conscientiousness, p.conscientiousness)
        assertEquals(samplePerson.extraversion, p.extraversion)
        assertEquals(samplePerson.agreeableness, p.agreeableness)
        assertEquals(samplePerson.neuroticism, p.neuroticism)
        assertEquals(samplePerson.tags, p.tags)
        assertEquals(samplePerson.notes, p.notes)
    }

    @Test
    fun `BackupPayload préserve les motivations avec notes`() {
        val payload = DriveSync.BackupPayload(1, 1000L, listOf(samplePerson))
        val json = gson.toJson(payload)
        val restored: DriveSync.BackupPayload = gson.fromJson(json, DriveSync.BackupPayload::class.java)
        val motivations = restored.persons.first().motivations

        assertEquals(2, motivations.size)
        assertEquals(MotivationType.POWER, motivations[0].type)
        assertEquals(8, motivations[0].intensity)
        assertEquals("Veut diriger", motivations[0].notes)
        assertEquals(MotivationType.LEARNING, motivations[1].type)
        assertEquals(6, motivations[1].intensity)
        assertEquals("", motivations[1].notes)
    }

    @Test
    fun `BackupPayload préserve les biais avec evidence`() {
        val payload = DriveSync.BackupPayload(1, 1000L, listOf(samplePerson))
        val json = gson.toJson(payload)
        val restored: DriveSync.BackupPayload = gson.fromJson(json, DriveSync.BackupPayload::class.java)
        val biases = restored.persons.first().biases

        assertEquals(2, biases.size)
        assertEquals(BiasType.CONFIRMATION, biases[0].type)
        assertEquals(7, biases[0].intensity)
        assertEquals("Ignore les avis contraires", biases[0].evidence)
        assertEquals(BiasType.ANCHORING, biases[1].type)
        assertEquals(5, biases[1].intensity)
        assertEquals("", biases[1].evidence)
    }

    @Test
    fun `BackupPayload gère une liste vide de personnes`() {
        val payload = DriveSync.BackupPayload(2, 2000L, emptyList())
        val json = gson.toJson(payload)
        val restored: DriveSync.BackupPayload = gson.fromJson(json, DriveSync.BackupPayload::class.java)

        assertEquals(2, restored.version)
        assertTrue(restored.persons.isEmpty())
    }

    @Test
    fun `BackupPayload gère des motivations et biais vides`() {
        val person = samplePerson.copy(motivations = emptyList(), biases = emptyList())
        val payload = DriveSync.BackupPayload(1, 1000L, listOf(person))
        val json = gson.toJson(payload)
        val restored: DriveSync.BackupPayload = gson.fromJson(json, DriveSync.BackupPayload::class.java)
        val p = restored.persons.first()

        assertTrue(p.motivations.isEmpty())
        assertTrue(p.biases.isEmpty())
    }

    @Test
    fun `BackupPayload gère plusieurs personnes`() {
        val p2 = samplePerson.copy(id = "p2", name = "User 2", motivations = emptyList())
        val payload = DriveSync.BackupPayload(1, 1000L, listOf(samplePerson, p2))
        val json = gson.toJson(payload)
        val restored: DriveSync.BackupPayload = gson.fromJson(json, DriveSync.BackupPayload::class.java)

        assertEquals(2, restored.persons.size)
        assertNotNull(restored.persons.find { it.id == "p2" })
    }
}
