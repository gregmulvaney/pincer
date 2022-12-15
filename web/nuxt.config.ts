// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ['@unocss/nuxt', '@vueuse/nuxt', '@nuxtjs/apollo'],
  css: ['@unocss/reset/tailwind.css'],
  runtimeConfig: {
    public: {

    },
  },
  apollo: {
    clients: {
      default: {
        httpEndpoint: 'http://127.0.0.1:4000/graphql',
        wsEndpoint: 'ws://127.0.0.1:4000/ws',
      },
    },
  },
})

