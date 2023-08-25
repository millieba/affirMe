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

    const DropdownIcon = ({ isOpen }: { isOpen: boolean }) => {
        return `
            <svg
                width="24"
                height="24"
                viewBox="0 0 24 24"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                class="${isOpen ? "rotate-180" : ""}"
            >
                <path
                    d="M7 10L12 15L17 10H7Z"
                    fill="currentColor"
                />
            </svg>
        `;
    };
</script>

<div class="relative inline-block text-left">
    <div class="w-40">
        <button
            type="button"
            class="inline-flex rounded-lg px-4 py-2 text-white bg-b1 hover:bg-b1-hover w-40"
            id="options-menu"
            aria-haspopup="true"
            aria-expanded={dropdownOpen}
            on:click={toggleDropdown}
        >
            Filter on Tags
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
