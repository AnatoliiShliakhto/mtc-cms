/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./mtc-wasm/src/**/*.{rs,html,css,js}", "./**/*.{html}"],
  future: {
    hoverOnlyWhenSupported: true,
  },
  theme: {
    extend: {
      transitionProperty: {
        'list': 'color, background-color',
      }
    },
  },
  plugins: [require("daisyui")],
  corePlugins: {
    preflight: true,
  }
}
