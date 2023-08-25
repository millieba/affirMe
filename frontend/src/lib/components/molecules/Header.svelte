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
    let isHovering = false;

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

<div class="flex bg-gradient-to-t from-m3 to-m4 items-center">
    <!-- Logo -->
    <div class="w-16 pr-2">
        <Logo />
    </div>

    <!-- Title and Affirmation -->
    <div class="flex flex-grow items-center text-t1">
        <span class="font-serif">affir</span>
        <span class="font-serif font-bold">Me</span>

        {#if affirmation}
            <p class="ml-2 text-sm font-light">{" ~ " + affirmation.text}</p>
        {:else}
            <p class="ml-2" />
        {/if}
    </div>

    <!-- Try Again Button for fetching a new random affirmation -->
    <div class="px-2">
        <Button
            buttonText="↺"
            tooltipText="Get new affirmation"
            onClick={fetchNewAffirmation}
        />
    </div>
</div>
