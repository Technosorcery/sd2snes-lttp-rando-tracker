module.exports = {
  plugins: [
    require('postcss-cssnext'),
    require('tailwindcss/nesting'),
    require('tailwindcss'),
    require('postcss-calc'),
    // require('autoprefixer'),
  ],
}
