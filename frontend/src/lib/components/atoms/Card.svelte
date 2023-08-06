<script lang="ts">
    import { onMount } from "svelte";
    import {
        fetchRandomAffirmation,
        type Affirmation,
    } from "../../services/affirmationService";

    let affirmation: Affirmation | null = null;
    let error: string | null = null;

    async function fetchNewAffirmation() {
        try {
            affirmation = await fetchRandomAffirmation();
            error = null;
        } catch (err) {
            affirmation = null;
            error = "Failed to fetch affirmation. Please try again later.";
        }
    }

    onMount(() => {
        fetchNewAffirmation();
    });
</script>

<!-- Display the card component -->
<div class="bg-white rounded-lg shadow-lg p-4">
    {#if error}
        <p class="text-red-600">{error}</p>
    {:else if affirmation}
        <p class="text-lg font-semibold mb-4">Your Affirmation</p>
        <p class="italic">{affirmation.text}</p>
    {:else}
        <p class="text-gray-500">Loading...</p>
    {/if}

    <button
        class="mt-4 px-4 py-2 hover:bg-[#f19060] bg-[#f19f60] text-white rounded-full shadow-[#404bbb]"
        on:click={fetchNewAffirmation}
    >
        Get Another Affirmation
    </button>
</div>
