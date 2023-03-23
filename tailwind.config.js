module.exports = {
    content: [
      './src/**/*.rs',
      './index.html',
      './src/**/*.html',
      './src/**/*.css'
    ],
    theme: {
      extend: {
        dropShadow: {
          blue: "0 0 2em #01A7D5"
        },
        keyframes: {
          dioxus: {
            "0%, 100%": { transform: "translateY(0px)"},
            "33%": { transform: "translateY(-10px)"},
            "66%": { transform: "translateY(10px)"},
          },
          tailwind: {
            "0%, 100%": { transform: "translateY(0px)"},
            "33%": { transform: "translateY(10px)"},
            "66%": { transform: "translateY(-10px)"},
          },
        },
        animation: {
          dioxus: "dioxus 1s ease-in-out",
          tailwind: "tailwind 1s ease-in-out",
        }
      },
    },
    variants: {},
    plugins: []
  }