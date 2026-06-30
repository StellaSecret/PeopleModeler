package com.stellasecret.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.Gravity
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ArrayAdapter
import android.widget.EditText
import android.widget.ImageButton
import android.widget.ImageView
import android.widget.LinearLayout
import android.widget.Spinner
import android.widget.TextView
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import androidx.navigation.fragment.findNavController
import com.google.android.material.slider.Slider
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.data.models.Bias
import com.stellasecret.peoplemodeler.data.models.BiasType
import com.stellasecret.peoplemodeler.data.models.Motivation
import com.stellasecret.peoplemodeler.data.models.MotivationType
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.databinding.FragmentPersonEditBinding
import com.stellasecret.peoplemodeler.viewmodels.PersonViewModel
import java.util.UUID

class PersonEditFragment : Fragment() {
    @Suppress("ktlint:standard:backing-property-naming")
    private var _binding: FragmentPersonEditBinding? = null
    private val binding get() = _binding!!
    private val viewModel: PersonViewModel by activityViewModels()
    private var editingPerson: Person? = null

    private val availableEmojis =
        listOf(
            "🧑",
            "👩",
            "👨",
            "🧠",
            "🎯",
            "💼",
            "🧔",
            "👤",
            "🎭",
            "🧿",
            "🦁",
            "🦊",
            "🐺",
            "🦅",
        )
    private var selectedEmoji = "🧑"

    private val motivations = mutableListOf<Motivation>()
    private val biases = mutableListOf<Bias>()

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        state: Bundle?,
    ): View {
        _binding = FragmentPersonEditBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(
        view: View,
        savedInstanceState: Bundle?,
    ) {
        super.onViewCreated(view, savedInstanceState)

        editingPerson = viewModel.currentPerson.value
        editingPerson?.let { populateForm(it) }

        setupAvatarPicker()
        setupOceanSliders()

        binding.btnAddMotivation.setOnClickListener { showAddMotivationDialog() }
        binding.btnAddBias.setOnClickListener { showAddBiasDialog() }
        binding.btnSave.setOnClickListener { savePerson() }
        binding.btnCancel.setOnClickListener { findNavController().navigateUp() }
    }

    private fun populateForm(p: Person) {
        motivations.clear()
        motivations.addAll(p.motivations)
        biases.clear()
        biases.addAll(p.biases)

        binding.apply {
            editName.setText(p.name)
            editRole.setText(p.role)
            editContext.setText(p.context)
            editTags.setText(p.tags.joinToString(", "))
            editNotes.setText(p.notes)
            selectedEmoji = p.avatarEmoji
            textSelectedEmoji.text = selectedEmoji
            sliderO.value = p.openness.toFloat().coerceIn(1f, 10f)
            sliderC.value = p.conscientiousness.toFloat().coerceIn(1f, 10f)
            sliderE.value = p.extraversion.toFloat().coerceIn(1f, 10f)
            sliderA.value = p.agreeableness.toFloat().coerceIn(1f, 10f)
            sliderN.value = p.neuroticism.toFloat().coerceIn(1f, 10f)
            valO.text = p.openness.toString()
            valC.text = p.conscientiousness.toString()
            valE.text = p.extraversion.toString()
            valA.text = p.agreeableness.toString()
            valN.text = p.neuroticism.toString()
        }
        renderMotivations()
        renderBiases()
    }

    private fun setupAvatarPicker() {
        binding.textSelectedEmoji.text = selectedEmoji
        binding.btnPickEmoji.setOnClickListener {
            val idx = (availableEmojis.indexOf(selectedEmoji) + 1) % availableEmojis.size
            selectedEmoji = availableEmojis[idx]
            binding.textSelectedEmoji.text = selectedEmoji
        }
    }

    private fun setupOceanSliders() {
        binding.sliderO.addOnChangeListener { _, value, _ ->
            binding.valO.text = value.toInt().toString()
        }
        binding.sliderC.addOnChangeListener { _, value, _ ->
            binding.valC.text = value.toInt().toString()
        }
        binding.sliderE.addOnChangeListener { _, value, _ ->
            binding.valE.text = value.toInt().toString()
        }
        binding.sliderA.addOnChangeListener { _, value, _ ->
            binding.valA.text = value.toInt().toString()
        }
        binding.sliderN.addOnChangeListener { _, value, _ ->
            binding.valN.text = value.toInt().toString()
        }
    }

    private fun renderMotivations() {
        binding.listMotivations.removeAllViews()
        motivations.forEachIndexed { i, m ->
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
                    layoutParams =
                        LinearLayout.LayoutParams(LinearLayout.LayoutParams.WRAP_CONTENT, LinearLayout.LayoutParams.WRAP_CONTENT).apply {
                            marginEnd =
                                8
                        }
                },
            )
            if (m.notes.isNotBlank()) {
                row.addView(
                    TextView(requireContext()).apply {
                        text = m.notes
                        textSize = 12f
                        setTextColor(resources.getColor(R.color.colorTextSecondary, null))
                        layoutParams =
                            LinearLayout
                                .LayoutParams(
                                    LinearLayout.LayoutParams.WRAP_CONTENT,
                                    LinearLayout.LayoutParams.WRAP_CONTENT,
                                ).apply {
                                    marginEnd =
                                        8
                                }
                        maxLines = 1
                    },
                )
            }
            row.addView(
                ImageButton(requireContext()).apply {
                    setImageResource(android.R.drawable.ic_menu_close_clear_cancel)
                    setBackgroundColor(android.graphics.Color.TRANSPARENT)
                    scaleType = ImageView.ScaleType.FIT_CENTER
                    layoutParams = LinearLayout.LayoutParams(48, 48)
                    setOnClickListener {
                        motivations.removeAt(i)
                        renderMotivations()
                    }
                },
            )
            binding.listMotivations.addView(row)
        }
    }

    private fun renderBiases() {
        binding.listBiases.removeAllViews()
        biases.forEachIndexed { i, b ->
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
                    layoutParams =
                        LinearLayout.LayoutParams(LinearLayout.LayoutParams.WRAP_CONTENT, LinearLayout.LayoutParams.WRAP_CONTENT).apply {
                            marginEnd =
                                8
                        }
                },
            )
            if (b.evidence.isNotBlank()) {
                row.addView(
                    TextView(requireContext()).apply {
                        text = b.evidence
                        textSize = 12f
                        setTextColor(resources.getColor(R.color.colorTextSecondary, null))
                        layoutParams =
                            LinearLayout
                                .LayoutParams(
                                    LinearLayout.LayoutParams.WRAP_CONTENT,
                                    LinearLayout.LayoutParams.WRAP_CONTENT,
                                ).apply {
                                    marginEnd =
                                        8
                                }
                        maxLines = 1
                    },
                )
            }
            row.addView(
                ImageButton(requireContext()).apply {
                    setImageResource(android.R.drawable.ic_menu_close_clear_cancel)
                    setBackgroundColor(android.graphics.Color.TRANSPARENT)
                    scaleType = ImageView.ScaleType.FIT_CENTER
                    layoutParams = LinearLayout.LayoutParams(48, 48)
                    setOnClickListener {
                        biases.removeAt(i)
                        renderBiases()
                    }
                },
            )
            binding.listBiases.addView(row)
        }
    }

    private fun showAddMotivationDialog() {
        val inflater = LayoutInflater.from(requireContext())
        val view = inflater.inflate(R.layout.dialog_add_motivation, null)
        val typeSpinner = view.findViewById<Spinner>(R.id.spinnerMotivationType)
        val intensitySlider = view.findViewById<Slider>(R.id.sliderMotivationIntensity)
        val intensityLabel = view.findViewById<TextView>(R.id.labelMotivationIntensity)
        val notesInput = view.findViewById<EditText>(R.id.inputMotivationNotes)

        typeSpinner.adapter =
            ArrayAdapter(
                requireContext(),
                android.R.layout.simple_spinner_dropdown_item,
                MotivationType.entries.map { "${it.emoji} ${it.label}" },
            )
        intensitySlider.addOnChangeListener { _, v, _ ->
            intensityLabel.text = "${v.toInt()}/10"
        }

        androidx.appcompat.app.AlertDialog
            .Builder(requireContext())
            .setTitle("💡 Ajouter une motivation")
            .setView(view)
            .setPositiveButton("Ajouter") { _, _ ->
                val idx = typeSpinner.selectedItemPosition
                if (idx >= 0) {
                    motivations.add(
                        Motivation(
                            type = MotivationType.entries[idx],
                            intensity = intensitySlider.value.toInt(),
                            notes = notesInput.text.toString().trim(),
                        ),
                    )
                    renderMotivations()
                }
            }.setNegativeButton("Annuler", null)
            .show()
    }

    private fun showAddBiasDialog() {
        val inflater = LayoutInflater.from(requireContext())
        val view = inflater.inflate(R.layout.dialog_add_bias, null)
        val typeSpinner = view.findViewById<Spinner>(R.id.spinnerBiasType)
        val intensitySlider = view.findViewById<Slider>(R.id.sliderBiasIntensity)
        val intensityLabel = view.findViewById<TextView>(R.id.labelBiasIntensity)
        val evidenceInput = view.findViewById<EditText>(R.id.inputBiasEvidence)

        typeSpinner.adapter =
            ArrayAdapter(
                requireContext(),
                android.R.layout.simple_spinner_dropdown_item,
                BiasType.entries.map { "${it.emoji} ${it.label}" },
            )
        intensitySlider.addOnChangeListener { _, v, _ ->
            intensityLabel.text = "${v.toInt()}/10"
        }

        androidx.appcompat.app.AlertDialog
            .Builder(requireContext())
            .setTitle("🧠 Ajouter un biais")
            .setView(view)
            .setPositiveButton("Ajouter") { _, _ ->
                val idx = typeSpinner.selectedItemPosition
                if (idx >= 0) {
                    biases.add(
                        Bias(
                            type = BiasType.entries[idx],
                            intensity = intensitySlider.value.toInt(),
                            evidence = evidenceInput.text.toString().trim(),
                        ),
                    )
                    renderBiases()
                }
            }.setNegativeButton("Annuler", null)
            .show()
    }

    private fun savePerson() {
        val name =
            binding.editName.text
                .toString()
                .trim()
        if (name.isBlank()) {
            binding.editName.error = "Nom requis"
            return
        }
        val tags =
            binding.editTags.text
                .toString()
                .split(",")
                .map { it.trim() }
                .filter { it.isNotBlank() }

        val base = editingPerson ?: Person(id = UUID.randomUUID().toString(), name = name)
        val person =
            base.copy(
                name = name,
                role =
                    binding.editRole.text
                        .toString()
                        .trim(),
                context =
                    binding.editContext.text
                        .toString()
                        .trim(),
                avatarEmoji = selectedEmoji,
                tags = tags,
                notes =
                    binding.editNotes.text
                        .toString()
                        .trim(),
                motivations = motivations.toList(),
                biases = biases.toList(),
                openness = binding.sliderO.value.toInt(),
                conscientiousness = binding.sliderC.value.toInt(),
                extraversion = binding.sliderE.value.toInt(),
                agreeableness = binding.sliderA.value.toInt(),
                neuroticism = binding.sliderN.value.toInt(),
            )
        viewModel.savePerson(person)
        findNavController().navigateUp()
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
