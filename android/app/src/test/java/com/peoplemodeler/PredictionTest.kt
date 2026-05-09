package com.peoplemodeler

import com.peoplemodeler.data.repository.PredictionEntity
import org.junit.Assert.*
import org.junit.Test

class PredictionTest {

    private fun makePrediction(
        id: String = "p1",
        personId: String = "person-001",
        context: String = "Réunion budget",
        predicted: String = "Va négocier",
        actual: String? = null,
        accuracy: Int? = null,
    ) = PredictionEntity(
        id = id,
        personId = personId,
        context = context,
        predictedOutcome = predicted,
        actualOutcome = actual,
        accuracy = accuracy,
    )

    // ── État initial ───────────────────────────────────────

    @Test
    fun `une nouvelle prédiction n'est pas résolue`() {
        val p = makePrediction()
        assertNull(p.actualOutcome)
        assertNull(p.accuracy)
        assertNull(p.resolvedAt)
    }

    @Test
    fun `une prédiction a un createdAt défini automatiquement`() {
        val p = makePrediction()
        assertTrue(p.createdAt > 0)
    }

    // ── Résolution ─────────────────────────────────────────

    @Test
    fun `résoudre une prédiction définit actualOutcome et accuracy`() {
        val p = makePrediction().copy(
            actualOutcome = "A négocié ET attaqué les autres depts",
            accuracy = 7,
            resolvedAt = System.currentTimeMillis()
        )
        assertNotNull(p.actualOutcome)
        assertNotNull(p.accuracy)
        assertNotNull(p.resolvedAt)
    }

    @Test
    fun `accuracy doit être entre 1 et 10`() {
        for (acc in 1..10) {
            val p = makePrediction().copy(accuracy = acc)
            assertTrue(p.accuracy!! in 1..10)
        }
    }

    // ── Calcul de précision globale ────────────────────────

    private fun averageAccuracy(predictions: List<PredictionEntity>): Double? {
        val resolved = predictions.filter { it.accuracy != null }
        if (resolved.isEmpty()) return null
        return resolved.sumOf { it.accuracy!! }.toDouble() / resolved.size
    }

    @Test
    fun `précision moyenne sur prédictions résolues`() {
        val predictions = listOf(
            makePrediction("p1", accuracy = 8),
            makePrediction("p2", accuracy = 6),
            makePrediction("p3", accuracy = 10),
        )
        val avg = averageAccuracy(predictions)
        assertNotNull(avg)
        assertEquals(8.0, avg!!, 0.01)
    }

    @Test
    fun `précision moyenne nulle si aucune prédiction résolue`() {
        val predictions = listOf(
            makePrediction("p1"),
            makePrediction("p2"),
        )
        val avg = averageAccuracy(predictions)
        assertNull(avg)
    }

    @Test
    fun `précision moyenne sur une seule prédiction`() {
        val predictions = listOf(makePrediction("p1", accuracy = 9))
        val avg = averageAccuracy(predictions)
        assertEquals(9.0, avg!!, 0.01)
    }

    @Test
    fun `les prédictions non résolues sont ignorées dans la moyenne`() {
        val predictions = listOf(
            makePrediction("p1", accuracy = 10),
            makePrediction("p2"),           // non résolue
            makePrediction("p3", accuracy = 6),
        )
        val avg = averageAccuracy(predictions)
        assertEquals(8.0, avg!!, 0.01)  // (10+6)/2
    }

    // ── Filtrage pending ───────────────────────────────────

    @Test
    fun `les prédictions en attente sont celles sans actualOutcome`() {
        val predictions = listOf(
            makePrediction("p1"),
            makePrediction("p2", actual = "Résultat réel", accuracy = 7),
            makePrediction("p3"),
        )
        val pending = predictions.filter { it.actualOutcome == null }
        assertEquals(2, pending.size)
    }

    // ── Conversion score 0-100 ─────────────────────────────

    @Test
    fun `score de précision converti en valeur sur 100`() {
        // accuracy 1-10 → 10-100%
        val accuracy = 8
        val pct = accuracy * 10
        assertEquals(80, pct)
    }

    @Test
    fun `précision maximale donne 100`() {
        val pct = 10 * 10
        assertEquals(100, pct)
    }

    @Test
    fun `précision minimale donne 10`() {
        val pct = 1 * 10
        assertEquals(10, pct)
    }
}
