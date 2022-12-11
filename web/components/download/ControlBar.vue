<script setup lang="ts">
const emit = defineEmits(['load', 'loaded'])

interface Control {
  icon: string
  hover?: string
  action?: Function
  tooltip?: string
}

const showModal = ref(false)

const start_downloads_query = gql`query {startDownloads}`
const pause_downloads_query = gql`query {pauseDownloads}`
const stop_downloads_query = gql`query {stopDownloads}`

const start_downloads = async () => {
  emit('load')
  const { data } = await useAsyncQuery(start_downloads_query)
  emit('loaded')
  console.log(data.value)
}
const pause_downloads = async () => {
  const { data } = await useAsyncQuery(pause_downloads_query)
  console.log(data.value)
}
const stop_downloads = async () => {
  const { data } = await useAsyncQuery(stop_downloads_query)
  console.log(data.value)
}

const download_controls: Control[] = [
  {
    icon: 'i-carbon-play-filled-alt',
    hover: 'hover:text-green-300',
    action: start_downloads,
    tooltip: 'Start',
  },
  {
    icon: 'i-carbon-pause-filled',
    hover: 'hover:text-amber-300',
    action: pause_downloads,
    tooltip: 'Pause',
  },
  {
    icon: 'i-carbon-stop-filled-alt',
    hover: 'hover:text-red-300',
    action: stop_downloads,
    tooltip: 'Stop',
  },
]
</script>

<template>
  <div class="flex w-full items-center px-4 py-2.5 justify-between">
    <!-- Download control buttons -->
    <div class="flex items-center space-x-4">
      <ToolTip v-for="(control, index) in download_controls" :key="index" :text="control.tooltip">
        <template #activator>
          <button
            @click="control.action"
          >
            <span :class="[control.icon, control.hover]" class="w-5 h-5 flex" />
          </button>
        </template>
      </ToolTip>
    </div>
    <!-- App control buttons -->
    <div class="flex items-center space-x-4">
      <a href="" @click.prevent="showModal = true">
        <span class="i-carbon-add h-6 w-6 flex" />
      </a>
      <a href="">
        <span
          class="i-carbon-settings h-5 w-5 flex hover:(animate-spin text-white)"
        />
      </a>
    </div>
    <DownloadAddModal v-show="showModal" @close="showModal = false" />
  </div>
</template>
