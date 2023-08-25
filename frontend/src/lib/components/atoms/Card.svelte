<script lang="ts">
    import Tag from "./Tag.svelte";

    export let affirmationText: string;
    export let tags: string[] = [];
</script>

<div
    class="bg-gradient-to-b from-m1 to-m2 rounded-xl shadow-lg pb-4 px-3 pt-2"
>
    {#if tags.length > 0}
        <p class="pb-2">
            {#each tags as tag}
                <Tag tagName={tag} />
            {/each}
        </p>
    {/if}
    <p class="text-t1 font-thin">{affirmationText}</p>
</div>
