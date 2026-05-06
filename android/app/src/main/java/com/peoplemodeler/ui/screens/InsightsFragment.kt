package com.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.*
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import com.peoplemodeler.databinding.FragmentInsightsBinding
import com.peoplemodeler.viewmodels.PersonViewModel

class InsightsFragment : Fragment() {
    private var _binding: FragmentInsightsBinding? = null
    private val binding get() = _binding!!
    private val viewModel: PersonViewModel by activityViewModels()

    override fun onCreateView(inflater: LayoutInflater, container: ViewGroup?, state: Bundle?): View {
        _binding = FragmentInsightsBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        viewModel.allPersons.observe(viewLifecycleOwner) { persons ->
            val total = persons.size
            val avgMotivations = if (total > 0)
                persons.sumOf { it.motivations.size }.toFloat() / total else 0f
            val avgBiases = if (total > 0)
                persons.sumOf { it.biases.size }.toFloat() / total else 0f

            binding.apply {
                textTotalPersons.text = "🧩 $total profil(s) modélisé(s)"
                textAvgMotivations.text = "💡 Moyenne motivations / profil : ${"%.1f".format(avgMotivations)}"
                textAvgBiases.text = "🧠 Moyenne biais / profil : ${"%.1f".format(avgBiases)}"

                // Top motivation across all persons
                val topMotivation = persons
                    .flatMap { it.motivations }
                    .groupBy { it.type }
                    .maxByOrNull { (_, v) -> v.size }
                    ?.key
                textTopMotivation.text = "👑 Motivation dominante : ${topMotivation?.label ?: "—"}"

                // Ethics note
                textEthicsNote.text = "⚖️ Ces modèles sont des outils de compréhension.\nUtilisez-les pour améliorer vos relations, jamais pour manipuler."
            }
        }
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
