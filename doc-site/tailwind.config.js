export default {
  content: ['./src/**/*.{svelte,ts,js}'],
  theme: {
    extend: {
      colors: {
        background: 'var(--color-background)',
        primary: 'var(--color-primary)',
        secondary: 'var(--color-secondary)',
        tertiary: 'var(--color-tertiary)',
      },
    },
  },
  plugins: [],
}
