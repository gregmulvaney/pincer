<script setup lang="ts">
const emit = defineEmits(['close'])
</script>

<template>
  <Modal @close="emit('close')">
    <h1>Add Link</h1>
  </Modal>
</template>

