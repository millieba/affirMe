<script lang="ts">
  import MultiSelectDropdown from "../atoms/MultiSelectDropdown.svelte";
  import Searchbar from "../atoms/Searchbar.svelte";
  import CardGrid from "../molecules/CardGrid.svelte";
  import { fetchAffirmations } from "../../services/affirmationService";

  let searchInput = "";
  let currentPage = 1;
  let itemsPerPage = 16;
  let selectedTags = [];

  let isLoading = true;
  let affirmations = [];
  let totalDocuments = 0;

  let prevSearchInput = "";
  let prevSelectedTags = [];

  async function fetchFilteredAffirmations() {
    isLoading = true;

    currentPage =
      (currentPage !== 1 && prevSearchInput !== searchInput) ||
      prevSelectedTags !== selectedTags
        ? 1 // Reset currentPage only if searchInput or selectedTags changed
        : currentPage;

    try {
      const response = await fetchAffirmations(
        searchInput,
        currentPage,
        itemsPerPage,
        selectedTags
      );

      if (
        searchInput !== prevSearchInput ||
        selectedTags !== prevSelectedTags
      ) {
        // Update prevSearchInput and prevSelectedTags only if searchInput or selectedTags changed
        prevSearchInput = searchInput;
        prevSelectedTags = selectedTags;
        affirmations = response.affirmations; // Replace affirmations only if searchInput or selectedTags changed
      } else {
        affirmations = [...affirmations, ...response.affirmations]; // Add new affirmations to existing ones if searchInput or selectedTags did not change
      }

      totalDocuments = response.total_documents;
    } catch (err) {
      affirmations = [];
    }

    isLoading = false;
  }

  $: {
    // Trigger fetchFilteredAffirmations() when searchInput, selectedTags or currentPage changes
    searchInput, selectedTags, currentPage, fetchFilteredAffirmations();
  }
</script>

<h1 class="text-2xl text-t1 font-light mb-6">Affirmations</h1>
<div class="flex items-center justify-between mb-5">
  <Searchbar bind:searchInput />
  <MultiSelectDropdown bind:selectedOptions={selectedTags} />
</div>
<CardGrid
  {affirmations}
  bind:currentPage
  {totalDocuments}
  {isLoading}
  {itemsPerPage}
/>
