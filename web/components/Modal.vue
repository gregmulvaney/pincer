<script setup lang="ts">
import { onClickOutside } from '@vueuse/core'

const emit = defineEmits(['close'])

const modalContent = ref(null)

onClickOutside(modalContent, () => {
  emit('close')
})
</script>

<template>
  <div class="absolute inset-0 bg-zinc-800/40 backdrop-blur-sm flex justify-center">
    <div ref="modalContent" class="px-4 py-2 bg-zinc-900 min-h-1/3 h-fit min-w-1/3 w-fit top-40 absolute shadow-lg">
      <slot />
    </div>
  </div>
</template>
