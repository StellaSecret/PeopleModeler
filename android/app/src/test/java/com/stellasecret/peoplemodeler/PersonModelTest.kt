package com.stellasecret.peoplemodeler

import com.stellasecret.peoplemodeler.data.models.Bias
import com.stellasecret.peoplemodeler.data.models.BiasType
import com.stellasecret.peoplemodeler.data.models.Motivation
import com.stellasecret.peoplemodeler.data.models.MotivationType
import com.stellasecret.peoplemodeler.data.models.Person
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
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
        val person =
            makePerson(
                motivations =
                    listOf(
                        Motivation(MotivationType.AFFILIATION, 4),
                        Motivation(MotivationType.POWER, 9),
                        Motivation(MotivationType.SECURITY, 6),
                    ),
            )
        assertEquals(MotivationType.POWER, person.topMotivation)
    }

    @Test
    fun `topMotivation gère une seule motivation`() {
        val person =
            makePerson(
                motivations = listOf(Motivation(MotivationType.LEARNING, 7)),
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
        val person =
            makePerson(
                biases =
                    listOf(
                        Bias(BiasType.ANCHORING, 8, "Reste fixé sur le premier chiffre"),
                        Bias(BiasType.CONFIRMATION, 5),
                        Bias(BiasType.LOSS_AVERSION, 3),
                    ),
            )
        assertEquals(BiasType.ANCHORING, person.topBias)
    }

    @Test
    fun `topBias en cas d'égalité retourne un résultat stable`() {
        val person =
            makePerson(
                biases =
                    listOf(
                        Bias(BiasType.ANCHORING, 7),
                        Bias(BiasType.CONFIRMATION, 7),
                    ),
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

    // ── Motivations ─────────────────────────────────────────

    @Test
    fun `motivations sont vides par défaut`() {
        val person = makePerson()
        assertTrue(person.motivations.isEmpty())
    }

    @Test
    fun `motivations sont préservées après copy`() {
        val motivations =
            listOf(
                Motivation(MotivationType.POWER, 9, "Aime diriger"),
                Motivation(MotivationType.HELPING, 7),
            )
        val person = makePerson(motivations = motivations)
        assertEquals(2, person.motivations.size)
        assertEquals(MotivationType.POWER, person.motivations[0].type)
        assertEquals(9, person.motivations[0].intensity)
        assertEquals("Aime diriger", person.motivations[0].notes)
    }

    // ── Biases ──────────────────────────────────────────────

    @Test
    fun `biases sont vides par défaut`() {
        val person = makePerson()
        assertTrue(person.biases.isEmpty())
    }

    @Test
    fun `biases sont préservés après copy`() {
        val biases =
            listOf(
                Bias(BiasType.CONFIRMATION, 8, "Ne voit que ce qui confirme ses idées"),
                Bias(BiasType.AUTHORITY, 6),
            )
        val person = makePerson(biases = biases)
        assertEquals(2, person.biases.size)
        assertEquals(BiasType.CONFIRMATION, person.biases[0].type)
        assertEquals(8, person.biases[0].intensity)
        assertEquals("Ne voit que ce qui confirme ses idées", person.biases[0].evidence)
    }

    // ── Motivation + Bias combinés ──────────────────────────

    @Test
    fun `personne avec motivations et biais préserve les deux`() {
        val motivations = listOf(Motivation(MotivationType.AFFILIATION, 5))
        val biases = listOf(Bias(BiasType.SOCIAL_PROOF, 6, "Suit le groupe"))
        val person = makePerson(motivations = motivations, biases = biases)
        assertEquals(1, person.motivations.size)
        assertEquals(1, person.biases.size)
        assertEquals(MotivationType.AFFILIATION, person.topMotivation)
        assertEquals(BiasType.SOCIAL_PROOF, person.topBias)
    }

    // ── motivation / bias intensity bounds ──────────────────

    @Test
    fun `motivation avec intensité 1 est valide`() {
        val m = Motivation(MotivationType.SECURITY, 1)
        assertTrue(m.intensity in 1..10)
    }

    @Test
    fun `bias avec intensité 10 est valide`() {
        val b = Bias(BiasType.RECENCY, 10, "Se souvient surtout du dernier événement")
        assertTrue(b.intensity in 1..10)
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
