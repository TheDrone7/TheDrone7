import { defineConfig, presetUno, presetAttributify, presetWebFonts, presetIcons } from 'unocss';

export default defineConfig({
  theme: {
    colors: {
      black: {
        '1': '#080A0A',
        '2': '#0F1111',
        '3': '#161818',
        '4': '#1D1F1F',
        '5': '#242626'
      },
      white: {
        '1': '#faffff',
        '2': '#e9eeee',
        '3': '#d7dddd',
        '4': '#c5cccc',
        '5': '#b3bbba'
      },
      primary: {
        base: '#61B64E',
        lighter: '#71BE60',
        lightest: '#7EC46E',
        darker: '#529F41',
        darkest: '#438235',
        textDark: '#3B752F',
        textLight: '#98D08A'
      },
      branding: {
        x: '#000000',
        xLight: '#FFFFFF',
        discord: '#5865F2',
        replit: '#F26207',
        github: '#24292f',
        githubLight: '#e6edf3'
      }
    },
    fontFamily: {
      sans: ['sans-serif'],
      mono: ['dm', 'monospace']
    },
    animation: {
      durations: {
        'fade-out': '300ms',
        'fade-in': '300ms'
      },
      keyframes: {
        spin: '{from{transform:rotate(0deg)}to{transform:rotate(1200deg)}}'
      }
    }
  },
  shortcuts: {
    main: `
      flex flex-col items-center justify-start 
      h-screen w-screen overflow-x-hidden overflow-y-auto
      p-y-16 p-x-4 
      line-height-normal font-sans leading-relaxed
      bg-black-3 text-white-2
      light:bg-white-3 light:text-black-2`,
    fab: `
      fixed bottom-8 right-8 z-50
      p-2
      flex items-center text-center justify-center
      bg-primary-base
      rounded-full text-xl
      transition-all transition-duration-300
      dark:bg-primary-darker dark:hover:bg-primary-darkest dark:text-white-1
      light:bg-primary-lighter light:hover:bg-primary-lightest light:text-black-1
      cursor-pointer outline-none border-1 border-primary-darkest border-solid
      shadow-md hover:shadow-lg focus:shadow-lg`,
    highlight: 'text-primary-text-light light:text-primary-text-dark font-bold',
    divider: 'border-b border-solid border-light-1/20 light:border-dark-1/20',
    branding: 'light:text-black-5:70 dark:text-white-5:70'
  },
  presets: [
    presetUno(),
    presetAttributify(),
    presetWebFonts({
      provider: 'google',
      fonts: {
        sans: 'Quicksand'
      }
    }),
    presetIcons({
      extraProperties: {
        display: 'inline-block',
        fill: 'currentColor',
        stroke: 'currentColor',
        'vertical-align': 'middle'
      }
    })
  ]
});
