package com.stellasecret.peoplemodeler.ui.components

import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.DiffUtil
import androidx.recyclerview.widget.ListAdapter
import androidx.recyclerview.widget.RecyclerView
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.databinding.ItemPersonBinding

class PersonAdapter(
    private val onPersonClick: (Person) -> Unit,
    private val onPersonLongClick: (Person) -> Boolean
) : ListAdapter<Person, PersonAdapter.PersonViewHolder>(DiffCallback) {

    inner class PersonViewHolder(private val binding: ItemPersonBinding) :
        RecyclerView.ViewHolder(binding.root) {

        fun bind(person: Person) {
            binding.apply {
                textAvatar.text = person.avatarEmoji
                textName.text = person.name
                textRole.text = person.role.ifBlank { person.context }
                textContext.text = person.context

                // Top motivation chip
                person.topMotivation?.let { motivation ->
                    chipMotivation.text = "${motivation.emoji} ${motivation.label}"
                    chipMotivation.visibility = android.view.View.VISIBLE
                } ?: run { chipMotivation.visibility = android.view.View.GONE }

                // Top bias chip
                person.topBias?.let { bias ->
                    chipBias.text = "${bias.emoji} ${bias.label}"
                    chipBias.visibility = android.view.View.VISIBLE
                } ?: run { chipBias.visibility = android.view.View.GONE }

                // OCEAN mini-bars
                barOpenness.progress = person.openness * 10
                barConscientious.progress = person.conscientiousness * 10
                barExtraversion.progress = person.extraversion * 10

                root.setOnClickListener { onPersonClick(person) }
                root.setOnLongClickListener { onPersonLongClick(person) }
            }
        }
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): PersonViewHolder {
        val binding = ItemPersonBinding.inflate(
            LayoutInflater.from(parent.context), parent, false
        )
        return PersonViewHolder(binding)
    }

    override fun onBindViewHolder(holder: PersonViewHolder, position: Int) {
        holder.bind(getItem(position))
    }

    companion object DiffCallback : DiffUtil.ItemCallback<Person>() {
        override fun areItemsTheSame(old: Person, new: Person) = old.id == new.id
        override fun areContentsTheSame(old: Person, new: Person) = old == new
    }
}
