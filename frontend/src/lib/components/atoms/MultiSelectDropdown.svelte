<script lang="ts">
    import { onMount } from "svelte";
    import { fetchDropdownOptions } from "../../services/affirmationService";

    let options: string[] = [];
    let selectedOptions: string[] = [];
    let dropdownOpen = false;

    onMount(async () => {
        options = await fetchDropdownOptions();
        selectAllOptions();
        console.log(selectedOptions);
    });

    function toggleOption(option: string) {
        if (selectedOptions.includes(option)) {
            selectedOptions = selectedOptions.filter((item) => item !== option);
        } else {
            selectedOptions = [...selectedOptions, option];
        }
        console.log(selectedOptions);
    }

    function toggleDropdown() {
        dropdownOpen = !dropdownOpen;
    }

    function selectAllOptions() {
        if (selectedOptions.length === options.length) {
            selectedOptions = [];
        } else {
            selectedOptions = [...options];
        }
    }

    const DropdownIcon = ({ isOpen }: { isOpen: boolean }) => `
        <svg
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            class="${isOpen ? "" : "rotate-180"}"
        >
            <path
                d="M7 15L12 10L17 15"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
            />
        </svg>
    `;
</script>

<div class="relative inline-block pl-2 text-sm">
    <div class="w-40">
        <button
            type="button"
            class="flex items-center justify-between w-full rounded-lg px-3 py-1.5 text-white bg-b1 hover:bg-b1-hover"
            id="options-menu"
            aria-haspopup="true"
            aria-expanded={dropdownOpen}
            on:click={toggleDropdown}
        >
            <span>Filter on Tags</span>
            <span class="ml-2">
                {@html DropdownIcon({ isOpen: dropdownOpen })}
            </span>
        </button>
    </div>

    {#if dropdownOpen}
        <div
            class="origin-top-right absolute right-0 mt-2 w-40 rounded-lg bg-m3 text-t1 z-10 max-h-52 overflow-scroll overflow-x-hidden"
            role="menu"
            aria-orientation="vertical"
            aria-labelledby="options-menu"
        >
            <label class="flex items-center space-x-2 py-2 px-4 cursor-pointer">
                <input
                    type="checkbox"
                    class="form-checkbox h-5 w-5 text-t1"
                    checked={selectedOptions.length === options.length}
                    on:change={selectAllOptions}
                    id="select-all"
                    role="menuitemcheckbox"
                    aria-checked={selectedOptions.length === options.length}
                />
                <span>Select All</span>
            </label>

            {#each options as option (option)}
                <label
                    class="flex items-center space-x-2 py-2 px-4 cursor-pointer hover:bg-m4
                    {selectedOptions.includes(option) ? 'bg-m4' : ''}"
                >
                    <input
                        type="checkbox"
                        class="form-checkbox h-5 w-5 text-t1"
                        checked={selectedOptions.includes(option)}
                        on:change={() => toggleOption(option)}
                        id={`option-${option}`}
                        aria-labelledby={`label-${option}`}
                        role="menuitemcheckbox"
                        aria-checked={selectedOptions.includes(option)}
                    />
                    <span id={`label-${option}`} role="menuitem">{option}</span>
                </label>
            {/each}
        </div>
    {/if}
</div>
