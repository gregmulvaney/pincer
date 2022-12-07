// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  modules: ['@unocss/nuxt', '@vueuse/nuxt', '@nuxtjs/apollo'],
  css: ['@unocss/reset/tailwind.css'],
  apollo: {
    clients: {
      default: {
        httpEndpoint: 'http://192.168.1.140:4000/',
      },
    },
  },
})


