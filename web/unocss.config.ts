import { defineConfig, presetIcons, presetUno, transformerVariantGroup } from 'unocss'

export default defineConfig({
  shortcuts: [
    ['dl-table-cell', 'border border-zinc-800 text-xs px-3 py-2'],
  ],
  presets: [
    presetUno(),
    presetIcons(),
  ],
  transformers: [
    transformerVariantGroup(),
  ],
})
