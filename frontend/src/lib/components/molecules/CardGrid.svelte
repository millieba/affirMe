<script lang="ts">
    import { onMount } from "svelte";
    import Card from "../atoms/Card.svelte";
    import SkeletonCard from "../atoms/SkeletonCard.svelte";
    import Button from "../atoms/Button.svelte";
    import ShowMore from "./ShowMore.svelte";
    import type { Affirmation } from "../../services/affirmationService";

    export let affirmations: Affirmation[];
    export let currentPage: number;
    export let itemsPerPage: number;
    export let totalDocuments: number;
    export let isLoading: boolean;

    let isScrolledDown = false; // To track if user has scrolled down

    onMount(() => {
        window.addEventListener("scroll", handleScroll);
    });

    function handleScroll() {
        isScrolledDown = window.scrollY > 0;
    }

    function goToTop() {
        window.scrollTo({
            top: 0,
            behavior: "smooth",
        });
    }
</script>

<div
    class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4 pb-12"
>
    {#each affirmations as affirmation}
        <Card affirmationText={affirmation.text} tags={affirmation.tags} />
    {/each}
    {#if isLoading}
        {#each Array(itemsPerPage) as _, index}
            <SkeletonCard />
        {/each}
    {/if}
</div>

{#if !isLoading && affirmations.length < totalDocuments}
    <ShowMore bind:currentPage {itemsPerPage} bind:totalDocuments />
{/if}

{#if !isLoading && affirmations.length > 0 && isScrolledDown}
    <div class="fixed bottom-6 right-6">
        <Button buttonText="↑ Go to Top" onClick={goToTop} />
    </div>
{/if}
