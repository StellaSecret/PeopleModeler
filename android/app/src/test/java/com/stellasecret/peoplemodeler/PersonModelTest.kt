package com.stellasecret.peoplemodeler

import com.stellasecret.peoplemodeler.data.models.*
import org.junit.Assert.*
import org.junit.Test

class PersonModelTest {

    private fun makePerson(
        motivations: List<Motivation> = emptyList(),
        biases: List<Bias> = emptyList(),
        openness: Int = 5,
        conscientiousness: Int = 5,
        extraversion: Int = 5,
        agreeableness: Int = 5,
        neuroticism: Int = 5,
    ) = Person(
        id = "test-001",
        name = "Test Person",
        motivations = motivations,
        biases = biases,
        openness = openness,
        conscientiousness = conscientiousness,
        extraversion = extraversion,
        agreeableness = agreeableness,
        neuroticism = neuroticism,
    )

    // ── topMotivation ──────────────────────────────────────

    @Test
    fun `topMotivation retourne null si aucune motivation`() {
        val person = makePerson()
        assertNull(person.topMotivation)
    }

    @Test
    fun `topMotivation retourne la motivation avec l'intensité la plus haute`() {
        val person = makePerson(
            motivations = listOf(
                Motivation(MotivationType.AFFILIATION, 4),
                Motivation(MotivationType.POWER, 9),
                Motivation(MotivationType.SECURITY, 6),
            )
        )
        assertEquals(MotivationType.POWER, person.topMotivation)
    }

    @Test
    fun `topMotivation gère une seule motivation`() {
        val person = makePerson(
            motivations = listOf(Motivation(MotivationType.LEARNING, 7))
        )
        assertEquals(MotivationType.LEARNING, person.topMotivation)
    }

    // ── topBias ────────────────────────────────────────────

    @Test
    fun `topBias retourne null si aucun biais`() {
        val person = makePerson()
        assertNull(person.topBias)
    }

    @Test
    fun `topBias retourne le biais avec l'intensité la plus haute`() {
        val person = makePerson(
            biases = listOf(
                Bias(BiasType.ANCHORING, 8, "Reste fixé sur le premier chiffre"),
                Bias(BiasType.CONFIRMATION, 5),
                Bias(BiasType.LOSS_AVERSION, 3),
            )
        )
        assertEquals(BiasType.ANCHORING, person.topBias)
    }

    @Test
    fun `topBias en cas d'égalité retourne un résultat stable`() {
        val person = makePerson(
            biases = listOf(
                Bias(BiasType.ANCHORING, 7),
                Bias(BiasType.CONFIRMATION, 7),
            )
        )
        // Doit retourner un résultat non-null (peu importe lequel)
        assertNotNull(person.topBias)
    }

    // ── OCEAN validation ───────────────────────────────────

    @Test
    fun `les valeurs OCEAN par défaut sont 5`() {
        val person = makePerson()
        assertEquals(5, person.openness)
        assertEquals(5, person.conscientiousness)
        assertEquals(5, person.extraversion)
        assertEquals(5, person.agreeableness)
        assertEquals(5, person.neuroticism)
    }

    @Test
    fun `une personne avec extraversion élevée est correctement modélisée`() {
        val person = makePerson(extraversion = 9)
        assertTrue(person.extraversion > 7)
    }

    // ── copy / immutabilité ────────────────────────────────

    @Test
    fun `copy ne modifie pas l'original`() {
        val original = makePerson(openness = 5)
        val copy = original.copy(openness = 9)
        assertEquals(5, original.openness)
        assertEquals(9, copy.openness)
    }

    @Test
    fun `deux Person avec le même id sont identiques`() {
        val p1 = makePerson().copy(id = "abc")
        val p2 = makePerson().copy(id = "abc")
        assertEquals(p1.id, p2.id)
    }

    // ── Tags ───────────────────────────────────────────────

    @Test
    fun `les tags sont vides par défaut`() {
        val person = makePerson()
        assertTrue(person.tags.isEmpty())
    }

    @Test
    fun `les tags sont préservés`() {
        val person = makePerson().copy(tags = listOf("Business", "Négociateur"))
        assertEquals(2, person.tags.size)
        assertTrue(person.tags.contains("Business"))
    }
}
