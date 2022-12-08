<script setup lang="ts">
const props = defineProps({
  top: {
    type: Number,
    default: 10,
  },
  left: {
    type: Number,
    default: 10,
  },
})
const emit = defineEmits(['close'])

const wrapper = ref(null)

onUpdated(() => {
  wrapper.value.style.top = `${props.top}px`
  wrapper.value.style.left = `${props.left}px`
})

onClickOutside(wrapper, () => {
  emit('close')
})
</script>

<template>
  <div ref="wrapper" class="rounded px-1 py-1 absolute bg-zinc-800 border border-zinc-700">
    <ul>
      <li class="flex items-center ">
        <a href="" class="w-fit grid grid-auto grid-cols-5 items-center text-xs rounded hover:bg-blue-500 px-1 py-1">
          <div class="justify-self-center">
            <span class="i-carbon-play-filled-alt w-3 h-3 flex text-green-300" />
          </div>
          <div class="col-span-4 justify-self-center">
            <span>Start Download</span>
          </div>
        </a>
      </li>
      <li class="flex items-center ">
        <a href="" class="w-fit grid grid-auto grid-cols-5 items-center text-xs rounded hover:bg-blue-500 px-1 py-1">
          <div class="justify-self-center">
            <span class="i-carbon-trash-can w-3 h-3 flex text-red-300" />
          </div>
          <div class="col-span-4 justify-self-center">
            <span>Start Download</span>
          </div>
        </a>
      </li>
    </ul>
  </div>
</template>
