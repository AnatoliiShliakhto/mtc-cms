/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./assets/**/*.{rs,html,css,js}", "./mtc-wasm/src/**/*.{rs,html,css,js}", "./mtc-wasm/**/*.html"],
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
  plugins: [
    require("@tailwindcss/typography"),
    require('daisyui'),
  ],
  daisyui: {
    //themes: false, // false: only light + dark | true: all themes | array: specific themes like this ["light", "dark", "cupcake"]
    darkTheme: "dark", // name of one of the included themes for dark mode
    base: false, // applies background color and foreground color for root element by default
    styled: true, // include daisyUI colors and design decisions for all components
    utils: true, // adds responsive and modifier utility classes
    prefix: "", // prefix for daisyUI classnames (components, modifiers and responsive class names. Not colors)
    logs: true, // Shows info about daisyUI version and used config in the console when building your CSS
    themeRoot: ":root", // The element that receives theme color CSS variables
    themes: [
      {
        light: {
          ...require("daisyui/src/theming/themes")["light"],
          "--rounded-box": "0.3rem",       // border radius rounded-box utility class, used in card and other large boxes
          "--rounded-btn": "0.3rem",       // border radius rounded-btn utility class, used in buttons and similar element
          "--rounded-badge": "1.9rem",      // border radius rounded-badge utility class, used in badges and similar
          "--animation-btn": "0.25s",       // duration of animation when you click on button
          "--animation-input": "0.2s",      // duration of animation for inputs like checkbox, toggle, radio, etc
          "--btn-focus-scale": "0.95",      // scale transform of button when you focus on it
          "--border-btn": "0px",            // border width of buttons
          "--tab-border": "0px",            // border width of tabs
          "--tab-radius": "0.5rem",         // border radius of tabs
          "--n": "50.1785% 0.02476 255.701624",
          "--bc": "33% 0.029596 256.847952"
        },
        dark: {
          ...require("daisyui/src/theming/themes")["dark"],
          "--rounded-box": "0.3rem",       // border radius rounded-box utility class, used in card and other large boxes
          "--rounded-btn": "0.3rem",       // border radius rounded-btn utility class, used in buttons and similar element
          "--rounded-badge": "1.9rem",      // border radius rounded-badge utility class, used in badges and similar
          "--animation-btn": "0.25s",       // duration of animation when you click on button
          "--animation-input": "0.2s",      // duration of animation for inputs like checkbox, toggle, radio, etc
          "--btn-focus-scale": "0.95",      // scale transform of button when you focus on it
          "--border-btn": "0px",            // border width of buttons
          "--tab-border": "0px",            // border width of tabs
          "--tab-radius": "0.5rem",         // border radius of tabs
          "--n": "40% 0.021108 254.139175"
        },
      },
    ],
  },
  corePlugins: {
    preflight: true,
  }
}
