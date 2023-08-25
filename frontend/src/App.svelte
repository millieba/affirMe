<script lang="ts">
  import Header from "./lib/components/molecules/Header.svelte";
  import CardGrid from "./lib/components/molecules/CardGrid.svelte";
  import MultiSelectDropdown from "./lib/components/atoms/MultiSelectDropdown.svelte";
</script>

<Header />

<div class="mx-auto pt-12 w-5/6">
  <div class="flex justify-end mb-4">
    <MultiSelectDropdown />
  </div>
  <CardGrid />
</div>
