package com.stellasecret.peoplemodeler

import com.stellasecret.peoplemodeler.data.models.BehaviorTrigger
import com.stellasecret.peoplemodeler.data.models.BehavioralPattern
import com.stellasecret.peoplemodeler.data.models.Bias
import com.stellasecret.peoplemodeler.data.models.BiasType
import com.stellasecret.peoplemodeler.data.models.Motivation
import com.stellasecret.peoplemodeler.data.models.MotivationType
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test

class EnumTest {
    // ── MotivationType ─────────────────────────────────────

    @Test
    fun `tous les MotivationType ont un label non vide`() {
        for (type in MotivationType.values()) {
            assertTrue(
                "Label vide pour $type",
                type.label.isNotBlank(),
            )
        }
    }

    @Test
    fun `tous les MotivationType ont un emoji non vide`() {
        for (type in MotivationType.values()) {
            assertTrue(
                "Emoji vide pour $type",
                type.emoji.isNotBlank(),
            )
        }
    }

    @Test
    fun `il y a au moins 8 types de motivation`() {
        assertTrue(MotivationType.values().size >= 8)
    }

    @Test
    fun `POWER a le bon label`() {
        assertEquals("Pouvoir", MotivationType.POWER.label)
    }

    @Test
    fun `LEARNING a le bon emoji`() {
        assertEquals("📚", MotivationType.LEARNING.emoji)
    }

    // ── BiasType ───────────────────────────────────────────

    @Test
    fun `tous les BiasType ont un label non vide`() {
        for (type in BiasType.values()) {
            assertTrue(
                "Label vide pour $type",
                type.label.isNotBlank(),
            )
        }
    }

    @Test
    fun `tous les BiasType ont un emoji non vide`() {
        for (type in BiasType.values()) {
            assertTrue(
                "Emoji vide pour $type",
                type.emoji.isNotBlank(),
            )
        }
    }

    @Test
    fun `il y a au moins 8 types de biais`() {
        assertTrue(BiasType.values().size >= 8)
    }

    @Test
    fun `ANCHORING a le bon label`() {
        assertEquals("Ancrage", BiasType.ANCHORING.label)
    }

    // ── BehaviorTrigger ────────────────────────────────────

    @Test
    fun `tous les BehaviorTrigger ont un label non vide`() {
        for (trigger in BehaviorTrigger.values()) {
            assertTrue(
                "Label vide pour $trigger",
                trigger.label.isNotBlank(),
            )
        }
    }

    @Test
    fun `il y a au moins 4 triggers comportementaux`() {
        assertTrue(BehaviorTrigger.values().size >= 4)
    }

    @Test
    fun `STRESS existe comme trigger`() {
        assertNotNull(BehaviorTrigger.valueOf("STRESS"))
    }

    @Test
    fun `CONFLICT existe comme trigger`() {
        assertNotNull(BehaviorTrigger.valueOf("CONFLICT"))
    }

    // ── Motivation data class ──────────────────────────────

    @Test
    fun `Motivation avec intensité 0 est valide`() {
        val m = Motivation(MotivationType.POWER, 0)
        assertEquals(0, m.intensity)
    }

    @Test
    fun `Motivation avec intensité 10 est valide`() {
        val m = Motivation(MotivationType.ACHIEVEMENT, 10)
        assertEquals(10, m.intensity)
    }

    @Test
    fun `Motivation notes sont vides par défaut`() {
        val m = Motivation(MotivationType.AFFILIATION, 5)
        assertEquals("", m.notes)
    }

    // ── Bias data class ────────────────────────────────────

    @Test
    fun `Bias evidence est vide par défaut`() {
        val b = Bias(BiasType.CONFIRMATION, 6)
        assertEquals("", b.evidence)
    }

    @Test
    fun `Bias avec evidence renseignée`() {
        val b = Bias(BiasType.ANCHORING, 8, "Reste fixé sur le premier prix")
        assertEquals("Reste fixé sur le premier prix", b.evidence)
    }

    // ── BehavioralPattern ──────────────────────────────────

    @Test
    fun `BehavioralPattern stocke le trigger et le comportement`() {
        val pattern =
            BehavioralPattern(
                trigger = BehaviorTrigger.STRESS,
                predictedBehavior = "Prend le contrôle",
                confidence = 8,
            )
        assertEquals(BehaviorTrigger.STRESS, pattern.trigger)
        assertEquals("Prend le contrôle", pattern.predictedBehavior)
        assertEquals(8, pattern.confidence)
    }
}
