<script lang="ts">
    export let buttonText: string;
    export let onClick: () => void;
    export let tooltipText: string = undefined; // Optional prop, default to undefined
    let isHovering = false;
</script>

<div class="relative inline-block group">
    <button
        class="px-2 py-1 rounded-full hover:bg-b1-hover bg-b1"
        on:click={onClick}
        on:mouseenter={() => (isHovering = true)}
        on:mouseleave={() => (isHovering = false)}
        on:focus={() => (isHovering = true)}
        on:blur={() => (isHovering = false)}
    >
        <span class="text-white">{buttonText}</span>
    </button>

    {#if isHovering && tooltipText !== undefined}
        <div
            role="tooltip"
            class="absolute bg-a1 text-t1 rounded-md p-1 text-xs w-32 top-1 -left-32 text-center"
        >
            {tooltipText}
        </div>
    {/if}
</div>
