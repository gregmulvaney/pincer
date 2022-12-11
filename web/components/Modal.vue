<script setup lang='ts'>
import { def } from '@vue/shared'

const emit = defineEmits(['close'])
const content = ref(null)
onMounted(() => {
  onClickOutside(content, () => {
    emit('close')
  })
})
</script>

<template>
  <div class="absolute inset-0 bg-zinc-700/30 backdrop-blur-sm flex justify-center">
    <div ref="content" class="bg-zinc-900 rounded min-w-1/3 h-fit min-h-40 top-1/4 absolute px-4 py-4">
      <slot />
    </div>
  </div>
</template>
