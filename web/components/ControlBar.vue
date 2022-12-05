<script setup lang="ts">
interface Control {
  icon: string
  hover: string
  aria: string
  action: Function
}

const start_download = () => {
  console.log('Start Downloads')
}

const pause_downloads = () => {
  console.log('Pause Downloads')
}

const stop_downloads = () => {
  console.log('Stopping Downloads')
}

const controls: Control[] = [
  {
    icon: 'i-carbon-play-filled-alt',
    hover: 'hover:text-green-300',
    aria: 'Start',
    action: start_download,
  },
  {
    icon: 'i-carbon-pause-filled',
    hover: 'hover:text-amber-300',
    aria: 'Pause',
    action: pause_downloads,
  },
  {
    icon: 'i-carbon-stop-filled-alt',
    hover: 'hover:text-red-300',
    aria: 'Stop',
    action: stop_downloads,
  },
]
</script>

<template>
  <div class="flex items-center justify-between px-4 py-2">
    <div class="flex items-center space-x-4">
      <ToolTip v-for="(control, index) in controls" :key="index">
        <template #activator>
          <button @click="control.action">
            <span :class="[control.icon, control.hover]" class="w-6 h-6 flex" />
          </button>
        </template>
        <span>{{ control.aria }}</span>
      </ToolTip>
    </div>
    <div class="flex items-center space-x-4">
      <a href="" class="flex items-center bg-green-300 text-zinc-900 rounded px-2 py-1 -mx-1 text-sm">
        <span class="i-carbon-add w-6 h-6 flex" />
        <span>Add Link</span>
      </a>
      <a href="" class="outline-none">
        <span class="i-carbon-settings flex h-6 w-6 translate hover:(text-white animate-spin)" />
      </a>
    </div>
  </div>
</template>
