module.exports = {
  purge: false, // we have manually configured purgecss
  darkMode: 'media', // or 'media' or 'class'
  content: [
    './templates/**/*.html'
  ],
  theme: {
    extend: {
      // fontFamily: {
      //   // 'sans': ['JetBrains Mono'],
      //   'mono': ['JetBrains Mono'],
      //   // 'display': ['JetBrains Mono'],
      //   // 'body': ['JetBrains Mono'],
      // },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
}
