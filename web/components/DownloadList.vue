<script setup lang="ts">
import { awaitExpression } from '@babel/types'

const table_headers: String[] = [
  'Name',
  'Size',
  'Host',
  'Status',
  'Progress',
]

const getDownloadsQuery = gql`
    query getAllDownloads {
        downloads {
            id
            name
        }
}
`
const { data } = await useAsyncQuery(getDownloadsQuery)

console.log(data?.value)
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
    <tbody>
      <tr v-for="(download, index) in data?.value" :key="index">
        <td>{{ download.name }}</td>
      </tr>
    </tbody>
  </table>
</template>
