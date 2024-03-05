import {defaultTheme, defineUserConfig} from 'vuepress'
import {searchPlugin} from "@vuepress/plugin-search";
import {highlightjsPlugin} from "./hljs/hlJsPlugin";

export default defineUserConfig({
    lang: 'en-US',
    title: ' ',
    description: 'A Dotfile manager written in Rust',
    markdown: {
        code: {
            lineNumbers: false
        }
    },
    head: [
        ['link', {rel: 'icon', href: '/favicon.png'}],
        ['meta', {name: 'theme-color', content: '#f86b6a'}],
        ['meta', {name: 'apple-mobile-web-app-capable', content: 'yes'}],
        ['meta', {name: 'apple-mobile-web-app-status-bar-style', content: 'black'}],
        ['meta', {property: 'og:title', content: 'Toml Bombadil'}],
        ['meta', {property: 'og:image', content: 'https://toml-bombadil.dev/logo.png'}],
        ['meta', {property: 'twitter:card', content: 'https://toml-bombadil.dev/logo.png'}],
        ['meta', {property: 'og:description', content: 'A Dotfile manager written in Rust'}],
        ['meta', {property: 'og:width', content: '100'}],
    ],


    plugins: [
        searchPlugin({
            // options
        }),
        highlightjsPlugin,
    ],

    theme: defaultTheme({
        logo: 'logo.png',
        repo: 'https://github.com/cocogitto/cocogitto',
        docsRepo: 'https://github.com/cocogitto/website',
        navbar: [
            {
                link: '/quickstart/',
                text: 'Quickstart',
            },
            {
                link: '/docs/',
                text: 'Documentation',
            },
            {
                link: '/config/',
                text: 'Configuration reference',
            },
        ],
        sidebar: [
            {
                link: '/quickstart/',
                text: 'User quickstart',
            },
            {
                link: '/docs/',
                text: 'Documentation',
            },
            {
                link: '/config/',
                text: 'Configuration reference',
            },
        ],
    }),
})