<script lang="ts">
    import Tag from "./Tag.svelte";

    export let affirmationText: string;
    export let tags: string[] = [];
</script>

<div
    class="bg-gradient-to-b from-[#350a80] to-[#1d0646] rounded-xl shadow-lg pb-4 px-3 pt-2"
>
    {#if tags.length > 0}
        <p class="pb-2">
            {#each tags as tag}
                <Tag tagName={tag} />
            {/each}
        </p>
    {/if}
    <p class="text-[#ffffff] font-thin">{affirmationText}</p>
</div>
