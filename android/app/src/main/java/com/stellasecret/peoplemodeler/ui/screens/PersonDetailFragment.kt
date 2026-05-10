package com.stellasecret.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.*
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
    private var _binding: FragmentPersonDetailBinding? = null
    private val binding get() = _binding!!
    private val viewModel: PersonViewModel by activityViewModels()

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?, state: Bundle?
    ): View {
        _binding = FragmentPersonDetailBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
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
            textPersonRole.text = listOf(person.role, person.context)
                .filter { it.isNotBlank() }.joinToString(" · ")

            chipGroupTags.removeAllViews()
            person.tags.forEach { tag ->
                chipGroupTags.addView(Chip(requireContext()).apply {
                    text = tag
                    isClickable = false
                })
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

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
