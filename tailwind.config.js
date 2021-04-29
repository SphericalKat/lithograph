module.exports = {
  purge: false, // we have manually configured purgecss
  darkMode: false, // or 'media' or 'class'
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
