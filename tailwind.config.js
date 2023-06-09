/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["./src/**/*.rs", "./static/**/*.html"],
    theme: {
        extend: {},
    },
    plugins: [require("daisyui")],
    daisyui: {
        themes: ['forest']
    }
}