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
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      fontFamily: {
        'sans': ['JetBrains Mono'],
        'mono': ['JetBrains Mono'],
        'display': ['JetBrains Mono'],
        'body': ['JetBrains Mono'],
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
