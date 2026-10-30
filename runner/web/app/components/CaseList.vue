<script setup lang="ts">
import IconCheck from '~icons/lucide/check'
import IconX from '~icons/lucide/x'
import IconCircle from '~icons/lucide/circle'
import IconLoader from '~icons/lucide/loader-circle'
import type { Run } from '~/composables/useLearn'

const props = defineProps<{
  cases: { index: number; description: string; function: string; lines: string[] }[]
  run: Run | undefined
}>()

function status(index: number): 'pass' | 'fail' | 'running' | 'idle' {
  const run = props.run
  if (!run) return 'idle'
  if (run.final) {
    const c = run.final.cases.find((c) => c.index === index)
    return c?.passed === true ? 'pass' : c?.passed === false ? 'fail' : 'idle'
  }
  if (index in run.live) return run.live[index] ? 'pass' : 'fail'
  return run.running ? 'running' : 'idle'
}

const message = (index: number) => props.run?.final?.cases.find((c) => c.index === index)?.message ?? null
const open = ref<Record<number, boolean>>({})
// Failing cases open by default so the reason is visible straight away.
watch(
  () => props.run?.final,
  (final) => {
    if (!final) return
    open.value = {}
    for (const c of final.cases) if (c.passed === false) open.value[c.index] = true
  },
)
</script>

<template>
  <ol class="cases">
    <li v-for="c in cases" :key="c.index" :class="['case', `case--${status(c.index)}`]">
      <button type="button" class="case__head" @click="open[c.index] = !open[c.index]">
        <span class="case__icon" :aria-label="status(c.index)">
          <IconCheck v-if="status(c.index) === 'pass'" />
          <IconX v-else-if="status(c.index) === 'fail'" />
          <IconLoader v-else-if="status(c.index) === 'running'" class="spin" />
          <IconCircle v-else />
        </span>
        <span class="case__desc">{{ c.description }}</span>
      </button>
      <div v-if="open[c.index]" class="case__body">
        <pre class="case__lines"><template v-for="l in c.lines" :key="l">{{ l }}
</template></pre>
        <pre v-if="message(c.index)" class="case__msg">{{ message(c.index) }}</pre>
      </div>
    </li>
  </ol>
</template>
