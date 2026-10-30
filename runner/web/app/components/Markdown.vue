<script setup lang="ts">
import MarkdownIt from 'markdown-it'
import { highlight } from '~/composables/highlight'

const props = defineProps<{ source: string | null | undefined }>()

// Raw HTML is off: notes are plain Markdown, and the page runs code.
const md = new MarkdownIt({ html: false, linkify: true, typographer: false })
md.options.highlight = (code: string, lang: string) =>
  `<pre class="code"><code class="language-${lang}">${highlight(code, lang || 'text')}</code></pre>`

// External links open in a new tab, without sending a referrer.
const defaultLink = md.renderer.rules.link_open ?? ((t, i, o, _e, s) => s.renderToken(t, i, o))
md.renderer.rules.link_open = (tokens, idx, options, env, self) => {
  tokens[idx].attrSet('target', '_blank')
  tokens[idx].attrSet('rel', 'noopener noreferrer')
  return defaultLink(tokens, idx, options, env, self)
}

const html = computed(() => (props.source ? md.render(props.source) : ''))
</script>

<template>
  <div class="prose" v-html="html" />
</template>
