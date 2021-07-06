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
      screens: {
        'desktop': '1280px'
      },
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
      },
      transitionProperty: {
        'border': 'border-radius',
      },
      backgroundImage: theme => ({
        'logo': "url('/static/logo.gif')",
        'primary-middle-blend': `linear-gradient(#990033, transparent, #660022)`,
      }),
      animation: {
        'slide-up': 'slide-up 250ms ease-in-out',
        'blur-in': 'blur 350ms ease-in-out'
      },
      keyframes: {
        'slide-up': {
          '0%': { opacity: 0, transform: 'translateY(120px)'},
          '100%': { opacity: 1, transform: 'translateY(0px)'}
        },
        'blur': {
          '0%': { opacity: 0, filter: 'blur(3px)'},
          '30%': { opacity: 1, filter: 'blur(3px)'},
          '100%': {opacity: 1, filter: 'blur(0px)'}
        }
      }
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
}
