<script setup lang="ts">
const show_tooltip = ref(false)
</script>

<template>
  <div class="relative flex flex-col justify-center">
    <div class="cursor-pointer" @mouseenter="show_tooltip = true" @mouseleave="show_tooltip = false">
      <slot name="activator" />
    </div>
    <div v-show="show_tooltip" class="absolute -bottom-8 left-1/2 transform -translate-x-1/2 bg-zinc-700 rounded px-2 py-1 text-sm shadow-sm ">
      <slot name="default" />
    </div>
  </div>
</template>
