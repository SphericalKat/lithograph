module.exports = {
  purge: {
    content: [
      'components/**/*.vue',
      'layouts/**/*.vue',
      'pages/**/*.vue',
      'plugins/**/*.js',
      'nuxt.config.js',
      // TypeScript
      'plugins/**/*.ts',
      'nuxt.config.ts'
    ]
  },
  darkMode: true, // or 'media' or 'class'
  theme: {
    extend: {
      fontFamily: {
        'sans': ['JetBrains Mono'],
        'mono': ['JetBrains Mono'],
        'display': ['JetBrains Mono'],
        'body': ['JetBrains Mono'],
      },

      colors: {
        'selected-grey': '#8f93a2',
        'amber': '#ffcb6b',
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
