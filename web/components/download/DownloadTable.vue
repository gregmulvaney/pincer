<script lang="ts" setup>
interface Downloads {
  downloads: {
    id: string
    name: string
    adjustedSize: string
    unit: string
    host: string
    url: string
  }[]
}

interface DownloadStatus {
  name: string
  icon: string
  color: string
}

const tableHeaders = [
  'File Name',
  'Status',
  'Size',
  'Host',
  'Progress',
]

const statuses: DownloadStatus[] = [
  {
    name: 'Pending',
    icon: 'i-carbon-pending',
    color: 'text-amber-300',
  },
]

const getDownloadsQuery = gql`
    query {
       downloads {
            id
            name,
            adjustedSize,
            unit,
            host,
            url
       }
    }`

const downloadsSubscription = gql`
    subscription {
        integers
    }`

const { data } = await useAsyncQuery<Downloads>(getDownloadsQuery)
const { downloads } = data.value!

// TODO: handle progress updates
// TODO: Handle status switch

onMounted(() => {

})
</script>

<template>
  <div>
    <table class="table-auto w-full border-collapse">
      <thead>
        <tr class="border-y border-zinc-800">
          <th
            v-for="(header, index) in tableHeaders"
            :key="index"
            class="text-sm text-zinc-500 text-left px-4 py-2"
          >
            {{ header }}
          </th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="(download, index) in downloads"
          :key="index"
          class="border-y cursor-pointer border-zinc-800 text-sm hover:(bg-zinc-800)"
          @contextmenu.prevent="console.log('works')"
        >
          <td class="px-4 py-2 ">
            <div class="flex items-center space-x-1">
              <span class="i-carbon-image w-5 h-5 flex" />
              <span class="text-sm">{{ download.name }}</span>
            </div>
          </td>
          <td class="px-4 py-3 flex items-center">
            <span class="i-carbon-pending w-4 h-4 flex mr-1 text-amber-300" />
            <span class="text-sm">Pending</span>
          </td>
          <td class="px-4 py-3">
            <span>{{ download.adjustedSize }}</span>
            <span class="ml-1 text-sm font-bold">{{ download.unit }}</span>
          </td>
          <td class="px-4 py-3">
            {{ download.host }}
          </td>
          <td class="px-4 py-3">
            <div class="rounded w-full h-5 bg-zinc-800 relative">
              <span class="absolute left-1/2 top-1/2 transform -translate-y-1/2 -translate-x-1/2 font-bold text-sm">50%</span>
              <div class="bg-blue-500 w-1/2 h-5 rounded" />
            </div>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<style scoped></style>
