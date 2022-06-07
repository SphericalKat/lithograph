// Export all plugins our postcss should use
module.exports = {
  plugins: [
    require("tailwindcss"),
    require("autoprefixer"),

    ...(process.env.NODE_ENV === "production"
      ? [
          require("cssnano")({
            preset: "default",
          }),
        ]
      : []),
  ],
};
