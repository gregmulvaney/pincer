<script lang='ts' setup>
import { sleep } from '@antfu/utils'

const props = defineProps(['download'])
enum DownloadStatus {
  Pending,
  Connecting,
  Downloading,
  Completed,
  Stopped,
  Paused,
}

const downloadStatus: { [key in DownloadStatus]: DownloadStatusElement } = {
  [DownloadStatus.Pending]: {
    icon: 'i-carbon-pending',
    name: 'Pending',
    color: 'text-amber-300',
  },
  [DownloadStatus.Connecting]: {
    icon: 'i-carbon-rotate',
    name: 'Connecting',
    color: 'text-amber-300',
  },
  [DownloadStatus.Downloading]: {
    icon: 'i-carbon-cloud-download',
    name: 'Downloading',
    color: 'text-green-300',
  },
  [DownloadStatus.Completed]: {
    icon: 'i-carbon-complete',
    name: 'Completed',
    color: 'text-green-300',
  },
  [DownloadStatus.Stopped]: {
    icon: 'i-carbon-stopped-filled-alt',
    name: 'Stopped',
    color: 'text-red-300',
  },
  [DownloadStatus.Paused]: {
    icon: 'i-carbon-pause-filled',
    name: 'Paused',
    color: 'text-amber-300',
  },
}

const progressBar = ref(null)

const timer = ms => new Promise(res => setTimeout(res, ms))

async function progressBarUpdate() {
  for (let i = 0; i <= 100; i++) {
    progressBar.value.style.width = `${i}%`
    await timer(200)
  }
}
</script>

<template>
  <tr :id="download.id" class="text-sm cursor-pointer border-y border-zinc-800 hover:(bg-zinc-700)" @click="progressBarUpdate">
    <td class="table-item">
      {{ props.download.fileName }}
    </td>
    <td class="table-item">
      <div class="flex items-center space-x-1">
        <span class="w-5 h-5 flex" :class="[downloadStatus[props.download.status].icon, downloadStatus[props.download.status].color]" />
        <span>{{ downloadStatus[props.download.status].name }}</span>
      </div>
    </td>
    <td class="table-item">
      <span>{{ download.adjustedSize }}</span>
      <span class="ml-1 font-bold text-xs">{{ download.byteUnit }}</span>
    </td>
    <td class="table-item">
      <span>{{ download.host }}</span>
    </td>
    <td class="table-item">
      <div class="relative">
        <div class="bg-zinc-800 w-full rounded h-5 overflow-hidden">
          <div ref="progressBar" class="bg-blue-500 h-5 w-0 transition-all" :class="`w-[${download.progress}%]`" />
        </div>
        <span class="font-bold left-1/2 top-1/2 transform -translate-x-1/2 -translate-y-1/2 absolute text-xs">{{ download.progress }}%</span>
      </div>
    </td>
  </tr>
</template>

<style scoped>
  .table-item {
    padding: 0.75rem 1rem;
  }
</style>
