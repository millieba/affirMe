<script lang="ts">
    export let searchInput: string;

    let placeholder: string = "Search";

    function handleInput(event: Event) {
        const target = event.target as HTMLInputElement;
        searchInput = target.value;
    }
</script>

<div class="relative w-2/6 text-white text-sm">
    <input
        type="text"
        id="search"
        class="w-full rounded-lg px-10 py-2 bg-b1 hover:bg-b1-hover focus:outline-none placeholder-white"
        {placeholder}
        value={searchInput}
        on:input={handleInput}
    />
    <div class="absolute inset-y-0 left-0 flex items-center pl-3">
        <svg
            class="w-4 h-4"
            aria-hidden="true"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 20 20"
        >
            <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"
            />
        </svg>
    </div>
</div>
