<script lang="ts">
  import Header from "./lib/components/molecules/Header.svelte";
  import CardGrid from "./lib/components/molecules/CardGrid.svelte";
  import MultiSelectDropdown from "./lib/components/atoms/MultiSelectDropdown.svelte";
  import Searchbar from "./lib/components/atoms/Searchbar.svelte";
</script>

<Header />

<main class="mx-auto pt-10 w-5/6">
  <h1 class="text-2xl text-t1 font-light mb-8">Affirmations</h1>
  <div class="flex items-center justify-between mb-4">
    <Searchbar />
    <MultiSelectDropdown />
  </div>
  <CardGrid />
</main>
