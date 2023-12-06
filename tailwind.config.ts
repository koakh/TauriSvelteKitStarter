import type { Config } from 'tailwindcss'

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      fontFamily: {
        walkway: ['Walkway'],
        lobster: ['Lobster Two']
      },
    },
  },
  plugins: [],
} satisfies Config