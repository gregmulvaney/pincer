// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ['@unocss/nuxt', '@nuxtjs/apollo', '@vueuse/nuxt'],
  css: ['@unocss/reset/tailwind.css'],
  apollo: {
    clients: {
      default: {
        httpEndpoint: 'http://127.0.0.1:4000/',
      },
    },
  },
})
