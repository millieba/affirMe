/** @type {import('tailwindcss').Config} */
export default {
  plugins: [],
  theme: {
    extend: {
      colors: {
        // Text colours
        't1': '#e5eeff',

        // Main colours
        //Card colours
        'm1': '#350a80',
        'm2': '#1d0646',

        // Header colours
        'm3': '#1c0644',
        'm4': '#160535',

        // Button colours
        'b1': '#c72a73',
        'b1-hover': '#e46800',

        // Accent colours
        'a1': '#12022e', // Dark, dark blue
        'a2': '#7d7d7d', // Grey
        'a3': '#ffba50ff', // Yellow
        'a4': '#9b4293' // Purple
      },
    },
  },
  content: ['./src/**/*.{html,js,svelte,ts}'], // purge unused css
  variants: {
    extend: {},
  },
}