<script lang="ts">
    import { onMount } from "svelte";
    import { tick } from "svelte";

    interface SnackbarProps {
        message: string;
        duration?: number; // Optional duration in milliseconds
    }

    export let message: string;
    export let duration: number = 10000; // Default duration for snackbar is 10 seconds
    let visible = false;

    onMount(() => {
        visible = true;
        const timeoutId = setTimeout(() => {
            visible = false;
            tick(); // Trigger a reactivity update
        }, duration);

        return () => clearTimeout(timeoutId);
    });
</script>

<div
    class={`fixed bottom-4 right-4 p-4 bg-gray-700 text-white rounded ${
        visible ? "visible" : "invisible"
    }`}
>
    Message: {message}
</div>
