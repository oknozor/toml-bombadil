import { defineConfig } from "vitepress";

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "Toml Bombadil",
  description: "A dotfile manager written in Rust",
  base: "/toml-bombadil",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Quick start", link: "/quickstart" },
      { text: "User guide", link: "/guide/templates" },
    ],

    search: {
      provider: "local",
    },
    sidebar: {
      "/guide/": {
        base: "/guide/",
        items: [
          {
            text: "Guide",
            collapsed: false,
            items: [
              { text: "Templates", link: "templates" },
              { text: "Profiles", link: "profiles" },
              { text: "Hooks", link: "hooks" },
              { text: "Imports", link: "imports" },
              { text: "GPG secrets", link: "secrets" },
            ],
          },
        ],
      },
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/oknozor/toml-bombadil" },
    ],
  },
});
