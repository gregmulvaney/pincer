<script lang="ts" setup>
interface GetDownloadsResult {
  getDownloads: DownloadResult[]
}

const tableHeaders = [
  'File Name',
  'Status',
  'Size',
  'Host',
  'Progress',
]

const getDownloadsQuery = gql`
    query {
       getDownloads {
            id
            fileName,
            status,
            adjustedSize,
            byteUnit,
            host,
            url,
            progress
       }
    }`

const { data } = await useAsyncQuery<GetDownloadsResult>(getDownloadsQuery)
const { getDownloads: downloads } = data.value!
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
        <DownloadTableItem v-for="(download, index) in downloads" :key="index" :download="download" />
      </tbody>
    </table>
  </div>
</template>
