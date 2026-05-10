package com.stellasecret.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.*
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import androidx.navigation.fragment.findNavController
import com.google.android.material.slider.Slider
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.databinding.FragmentPersonEditBinding
import com.stellasecret.peoplemodeler.viewmodels.PersonViewModel
import java.util.UUID

class PersonEditFragment : Fragment() {
    private var _binding: FragmentPersonEditBinding? = null
    private val binding get() = _binding!!
    private val viewModel: PersonViewModel by activityViewModels()
    private var editingPerson: Person? = null

    private val availableEmojis = listOf(
        "🧑","👩","👨","🧠","🎯","💼","🧔","👤","🎭","🧿","🦁","🦊","🐺","🦅"
    )
    private var selectedEmoji = "🧑"

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?, state: Bundle?
    ): View {
        _binding = FragmentPersonEditBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)

        editingPerson = viewModel.currentPerson.value
        editingPerson?.let { populateForm(it) }

        setupAvatarPicker()
        setupOceanSliders()

        binding.btnSave.setOnClickListener { savePerson() }
        binding.btnCancel.setOnClickListener { findNavController().navigateUp() }
    }

    private fun populateForm(p: Person) {
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

    private fun savePerson() {
        val name = binding.editName.text.toString().trim()
        if (name.isBlank()) {
            binding.editName.error = "Nom requis"
            return
        }
        val tags = binding.editTags.text.toString()
            .split(",").map { it.trim() }.filter { it.isNotBlank() }

        val base = editingPerson ?: Person(id = UUID.randomUUID().toString(), name = name)
        val person = base.copy(
            name = name,
            role = binding.editRole.text.toString().trim(),
            context = binding.editContext.text.toString().trim(),
            avatarEmoji = selectedEmoji,
            tags = tags,
            notes = binding.editNotes.text.toString().trim(),
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
