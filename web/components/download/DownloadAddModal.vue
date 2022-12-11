<script setup lang='ts'>
const emit = defineEmits(['close'])
</script>

<template>
  <Modal @close="emit('close')">
    <h1>Add Links</h1>
    <textarea class="w-full bg-zinc-800 resize-none outline-none p-2" />
  </Modal>
</template>
