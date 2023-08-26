<script lang="ts">
    import Button from "../atoms/Button.svelte";

    export let currentPage: number;
    export let totalDocuments: number;
    export let itemsPerPage: number;
    let maxPage = Math.ceil(totalDocuments / itemsPerPage);

    function loadMoreAffirmations() {
        if (currentPage + 1 <= maxPage) currentPage++;
        else currentPage = 1;
    }
</script>

<div class="flex justify-center pb-6">
    <Button buttonText="+ Load More" onClick={loadMoreAffirmations} />
</div>
