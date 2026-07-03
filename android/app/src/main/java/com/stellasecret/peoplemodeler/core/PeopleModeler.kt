package com.stellasecret.peoplemodeler.core

object PeopleModeler {
    init {
        System.loadLibrary("peoplemodeler_core")
    }

    /** JSON in, JSON out — pass OceanScores, get interpretation string. */
    external fun analyzeOcean(json: String): String

    /** Generate behavior insight from person JSON + context key. */
    external fun generateInsight(
        ctx: String,
        personJson: String,
    ): String

    /** Predict behavior from person JSON + scenario context. */
    external fun suggestPrediction(
        personJson: String,
        context: String,
    ): String

    /** Calculate accuracy % from predictions JSON array. */
    external fun calcAccuracy(predictionsJson: String): Double

    /** Create a new Prediction via Rust core (returns JSON). */
    external fun createPrediction(
        personId: String,
        context: String,
        predictedOutcome: String,
    ): String

    /** Resolve a Prediction via Rust core (takes JSON, returns updated JSON). */
    external fun resolvePrediction(
        predictionJson: String,
        actualOutcome: String,
        accuracy: Int,
    ): String
}
