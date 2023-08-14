<script lang="ts">
    import { onMount } from "svelte";
    import {
        fetchRandomAffirmation,
        type Affirmation,
    } from "../../services/affirmationService";
    import Logo from "../atoms/Logo.svelte";
    import Button from "../atoms/Button.svelte";

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

<div class="flex bg-gradient-to-t from-[#1c0644] to-[#160535] pt-3 pb-2">
    <!-- Logo -->
    <div class="w-8 h-8 pl-2 mr-6">
        <Logo />
    </div>

    <!-- Title and Affirmation -->
    <div class="flex flex-grow items-center text-[#e5eeff]">
        <span class="font-serif">affir</span>
        <span class="font-serif font-bold">Me</span>

        {#if affirmation}
            <p class="ml-2 text-sm font-light">{" ~ " + affirmation.text}</p>
        {:else}
            <p class="ml-2" />
        {/if}
    </div>

    <!-- Try Again Icon, temporary solution -->
    <div class="px-2">
        <Button buttonText="&#10227;" onClick={fetchNewAffirmation} />
    </div>
</div>
