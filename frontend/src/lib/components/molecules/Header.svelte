<script lang="ts">
    import { onMount } from "svelte";
    import {
        fetchRandomAffirmation,
        type Affirmation,
    } from "../../services/affirmationService";
    import Logo from "../atoms/Logo.svelte";

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

<div class="flex bg-[#777fff] pt-2">
    <!-- Logo -->
    <div class="w-8 h-8 pl-1 mr-8">
        <Logo />
    </div>

    <!-- Title and Affirmation -->
    <div class="flex flex-grow items-center text-white">
        <span class="font-serif">affir</span>
        <span class="font-serif font-bold">Me</span>

        {#if affirmation}
            <p class="ml-2 text-sm">{" ~ " + affirmation.text}</p>
        {:else}
            <p class="ml-2" />
        {/if}
    </div>

    <!-- Try Again Icon, temporary solution -->
    <button
        class="m-2 pl-2 pr-2 pb-1 rounded-full hover:bg-[#f19060] bg-[#f19f60]"
        on:click={fetchNewAffirmation}
    >
        <span class="text-white text-lg">&#10227;</span>
    </button>
</div>
