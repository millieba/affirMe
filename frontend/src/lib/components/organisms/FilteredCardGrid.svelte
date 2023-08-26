<script>
    import MultiSelectDropdown from "../atoms/MultiSelectDropdown.svelte";
    import Searchbar from "../atoms/Searchbar.svelte";
    import CardGrid from "../molecules/CardGrid.svelte";
</script>

<h1 class="text-2xl text-t1 font-light mb-6">Affirmations</h1>
<div class="flex items-center justify-between mb-5">
    <Searchbar />
    <MultiSelectDropdown />
</div>
<CardGrid />
