package com.stellasecret.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.databinding.FragmentInsightsBinding
import com.stellasecret.peoplemodeler.viewmodels.PersonViewModel

class InsightsFragment : Fragment() {
    @Suppress("ktlint:standard:backing-property-naming")
    private var _binding: FragmentInsightsBinding? = null
    private val binding get() = _binding!!
    private val viewModel: PersonViewModel by activityViewModels()

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        state: Bundle?,
    ): View {
        _binding = FragmentInsightsBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(
        view: View,
        savedInstanceState: Bundle?,
    ) {
        super.onViewCreated(view, savedInstanceState)

        viewModel.allPersons.observe(viewLifecycleOwner) { persons ->
            val total = persons.size
            val avgMotivations =
                if (total > 0) {
                    persons.sumOf { it.motivations.size }.toFloat() / total
                } else {
                    0f
                }
            val avgBiases =
                if (total > 0) {
                    persons.sumOf { it.biases.size }.toFloat() / total
                } else {
                    0f
                }

            binding.apply {
                textTotalPersons.text = getString(R.string.insights_total_format, total)
                textAvgMotivations.text = getString(R.string.insights_avg_motivations_format, avgMotivations)
                textAvgBiases.text = getString(R.string.insights_avg_biases_format, avgBiases)

                // Top motivation across all persons
                val topMotivation =
                    persons
                        .flatMap { it.motivations }
                        .groupBy { it.type }
                        .maxByOrNull { (_, v) -> v.size }
                        ?.key
                val topMotLabel =
                    topMotivation
                        ?.let { getString(it.labelResId) }
                        ?: "—"
                textTopMotivation.text =
                    getString(
                        R.string.insights_top_motivation_format,
                        topMotLabel,
                    )

                // Ethics note
                textEthicsNote.text = getString(R.string.insights_ethics_detail)
            }
        }
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
