<script setup lang="ts">
import type { DownloadResult } from '~/types/types'

const table_headers: String[] = [
  'File Name',
  'Status',
  'Size',
  'Host',
  'Progress',
]

const getDownloadsQuery = gql`
    query {
      downloads {
        id,
        name,
        adjustedSize,
        unit,
        url,
        host
      }
}
`
const { data } = await useAsyncQuery<DownloadResult>(getDownloadsQuery)
const { downloads } = data.value!

const contextMenu = ref(null)
const showContext = ref(false)
const coordinates = reactive({
  top: 0,
  left: 0,
})
async function toggleContext(e: Event) {
  showContext.value = true
  coordinates.top = e.clientY
  coordinates.left = e.clientX
}
</script>

<template>
  <div>
    <table class="table w-full border-collapse">
      <thead>
        <tr class="bg-zinc-800">
          <th v-for="(head, index) in table_headers" :key="index" class="text-xs px-3 py-2 border-x border-zinc-700 text-left">
            {{ head }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(download, index) in downloads" :key="index" ref="items" class="select-none" @contextmenu.prevent="toggleContext">
          <td class="dl-table-cell">
            {{ download.name }}
          </td>
          <td class="dl-table-cell">
            Pending
          </td>
          <td class="dl-table-cell">
            <span>{{ download.adjustedSize }}</span>
            <span class="text-xs">{{ download.unit }}</span>
          </td>
          <td class="dl-table-cell">
            {{ download.host }}
          </td>
          <td class="dl-table-cell">
            Progress
          </td>
        </tr>
      </tbody>
    </table>
    <DownloadContext v-show="showContext" ref="contextMenu" :left="coordinates.left" :top="coordinates.top" @close="showContext = false" />
  </div>
</template>
