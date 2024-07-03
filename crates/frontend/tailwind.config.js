/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  darkMode: 'selector',
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
    screens: {
      sm: "640px",
      md: "768px",
      lg: "1024px",
      xl: "1280px",
      "2xl": "1536px",
    },
  },
  colors: {
    pri: {
      "50":"#eff6ff",
      "100":"#dbeafe",
      "200":"#bfdbfe",
      "300":"#93c5fd",
      "400":"#60a5fa",
      "500":"#3b82f6",
      "600":"#2563eb",
      "700":"#1d4ed8",
      "800":"#1e40af",
      "900":"#1e3a8a",
      "950":"#172554"
    },
    sec: {
      "50":"#eff6ff",
      "100":"#dbeafe",
      "200":"#bfdbfe",
      "300":"#93c5fd",
      "400":"#60a5fa",
      "500":"#3b82f6",
      "600":"#2563eb",
      "700":"#1d4ed8",
      "800":"#1e40af",
      "900":"#1e3a8a",
      "950":"#172554"
    },
    light: "#f3f4f6",
    dark: "#030712",
  },
  plugins: [],
};
