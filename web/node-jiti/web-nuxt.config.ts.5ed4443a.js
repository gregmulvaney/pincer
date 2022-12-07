"use strict";Object.defineProperty(exports, "__esModule", { value: true });exports.default = void 0; // https://nuxt.com/docs/api/configuration/nuxt-config
var _default = defineNuxtConfig({
  modules: ['@unocss/nuxt', '@vueuse/nuxt', '@nuxtjs/apollo'],
  css: ['@unocss/reset/tailwind.css'],
  apollo: {
    clients: {
      default: {
        httpEndpoint: 'http://localhost:4000/' } } } });exports.default = _default; /* v7-7b5cf544ab89d56a */