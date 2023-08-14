<script lang="ts">
    import { onMount } from "svelte";
    import { fetchAllAffirmations } from "../../services/affirmationService";
    import Card from "../atoms/Card.svelte";
    import SkeletonCard from "../atoms/SkeletonCard.svelte";
    import Button from "../atoms/Button.svelte";

    let affirmations = [];
    let currentPage = 1;
    let itemsPerPage = 16;
    let totalDocuments = 0;
    let isLoading = true;
    let isScrolledDown = false; // To track if user has scrolled down

    async function fetchAffirmations(pageNumber: number) {
        try {
            isLoading = true;

            const startTime = Date.now();
            const response = await fetchAllAffirmations(
                pageNumber,
                itemsPerPage
            );
            const fetchTime = Date.now() - startTime;

            await (fetchTime < 300 // If the fetch time is less than 300ms, add a delay to avoid UI flickering from the skeleton cards
                ? new Promise((resolve) => setTimeout(resolve, 300))
                : Promise.resolve());

            affirmations = affirmations.concat(response.affirmations);
            totalDocuments = response.total_documents;
            isLoading = false;
        } catch (err) {
            affirmations = [];
            isLoading = false;
        }
    }

    onMount(() => {
        fetchAffirmations(currentPage);
        window.addEventListener("scroll", handleScroll);
    });

    function handleScroll() {
        isScrolledDown = window.scrollY > 0;
    }

    function loadMoreAffirmations() {
        currentPage++;
        fetchAffirmations(currentPage);
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
    <div class="flex justify-center pb-6">
        <Button
            buttonText="+ Load More"
            onClick={loadMoreAffirmations}
        />
    </div>
{/if}

{#if !isLoading && affirmations.length > 0 && isScrolledDown}
    <div class="fixed bottom-6 right-6">
        <Button buttonText="↑ Go to Top" onClick={goToTop} />
    </div>
{/if}
