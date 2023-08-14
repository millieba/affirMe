<script lang="ts">
    export let buttonText: string;
    export let onClick: () => void;
</script>

<button
    class="px-2 py-1 rounded-full hover:bg-[#e46800] bg-[#c72a73]"
    on:click={onClick}
>
    <span class="text-[#ffffff]">{buttonText}</span>
</button>
