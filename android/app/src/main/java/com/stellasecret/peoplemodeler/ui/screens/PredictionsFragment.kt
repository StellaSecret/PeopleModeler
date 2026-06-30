package com.stellasecret.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import androidx.recyclerview.widget.LinearLayoutManager
import com.stellasecret.peoplemodeler.databinding.FragmentPredictionsBinding
import com.stellasecret.peoplemodeler.viewmodels.PersonViewModel

class PredictionsFragment : Fragment() {
    @Suppress("ktlint:standard:backing-property-naming")
    private var _binding: FragmentPredictionsBinding? = null
    private val binding get() = _binding!!
    private val viewModel: PersonViewModel by activityViewModels()

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        state: Bundle?,
    ): View {
        _binding = FragmentPredictionsBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(
        view: View,
        savedInstanceState: Bundle?,
    ) {
        super.onViewCreated(view, savedInstanceState)

        viewModel.pendingPredictions.observe(viewLifecycleOwner) { predictions ->
            binding.textPendingCount.text = "⏳ ${predictions.size} prédiction(s) en attente de résolution"
            binding.emptyPredictions.visibility =
                if (predictions.isEmpty()) View.VISIBLE else View.GONE
        }
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
