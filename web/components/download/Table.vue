<script setup lang="ts">
interface DownloadItem {
  downloads: {
    id: string
    name: string
    adjustedSize: string
    unit: string
    url: string
    host: string
  }[]
}

const table_headers = ['File Name', 'Status', 'Size', 'Host', 'Progress']

const downloads_query = gql`
  query {
          downloads {
            id
            name
            adjustedSize
            unit
            url
            host
        }
  }`

const { data } = await useAsyncQuery<DownloadItem>(downloads_query)
const { downloads } = data.value!
</script>

<template>
  <table class="table-auto w-full">
    <thead>
      <tr class="bg-zinc-800">
        <th
          v-for="(header, index) in table_headers"
          :key="index"
          class="text-sm text-left border-r border-zinc-700 px-4 py-1"
        >
          {{ header }}
        </th>
      </tr>
    </thead>
    <tbody v-if="downloads[0]">
      <tr>
        <td>Works</td>
      </tr>
    </tbody>
  </table>
  <!-- No downloads message -->
  <div v-if="!downloads[0]" class="flex h-full items-center justify-center">
    <span class="text-4xl font-bold text-zinc-700 drop-shadow"> No Downloads Added</span>
  </div>
</template>
