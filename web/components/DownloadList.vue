<script setup lang="ts">
const table_headers: String[] = [
  'Name',
  'Size',
  'Host',
  'Status',
  'Progress',
]

const getDownloadsQuery = gql`
    query getDownloads {
      downloads{
        id,
        name,
        size
      }
}
`

interface DownloadsResult {
  downloads: {
    id: string
    name: string
    size: string
  }[]
}

const context = () => {

}

// Pull download list from DB
// TODO: Add conditional for no downloads
const { data } = await useAsyncQuery<DownloadsResult>(getDownloadsQuery)
const { downloads } = data.value!
</script>

<template>
  <table class="table w-full">
    <thead>
      <tr class="bg-zinc-800">
        <th v-for="(head, index) in table_headers" :key="index" class="text-xs px-3 py-2 border-x border-zinc-700 text-left">
          {{ head }}
        </th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="(download, index) in downloads"
        :key="index" class="border-b border-zinc-800 cursor-pointer w-full"
        @contextmenu.prevent="context"
      >
        <td class="px-3 py-2 border-x border-zinc-800 text-sm">
          {{ download.name }}
        </td>
        <td class="px-3 py-2 border-x border-zinc-800 text-sm">
          {{ download.size }}
        </td>
      </tr>
    </tbody>
  </table>
</template>
