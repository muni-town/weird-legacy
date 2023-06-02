const config = {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {},
  },
  daisyui: {
    themes: [
      {
        weird: {
          "primary": "#1d4ed8",
          "secondary": "#818CF8",
          "accent": "#F471B5",
          "neutral": "#394B65",
          "base-100": "#1F2937",
          "info": "#0CA5E9",
          "success": "#1FD65F",
          "warning": "#F4BF50",
          "error": "#FB7085",
        },
      }
    ],
  },

  plugins: [require("daisyui")],
}

module.exports = config
