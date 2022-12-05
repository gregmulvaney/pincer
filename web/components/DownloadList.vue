<script setup lang="ts">
const table_headers: String[] = [
  'Name',
  'Size',
  'Host',
  'Status',
  'Progress',
]
</script>

<template>
  <table class="table w-full">
    <thead>
      <tr class="border-y border-zinc-800">
        <th v-for="(head, index) in table_headers" :key="index" class="text-xs px-3 py-2 border-x border-zinc-800 text-left">
          {{ head }}
        </th>
      </tr>
    </thead>
  </table>
</template>
