package com.stellasecret.peoplemodeler

import com.stellasecret.peoplemodeler.data.models.BehaviorTrigger
import com.stellasecret.peoplemodeler.data.models.BehavioralPattern
import com.stellasecret.peoplemodeler.data.models.Bias
import com.stellasecret.peoplemodeler.data.models.BiasType
import com.stellasecret.peoplemodeler.data.models.Motivation
import com.stellasecret.peoplemodeler.data.models.MotivationType
import com.stellasecret.peoplemodeler.data.models.Person
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test

/**
 * Teste la logique de génération d'insights comportementaux
 * sans dépendance Android (pas de Context/ViewModel).
 */
class BehavioralInsightTest {
    // Reproduit la logique de PersonViewModel.generateBehavioralInsight
    private fun generateInsight(
        person: Person,
        trigger: BehaviorTrigger,
    ): String {
        val topMotivation = person.motivations.maxByOrNull { it.intensity }
        val topBias = person.biases.maxByOrNull { it.intensity }
        return buildString {
            append("Sous '${trigger.name}', ${person.name} est susceptible de :\n\n")
            topMotivation?.let {
                append("• Chercher à satisfaire : ${it.type.name} ${it.type.emoji}\n")
            }
            topBias?.let {
                append("• Être influencé par : ${it.type.name} ${it.type.emoji}\n")
            }
            if (person.neuroticism > 7) append("• Réagir de façon émotionnelle\n")
            if (person.conscientiousness > 7) append("• Chercher à contrôler et planifier\n")
            if (person.agreeableness > 7) append("• Éviter le conflit, rechercher l'harmonie\n")
            if (person.extraversion > 7) append("• Exprimer verbalement ses préoccupations\n")
            person.behavioralPatterns
                .find { it.trigger == trigger }
                ?.let { append("\n📌 Comportement observé : ${it.predictedBehavior}") }
        }
    }

    private lateinit var basePerson: Person

    @Before
    fun setup() {
        basePerson =
            Person(
                id = "insight-test",
                name = "Sophie Martin",
                motivations =
                    listOf(
                        Motivation(MotivationType.POWER, 9),
                        Motivation(MotivationType.RECOGNITION, 6),
                    ),
                biases =
                    listOf(
                        Bias(BiasType.ANCHORING, 8),
                        Bias(BiasType.LOSS_AVERSION, 5),
                    ),
                extraversion = 8,
                neuroticism = 6,
                conscientiousness = 7,
                agreeableness = 4,
            )
    }

    // ── Contenu de base ────────────────────────────────────

    @Test
    fun `l'insight contient le nom de la personne`() {
        val insight = generateInsight(basePerson, BehaviorTrigger.STRESS)
        assertTrue(insight.contains("Sophie Martin"))
    }

    @Test
    fun `l'insight contient le trigger`() {
        val insight = generateInsight(basePerson, BehaviorTrigger.STRESS)
        assertTrue(insight.contains(BehaviorTrigger.STRESS.name))
    }

    @Test
    fun `l'insight mentionne la motivation principale`() {
        val insight = generateInsight(basePerson, BehaviorTrigger.CONFLICT)
        assertTrue(insight.contains(MotivationType.POWER.name))
    }

    @Test
    fun `l'insight mentionne le biais principal`() {
        val insight = generateInsight(basePerson, BehaviorTrigger.STRESS)
        assertTrue(insight.contains(BiasType.ANCHORING.name))
    }

    // ── Traits OCEAN ──────────────────────────────────────

    @Test
    fun `extraversion élevée génère mention verbale`() {
        val person = basePerson.copy(extraversion = 9)
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertTrue("Doit mentionner l'expression verbale", insight.contains("verbalement"))
    }

    @Test
    fun `extraversion faible ne génère pas mention verbale`() {
        val person = basePerson.copy(extraversion = 3)
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertFalse(insight.contains("verbalement"))
    }

    @Test
    fun `neuroticism élevé génère mention émotionnelle`() {
        val person = basePerson.copy(neuroticism = 9)
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertTrue(insight.contains("émotionnelle"))
    }

    @Test
    fun `neuroticism faible ne génère pas mention émotionnelle`() {
        val person = basePerson.copy(neuroticism = 4)
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertFalse(insight.contains("émotionnelle"))
    }

    @Test
    fun `agréabilité élevée génère mention d'harmonie`() {
        val person = basePerson.copy(agreeableness = 9)
        val insight = generateInsight(person, BehaviorTrigger.CONFLICT)
        assertTrue(insight.contains("harmonie"))
    }

    @Test
    fun `consciencieux élevé génère mention de contrôle`() {
        val person = basePerson.copy(conscientiousness = 9)
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertTrue(insight.contains("contrôler"))
    }

    // ── Sans motivations ni biais ──────────────────────────

    @Test
    fun `insight fonctionne sans motivations`() {
        val person = basePerson.copy(motivations = emptyList())
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertFalse(insight.contains("Chercher à satisfaire"))
        assertTrue(insight.contains("Sophie Martin"))
    }

    @Test
    fun `insight fonctionne sans biais`() {
        val person = basePerson.copy(biases = emptyList())
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertFalse(insight.contains("Être influencé"))
    }

    @Test
    fun `insight fonctionne avec personne vide`() {
        val person = Person(id = "empty", name = "Inconnu")
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertNotNull(insight)
        assertTrue(insight.contains("Inconnu"))
    }

    // ── Patterns comportementaux ───────────────────────────

    @Test
    fun `pattern observé apparaît dans l'insight`() {
        val person =
            basePerson.copy(
                behavioralPatterns =
                    listOf(
                        BehavioralPattern(
                            trigger = BehaviorTrigger.STRESS,
                            predictedBehavior = "Prend le contrôle de la réunion",
                            confidence = 8,
                        ),
                    ),
            )
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertTrue(insight.contains("Prend le contrôle de la réunion"))
    }

    @Test
    fun `pattern pour un autre trigger n'apparaît pas`() {
        val person =
            basePerson.copy(
                behavioralPatterns =
                    listOf(
                        BehavioralPattern(
                            trigger = BehaviorTrigger.CONFLICT,
                            predictedBehavior = "Attaque frontalement",
                            confidence = 7,
                        ),
                    ),
            )
        val insight = generateInsight(person, BehaviorTrigger.STRESS)
        assertFalse(insight.contains("Attaque frontalement"))
    }

    // ── Tous les triggers ──────────────────────────────────

    @Test
    fun `l'insight est généré pour tous les triggers`() {
        for (trigger in BehaviorTrigger.values()) {
            val insight = generateInsight(basePerson, trigger)
            assertNotNull("Insight null pour trigger $trigger", insight)
            assertTrue("Insight vide pour trigger $trigger", insight.isNotBlank())
        }
    }
}
