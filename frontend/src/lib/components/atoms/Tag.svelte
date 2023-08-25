<script lang="ts">
    import Button from "./Button.svelte";

    export let tagName: string;
    export let isActive: boolean = false;

    function toggleActive() {
        isActive = !isActive;
    }
</script>

<div class="inline-flex text-xs font-light items-center rounded-full">
    <Button buttonText={tagName} onClick={toggleActive} />
</div>
