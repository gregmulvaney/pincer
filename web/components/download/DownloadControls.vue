<script lang="ts" setup>
interface Control {
  icon: string
  hover: string
  action?: Function
  text?: string
  overrides?: string
}

const downloadControls: Control[] = [
  {
    icon: 'i-carbon-play-filled-alt',
    hover: 'hover:text-green-300',
    text: 'Start',
  },
  {
    icon: 'i-carbon-pause-filled',
    hover: 'hover:text-amber-300',
    text: 'Pause',
  },
  {
    icon: 'i-carbon-stop-filled-alt',
    hover: 'hover:text-red-300',
    text: 'Stop',
  },
]

const appControls: Control[] = [
  {
    icon: 'i-mdi-link-plus',
    hover: 'hover:text-green-300',
  },
  {
    icon: 'i-mdi-cog',
    hover: 'hover:(anime-spin text-white transform)',
    text: 'Settings',
  },
]
</script>

<template>
  <div class="flex items-center px-4 py-2.5 justify-between">
    <div class="flex items-center space-x-3">
      <button v-for="(control, index) in downloadControls" :key="index">
        <span class="h-5 w-5 flex" :class="[control.icon, control.hover]" />
      </button>
    </div>
    <div class="flex items-center space-x-3">
      <button v-for="(control, index) in appControls" :key="index">
        <span class="h-5 w-5 flex" :class="[control.icon, control.hover, control.overrides]" />
      </button>
    </div>
  </div>
</template>

