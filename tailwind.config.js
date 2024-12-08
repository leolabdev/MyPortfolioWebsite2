/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,rs}",
    "./index.html",
  ],
  theme: {
    extend: {
      colors: {
        "custom-bg-main": '#0d314b',
        "custom-bg-add": '#171321',
        "custom-bg-special": '#3b1f4b',

        "custom-main": '#f7f8fa',
        "custom-add": '#86fbfb',
        "custom-special": '#8e67c9',
        "custom-active": '#7a47c7',
      },
      fontFamily: {
        sans: ['Oxygen', 'sans-serif'],
        mono: ['Oxygen mono', 'monospace'],
      },
      backgroundImage: {
        'gradient': 'linear-gradient(90deg, rgba(143, 96, 213, 1) 0%, rgba(0, 212, 255, 1) 100%)'
      },
    },
  },
  plugins: [],
}

