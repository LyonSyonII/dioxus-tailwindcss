module.exports = {
    content: [
      './src/**/*.rs',
      './index.html',
      './src/**/*.html',
      './src/**/*.css'
    ],
    theme: {
      extend: {
        animation: {
          wave: 'wave 1s ease-in-out infinite',
        },
        keyframes: {
          wave: {
            '0%': {
              transform: 'translateY(0)'
            },
            '50%': {
              transform: 'translateY(-5px)'
            },
            '100%': {
              transform: 'translateY(0)'
            }
          },
        },
      },
    },
    variants: {},
    plugins: []
  }