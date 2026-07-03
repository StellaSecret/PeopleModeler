package com.stellasecret.peoplemodeler.ui.screens

import android.app.AlertDialog
import android.os.Bundle
import android.view.Gravity
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import androidx.navigation.fragment.findNavController
import com.google.android.material.chip.Chip
import com.google.android.material.slider.Slider
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.data.models.BehaviorTrigger
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.data.repository.PredictionEntity
import com.stellasecret.peoplemodeler.databinding.FragmentPersonDetailBinding
import com.stellasecret.peoplemodeler.viewmodels.PersonViewModel

class PersonDetailFragment : Fragment() {
    @Suppress("ktlint:standard:backing-property-naming")
    private var _binding: FragmentPersonDetailBinding? = null
    private val binding get() = _binding!!
    private val viewModel: PersonViewModel by activityViewModels()

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        state: Bundle?,
    ): View {
        _binding = FragmentPersonDetailBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(
        view: View,
        savedInstanceState: Bundle?,
    ) {
        super.onViewCreated(view, savedInstanceState)

        viewModel.currentPerson.observe(viewLifecycleOwner) { person ->
            person ?: return@observe
            renderPerson(person)
            viewModel.getPredictions(person.id).observe(viewLifecycleOwner) { predictions ->
                renderPredictions(predictions)
            }
        }

        binding.fabEdit.setOnClickListener {
            findNavController().navigate(R.id.action_detail_to_edit)
        }

        binding.btnAddPrediction.setOnClickListener {
            val person = viewModel.currentPerson.value ?: return@setOnClickListener
            val ctx =
                binding.inputPredContext.text
                    .toString()
                    .trim()
            val outcome =
                binding.inputPredOutcome.text
                    .toString()
                    .trim()
            if (ctx.isEmpty() || outcome.isEmpty()) {
                AlertDialog
                    .Builder(requireContext())
                    .setMessage(R.string.pred_alert_fill)
                    .setPositiveButton(android.R.string.ok, null)
                    .show()
                return@setOnClickListener
            }
            viewModel.addPrediction(person.id, ctx, outcome)
            binding.inputPredContext.text.clear()
            binding.inputPredOutcome.text.clear()
        }
    }

    private fun showInsight(
        person: Person,
        trigger: BehaviorTrigger,
    ) {
        binding.textInsightOutput.text = viewModel.generateBehavioralInsight(person, trigger)
    }

    private fun renderPerson(person: Person) {
        binding.apply {
            textPersonAvatar.text = person.avatarEmoji
            textPersonName.text = person.name
            textPersonRole.text =
                listOf(person.role, person.context)
                    .filter { it.isNotBlank() }
                    .joinToString(" · ")

            chipGroupTags.removeAllViews()
            person.tags.forEach { tag ->
                chipGroupTags.addView(
                    Chip(requireContext()).apply {
                        text = tag
                        isClickable = false
                    },
                )
            }

            listMotivations.removeAllViews()
            person.motivations.forEach { m ->
                listMotivations.addView(createMotivationRow(m))
            }

            listBiases.removeAllViews()
            person.biases.forEach { b ->
                listBiases.addView(createBiasRow(b))
            }

            barO.progress = person.openness * 10
            barC.progress = person.conscientiousness * 10
            barE.progress = person.extraversion * 10
            barA.progress = person.agreeableness * 10
            barN.progress = person.neuroticism * 10

            btnInsightStress.setOnClickListener { showInsight(person, BehaviorTrigger.STRESS) }
            btnInsightConflict.setOnClickListener { showInsight(person, BehaviorTrigger.CONFLICT) }
            btnInsightSuccess.setOnClickListener { showInsight(person, BehaviorTrigger.SUCCESS) }
            btnInsightUncertainty.setOnClickListener { showInsight(person, BehaviorTrigger.UNCERTAINTY) }
            btnInsightRecognition.setOnClickListener { showInsight(person, BehaviorTrigger.RECOGNITION) }
            btnInsightThreat.setOnClickListener { showInsight(person, BehaviorTrigger.THREATENED) }
        }
    }

    private fun renderPredictions(predictions: List<PredictionEntity>) {
        binding.listPredictions.removeAllViews()
        if (predictions.isEmpty()) {
            binding.listPredictions.addView(
                TextView(requireContext()).apply {
                    text = getString(R.string.pred_empty)
                    textSize = 13f
                    setTextColor(resources.getColor(R.color.colorTextSecondary, null))
                    setPadding(0, 4, 0, 4)
                },
            )
            return
        }
        predictions.forEach { pred ->
            binding.listPredictions.addView(createPredictionRow(pred))
        }
    }

    private fun createPredictionRow(pred: PredictionEntity): View {
        val row =
            LinearLayout(requireContext()).apply {
                orientation = LinearLayout.VERTICAL
                setPadding(0, 8, 0, 8)
            }
        val ctxLine =
            TextView(requireContext()).apply {
                text = "📍 ${pred.context}"
                textSize = 13f
                setTextColor(resources.getColor(R.color.colorText, null))
            }
        row.addView(ctxLine)

        val outLine =
            TextView(requireContext()).apply {
                text = "🔮 ${pred.predictedOutcome}"
                textSize = 13f
                setTextColor(resources.getColor(R.color.colorTextSecondary, null))
            }
        row.addView(outLine)

        if (pred.actualOutcome != null) {
            val actualLine =
                TextView(requireContext()).apply {
                    text = "→ ${pred.actualOutcome}"
                    textSize = 12f
                    setTextColor(resources.getColor(R.color.colorMotivation, null))
                }
            row.addView(actualLine)

            val accLine =
                TextView(requireContext()).apply {
                    text = "${getString(R.string.pred_accuracy_prefix)} : ${pred.accuracy}/10"
                    textSize = 12f
                    setTextColor(resources.getColor(R.color.colorAccent, null))
                }
            row.addView(accLine)
        } else {
            val frame =
                LinearLayout(requireContext()).apply {
                    orientation = LinearLayout.HORIZONTAL
                    gravity = Gravity.CENTER_VERTICAL
                }
            frame.addView(
                TextView(requireContext()).apply {
                    text = getString(R.string.pred_pending_badge)
                    textSize = 12f
                    setTextColor(resources.getColor(R.color.colorAccent, null))
                    layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
                },
            )
            frame.addView(
                Button(requireContext(), null, android.R.attr.buttonBarButtonStyle).apply {
                    text = getString(R.string.pred_resolve)
                    textSize = 12f
                    setOnClickListener { showResolveDialog(pred) }
                },
            )
            row.addView(frame)
        }
        row.addView(
            Button(requireContext(), null, android.R.attr.buttonBarButtonStyle).apply {
                text = getString(R.string.pred_delete)
                textSize = 11f
                setTextColor(resources.getColor(R.color.colorBias, null))
                setOnClickListener { showDeleteDialog(pred) }
            },
        )
        return row
    }

    private fun showResolveDialog(pred: PredictionEntity) {
        val slider =
            Slider(requireContext()).apply {
                valueFrom = 1f
                valueTo = 10f
                value = 7f
            }
        val input =
            android.widget.EditText(requireContext()).apply {
                hint = getString(R.string.pred_actual_hint)
            }
        val body =
            LinearLayout(requireContext()).apply {
                orientation = LinearLayout.VERTICAL
                setPadding(48, 24, 48, 24)
                addView(input)
                addView(
                    TextView(requireContext()).apply {
                        text = getString(R.string.pred_accuracy_prefix)
                        setPadding(0, 16, 0, 0)
                    },
                )
                addView(slider)
            }
        AlertDialog
            .Builder(requireContext())
            .setTitle(R.string.pred_resolve)
            .setView(body)
            .setPositiveButton(android.R.string.ok) { _, _ ->
                val actual = input.text.toString().trim()
                if (actual.isEmpty()) {
                    AlertDialog
                        .Builder(requireContext())
                        .setMessage(R.string.pred_alert_actual)
                        .setPositiveButton(android.R.string.ok, null)
                        .show()
                    return@setPositiveButton
                }
                viewModel.resolvePrediction(pred, actual, slider.value.toInt())
            }.setNegativeButton(android.R.string.cancel, null)
            .show()
    }

    private fun showDeleteDialog(pred: PredictionEntity) {
        AlertDialog
            .Builder(requireContext())
            .setTitle(R.string.pred_delete_title)
            .setMessage(R.string.pred_delete_message)
            .setPositiveButton(R.string.pred_delete) { _, _ -> viewModel.deletePrediction(pred) }
            .setNegativeButton(android.R.string.cancel, null)
            .show()
    }

    private fun createMotivationRow(m: com.stellasecret.peoplemodeler.data.models.Motivation): View {
        val row =
            LinearLayout(requireContext()).apply {
                orientation = LinearLayout.VERTICAL
                setPadding(0, 4, 0, 4)
            }
        val topLine =
            LinearLayout(requireContext()).apply {
                orientation = LinearLayout.HORIZONTAL
                gravity = Gravity.CENTER_VERTICAL
            }
        topLine.addView(
            TextView(requireContext()).apply {
                text = "${m.type.emoji} ${getString(m.type.labelResId)}"
                textSize = 14f
                setTextColor(resources.getColor(R.color.colorText, null))
                layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
            },
        )
        topLine.addView(
            TextView(requireContext()).apply {
                text = "${m.intensity}/10"
                textSize = 12f
                setTextColor(resources.getColor(R.color.colorMotivation, null))
            },
        )
        row.addView(topLine)
        if (m.notes.isNotBlank()) {
            row.addView(
                TextView(requireContext()).apply {
                    text = m.notes
                    textSize = 12f
                    setTextColor(resources.getColor(R.color.colorTextSecondary, null))
                    setPadding(16, 2, 0, 0)
                },
            )
        }
        return row
    }

    private fun createBiasRow(b: com.stellasecret.peoplemodeler.data.models.Bias): View {
        val row =
            LinearLayout(requireContext()).apply {
                orientation = LinearLayout.VERTICAL
                setPadding(0, 4, 0, 4)
            }
        val topLine =
            LinearLayout(requireContext()).apply {
                orientation = LinearLayout.HORIZONTAL
                gravity = Gravity.CENTER_VERTICAL
            }
        topLine.addView(
            TextView(requireContext()).apply {
                text = "${b.type.emoji} ${getString(b.type.labelResId)}"
                textSize = 14f
                setTextColor(resources.getColor(R.color.colorText, null))
                layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
            },
        )
        topLine.addView(
            TextView(requireContext()).apply {
                text = "${b.intensity}/10"
                textSize = 12f
                setTextColor(resources.getColor(R.color.colorBias, null))
            },
        )
        row.addView(topLine)
        if (b.evidence.isNotBlank()) {
            row.addView(
                TextView(requireContext()).apply {
                    text = b.evidence
                    textSize = 12f
                    setTextColor(resources.getColor(R.color.colorTextSecondary, null))
                    setPadding(16, 2, 0, 0)
                },
            )
        }
        return row
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
