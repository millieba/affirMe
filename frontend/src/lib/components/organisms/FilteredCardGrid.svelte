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

    // Set currentPage to 1 if searchInput or selectedTags changes, also update prevSearchInput and prevSelectedTags
    if (searchInput !== prevSearchInput || selectedTags !== prevSelectedTags) {
      currentPage = 1;
      prevSearchInput = searchInput;
      prevSelectedTags = selectedTags;
    }

    try {
      const response = await fetchAffirmations(
        searchInput,
        currentPage,
        itemsPerPage,
        selectedTags
      );

      affirmations =
        // currentPage is only 1 if searchInput or selectedTags changes, if we have no more affirmations to load, or if the page just loaded
        currentPage === 1
          ? response.affirmations // If currentPage is 1, we want to replace the affirmations to avoid duplicates
          : [...affirmations, ...response.affirmations]; // If currentPage is not 1, we want to append the affirmations to the existing ones

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
