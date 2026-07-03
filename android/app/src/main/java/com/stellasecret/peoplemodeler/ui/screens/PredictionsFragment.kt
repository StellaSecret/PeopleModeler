package com.stellasecret.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.LinearLayout
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.data.repository.PredictionEntity
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

        binding.recyclerPredictions.layoutManager = LinearLayoutManager(requireContext())
        val adapter =
            PredictionAdapter(
                onResolve = { pred -> showResolveDialog(pred) },
                onDelete = { pred -> showDeleteDialog(pred) },
            )
        binding.recyclerPredictions.adapter = adapter

        viewModel.pendingPredictions.observe(viewLifecycleOwner) { predictions ->
            binding.textPendingCount.text = getString(R.string.predictions_pending_format, predictions.size)
            binding.emptyPredictions.visibility =
                if (predictions.isEmpty()) View.VISIBLE else View.GONE
            adapter.submitList(predictions)
        }
    }

    private fun showResolveDialog(pred: PredictionEntity) {
        val actualInput =
            android.widget.EditText(requireContext()).apply {
                hint = getString(R.string.pred_actual_hint)
            }
        val slider =
            com.google.android.material.slider.Slider(requireContext()).apply {
                valueFrom = 1f
                valueTo = 10f
                value = 7f
            }
        val body =
            android.widget.LinearLayout(requireContext()).apply {
                orientation = LinearLayout.VERTICAL
                setPadding(48, 24, 48, 24)
                addView(actualInput)
                addView(
                    TextView(requireContext()).apply {
                        text = getString(R.string.pred_accuracy_prefix)
                        setPadding(0, 16, 0, 0)
                    },
                )
                addView(slider)
            }
        android.app.AlertDialog
            .Builder(requireContext())
            .setTitle(R.string.pred_resolve)
            .setView(body)
            .setPositiveButton(android.R.string.ok) { _, _ ->
                val actual = actualInput.text.toString().trim()
                if (actual.isEmpty()) {
                    android.app.AlertDialog
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
        android.app.AlertDialog
            .Builder(requireContext())
            .setTitle(R.string.pred_delete_title)
            .setMessage(R.string.pred_delete_message)
            .setPositiveButton(R.string.pred_delete) { _, _ -> viewModel.deletePrediction(pred) }
            .setNegativeButton(android.R.string.cancel, null)
            .show()
    }

    private class PredictionAdapter(
        private val onResolve: (PredictionEntity) -> Unit,
        private val onDelete: (PredictionEntity) -> Unit,
    ) : RecyclerView.Adapter<PredictionAdapter.ViewHolder>() {
        private val items = mutableListOf<PredictionEntity>()

        fun submitList(list: List<PredictionEntity>) {
            items.clear()
            items.addAll(list)
            notifyDataSetChanged()
        }

        override fun onCreateViewHolder(
            parent: ViewGroup,
            viewType: Int,
        ): ViewHolder {
            val v =
                LayoutInflater
                    .from(parent.context)
                    .inflate(android.R.layout.simple_list_item_2, parent, false)
            return ViewHolder(v)
        }

        override fun onBindViewHolder(
            holder: ViewHolder,
            position: Int,
        ) {
            val p = items[position]
            holder.text1.text = "📍 ${p.context}"
            holder.text2.text =
                buildString {
                    append("🔮 ${p.predictedOutcome}")
                    if (p.actualOutcome != null) {
                        append(
                            " → ${p.actualOutcome}  (${holder.itemView.context.getString(
                                R.string.pred_accuracy_prefix,
                            )}: ${p.accuracy}/10)",
                        )
                    }
                }
            holder.itemView.setOnClickListener {
                if (p.actualOutcome == null) onResolve(p)
            }
            holder.itemView.setOnLongClickListener {
                onDelete(p)
                true
            }
        }

        override fun getItemCount() = items.size

        class ViewHolder(
            v: View,
        ) : RecyclerView.ViewHolder(v) {
            val text1: TextView = v.findViewById(android.R.id.text1)
            val text2: TextView = v.findViewById(android.R.id.text2)
        }
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
