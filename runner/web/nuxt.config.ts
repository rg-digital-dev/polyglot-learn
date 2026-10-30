import Icons from 'unplugin-icons/vite'

// A static single-page app: `nuxt generate` writes plain files that the learn
// binary embeds and serves. No server-side rendering, no Node at runtime.
export default defineNuxtConfig({
  compatibilityDate: '2026-10-01',
  ssr: false,
  devtools: { enabled: false },
  telemetry: false,
  // Reproducible builds: CI checks that the committed dist/ matches the sources,
  // so no random build id and no app manifest (an embedded SPA doesn't need one).
  buildId: 'learn',
  experimental: { appManifest: false },
  css: ['~/assets/brand/styles.css', '~/assets/app.css'],
  app: {
    head: {
      title: 'polyglot-learn',
      htmlAttrs: { lang: 'en' },
      meta: [
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'color-scheme', content: 'light dark' },
      ],
      link: [{ rel: 'icon', href: 'data:,' }],
    },
  },
  vite: {
    plugins: [Icons({ compiler: 'vue3', scale: 1 })],
  },
  // `nuxt dev` talks to a running `learn serve --port 4321`.
  nitro: {
    devProxy: {
      '/api': { target: 'http://127.0.0.1:4321/api', changeOrigin: true },
    },
  },
})
