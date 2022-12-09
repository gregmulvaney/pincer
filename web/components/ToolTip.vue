<script setup lang='ts'>
const props = defineProps({
  text: {
    type: String,
  },
})

const show_tooltip = ref(false)
</script>

<template>
  <div class="relative">
    <div class="flex w-fit" @mouseenter="show_tooltip = true" @mouseleave="show_tooltip = false">
      <slot name="activator" />
    </div>
    <div v-show="show_tooltip" class="absolute w-fit rounded bg-zinc-700 px-2 py-1 text-sm top-full mt-0.5 left-1/2 transform -translate-x-1/2 shadow-sm">
      {{ props.text }}
    </div>
  </div>
</template>
