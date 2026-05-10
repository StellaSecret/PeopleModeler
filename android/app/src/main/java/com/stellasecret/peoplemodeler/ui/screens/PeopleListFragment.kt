package com.stellasecret.peoplemodeler.ui.screens

import android.os.Bundle
import android.view.*
import androidx.appcompat.widget.SearchView
import androidx.fragment.app.Fragment
import androidx.fragment.app.activityViewModels
import androidx.navigation.fragment.findNavController
import androidx.recyclerview.widget.LinearLayoutManager
import com.google.android.material.dialog.MaterialAlertDialogBuilder
import com.stellasecret.peoplemodeler.R
import com.stellasecret.peoplemodeler.data.models.Person
import com.stellasecret.peoplemodeler.databinding.FragmentPeopleListBinding
import com.stellasecret.peoplemodeler.ui.components.PersonAdapter
import com.stellasecret.peoplemodeler.viewmodels.PersonViewModel

class PeopleListFragment : Fragment() {

    private var _binding: FragmentPeopleListBinding? = null
    private val binding get() = _binding!!
    private val viewModel: PersonViewModel by activityViewModels()
    private lateinit var adapter: PersonAdapter

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?, state: Bundle?
    ): View {
        _binding = FragmentPeopleListBinding.inflate(inflater, container, false)
        return binding.root
    }

    override fun onViewCreated(view: View, savedInstanceState: Bundle?) {
        super.onViewCreated(view, savedInstanceState)
        setupRecyclerView()
        setupSearch()
        setupFab()
        observeData()
    }

    private fun setupRecyclerView() {
        adapter = PersonAdapter(
            onPersonClick = { person ->
                viewModel.loadPerson(person.id)
                findNavController().navigate(R.id.action_list_to_detail)
            },
            onPersonLongClick = { person ->
                showDeleteDialog(person)
                true
            }
        )
        binding.recyclerView.apply {
            layoutManager = LinearLayoutManager(requireContext())
            adapter = this@PeopleListFragment.adapter
        }
    }

    private fun setupSearch() {
        binding.searchView.setOnQueryTextListener(object : SearchView.OnQueryTextListener {
            override fun onQueryTextSubmit(query: String?) = false
            override fun onQueryTextChange(newText: String?): Boolean {
                viewModel.search(newText ?: "")
                return true
            }
        })
    }

    private fun setupFab() {
        binding.fabAdd.setOnClickListener {
            viewModel.loadPerson("") // clear current
            findNavController().navigate(R.id.action_list_to_edit)
        }
    }

    private fun observeData() {
        viewModel.searchResults.observe(viewLifecycleOwner) { persons ->
            adapter.submitList(persons)
            binding.emptyState.visibility =
                if (persons.isEmpty()) View.VISIBLE else View.GONE
        }
    }

    private fun showDeleteDialog(person: Person) {
        MaterialAlertDialogBuilder(requireContext())
            .setTitle("Supprimer ${person.name} ?")
            .setMessage("Cette action est irréversible.")
            .setPositiveButton("Supprimer") { _, _ -> viewModel.deletePerson(person) }
            .setNegativeButton("Annuler", null)
            .show()
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}
