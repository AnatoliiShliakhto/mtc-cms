/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./assets/**/*.{rs,html,js}", "./mtc-wasm/src/**/*.{rs,html,css,js}", "./mtc-wasm/**/*.html"],
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
