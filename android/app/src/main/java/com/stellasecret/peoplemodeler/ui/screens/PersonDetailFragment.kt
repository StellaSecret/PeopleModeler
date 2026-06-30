package com.stellasecret.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.Gravity
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.LinearLayout
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import androidx.navigation.fragment.findNavController
import com.google.android.material.chip.Chip
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.data.models.BehaviorTrigger
import com.stellasecret.peoplemodeler.data.models.Person
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
        }

        binding.fabEdit.setOnClickListener {
            findNavController().navigate(R.id.action_detail_to_edit)
        }
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

            // Motivations
            listMotivations.removeAllViews()
            person.motivations.forEach { m ->
                listMotivations.addView(createMotivationRow(m))
            }

            // Biases
            listBiases.removeAllViews()
            person.biases.forEach { b ->
                listBiases.addView(createBiasRow(b))
            }

            // OCEAN bars (0–100 scale)
            barO.progress = person.openness * 10
            barC.progress = person.conscientiousness * 10
            barE.progress = person.extraversion * 10
            barA.progress = person.agreeableness * 10
            barN.progress = person.neuroticism * 10

            btnShowInsight.setOnClickListener {
                val insight = viewModel.generateBehavioralInsight(person, BehaviorTrigger.STRESS)
                textInsightOutput.text = insight
            }
        }
    }

    private fun createMotivationRow(m: com.stellasecret.peoplemodeler.data.models.Motivation): View {
        val row =
            LinearLayout(requireContext()).apply {
                orientation = LinearLayout.HORIZONTAL
                gravity = Gravity.CENTER_VERTICAL
                setPadding(0, 4, 0, 4)
            }
        row.addView(
            TextView(requireContext()).apply {
                text = "${m.type.emoji} ${m.type.label}"
                textSize = 14f
                setTextColor(resources.getColor(R.color.colorText, null))
                layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
            },
        )
        row.addView(
            TextView(requireContext()).apply {
                text = "${m.intensity}/10"
                textSize = 12f
                setTextColor(resources.getColor(R.color.colorMotivation, null))
            },
        )
        if (m.notes.isNotBlank()) {
            row.addView(
                TextView(requireContext()).apply {
                    text = "  ·  ${m.notes}"
                    textSize = 12f
                    setTextColor(resources.getColor(R.color.colorTextSecondary, null))
                    maxLines = 2
                },
            )
        }
        return row
    }

    private fun createBiasRow(b: com.stellasecret.peoplemodeler.data.models.Bias): View {
        val row =
            LinearLayout(requireContext()).apply {
                orientation = LinearLayout.HORIZONTAL
                gravity = Gravity.CENTER_VERTICAL
                setPadding(0, 4, 0, 4)
            }
        row.addView(
            TextView(requireContext()).apply {
                text = "${b.type.emoji} ${b.type.label}"
                textSize = 14f
                setTextColor(resources.getColor(R.color.colorText, null))
                layoutParams = LinearLayout.LayoutParams(0, LinearLayout.LayoutParams.WRAP_CONTENT, 1f)
            },
        )
        row.addView(
            TextView(requireContext()).apply {
                text = "${b.intensity}/10"
                textSize = 12f
                setTextColor(resources.getColor(R.color.colorBias, null))
            },
        )
        if (b.evidence.isNotBlank()) {
            row.addView(
                TextView(requireContext()).apply {
                    text = "  ·  ${b.evidence}"
                    textSize = 12f
                    setTextColor(resources.getColor(R.color.colorTextSecondary, null))
                    maxLines = 2
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
