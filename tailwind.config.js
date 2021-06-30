module.exports = {
  mode: "jit",
  purge: ["./src/**/*.rs", "./index.html"],
  darkMode: false, // or 'media' or 'class'
  theme: {
    fontFamily: {
      'nanum': ['Nanum Gothic'],
      'mulish': ['Mulish']
    },
    extend: {
      colors: {
        'primary-accent': {
          'lt': '#ff0a5c',
          'md': '#990033',
          'dk': '#660022'
        },
        'secondary-accent': {
          'lt': '#4bd2d0',
          'md': '#218380',
          'dk': '#104140'
        },
        'tertiary-accent': {
          'lt': '#fdd89b',
          'md': '#fbb13c',
          'dk': '#dc8904'
        },
        'base': {
          'lt': '#fdfffc',
          'md': '#064174',
          'dk': '#011627'
        }
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
