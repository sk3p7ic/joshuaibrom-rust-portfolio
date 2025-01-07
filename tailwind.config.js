// From https://github.com/drcmda/poimandres-theme/blob/main/src/theme.js

const poimandres = {
    brightYellow: '#fffac2',
    brightMint: '#5DE4c7',
    lowerMint: '#5fb3a1',
    blueishGreen: '#42675A',

    lowerBlue: '#89ddff',
    lightBlue: '#ADD7FF',
    desaturatedBlue: '#91B4D5',
    bluishGrayBrighter: '#7390AA',

    hotRed: '#d0679d',
    pink: '#f087bd',
    gray: '#a6accd',

    darkerGray: '#767c9d',
    bluishGray: '#506477',
    focus: '#303340',
    bg: '#1b1e28',

    offWhite: '#e4f0fb',
    selection: '#717cb425',

    white: '#ffffff',
    black: '#000000',
};

const poimandresStorm = {
    darkerGray: '#868cad',
    bluishGray: '#607487',
    focus: '#404350',
    bg: '#252b37',
    selection: '#818cc425',
    black: '#101010',
};

// End from


/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./src/**/*.rs'],
    theme: {
        extend: {
            colors: {
                poimandres,
                poimandresStorm
            },
            fontFamily: {
                sans: ['Inter Variable', 'sans-serif']
            },
            typography: ({theme}) => ({
                poimandres: {
                    css: {
                        '--tw-prose-body': theme('colors.poimandres.white'),
                        '--tw-prose-headings': theme('colors.poimandres.offWhite'),
                        '--tw-prose-lead': theme('colors.poimandres.white'),
                        '--tw-prose-links': theme('colors.poimandres.brightMint'),
                        '--tw-prose-bold': theme('colors.poimandres.white'),
                        '--tw-prose-counters': theme('colors.poimandres.white'),
                        '--tw-prose-bullets': theme('colors.poimandres.white'),
                        '--tw-prose-hr': theme('colors.poimandres.white'),
                        '--tw-prose-quotes': theme('colors.poimandres.white'),
                        '--tw-prose-quote-borders': theme('colors.poimandres.pink'),
                        '--tw-prose-captions': theme('colors.poimandres.white'),
                        '--tw-prose-code': theme('colors.poimandres.white'),
                        '--tw-prose-pre-code': theme('colors.poimandres.white'),
                        '--tw-prose-pre-bg': theme('colors.poimandresStorm.bg'),
                        '--tw-prose-th-borders': theme('colors.poimandres.white'),
                        '--tw-prose-td-borders': theme('colors.poimandres.white'),
                        '--tw-prose-invert-body': theme('colors.poimandres.white'),
                        '--tw-prose-invert-headings': theme('colors.white'),
                        '--tw-prose-invert-lead': theme('colors.poimandres.white'),
                        '--tw-prose-invert-links': theme('colors.white'),
                        '--tw-prose-invert-bold': theme('colors.white'),
                        '--tw-prose-invert-counters': theme('colors.poimandres.white'),
                        '--tw-prose-invert-bullets': theme('colors.poimandres.white'),
                        '--tw-prose-invert-hr': theme('colors.poimandres.white'),
                        '--tw-prose-invert-quotes': theme('colors.poimandres.white'),
                        '--tw-prose-invert-quote-borders': theme('colors.poimandres.white'),
                        '--tw-prose-invert-captions': theme('colors.poimandres.white'),
                        '--tw-prose-invert-code': theme('colors.white'),
                        '--tw-prose-invert-pre-code': theme('colors.poimandres.white'),
                        '--tw-prose-invert-pre-bg': 'rgb(0 0 0 / 50%)',
                        '--tw-prose-invert-th-borders': theme('colors.poimandres.white'),
                        '--tw-prose-invert-td-borders': theme('colors.poimandres.white'),
                    }
                }
            }),
        },
    },
    plugins: [ require('@tailwindcss/typography') ],
}
