<script setup lang="ts">
// The exercise's right column: the files to edit, Run tests, live results and hints.
import IconPlay from '~icons/lucide/play'
import IconCopy from '~icons/lucide/copy'
import IconLightbulb from '~icons/lucide/lightbulb'
import IconArrowRight from '~icons/lucide/arrow-right'
import IconAlert from '~icons/lucide/triangle-alert'
import IconCheck from '~icons/lucide/check'

const props = defineProps<{ ex: Exercise }>()
const emit = defineEmits<{ hints: [hints: string[]]; error: [message: string] }>()
const { state, runs } = useLearn()

const run = computed(() => runs.value[runKey(props.ex.track, props.ex.slug)])
const passing = computed(() => run.value?.final?.cases.filter((c) => c.passed).length ?? 0)

async function check() {
  try {
    await startCheck(props.ex.track, props.ex.slug)
  } catch (e) {
    emit('error', (e as Error).message)
  }
}

async function nextHint() {
  const r = await api<{ hints: string[] }>(`/api/hint/${props.ex.slug}`, { method: 'POST' })
  emit('hints', r.hints)
}

const copied = ref<string | null>(null)
async function copy(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    copied.value = text
    setTimeout(() => (copied.value = null), 1500)
  } catch {}
}

defineExpose({ check })
</script>

<template>
  <div class="tests-panel">
    <section class="panel pad stack-md">
      <h2 class="h-card">Edit</h2>
      <div v-for="f in ex.files" :key="f.path" class="file">
        <div class="file__path">
          <code :title="f.absolute">{{ ex.track }}/{{ f.path }}</code>
          <button class="iconbtn iconbtn--sm" type="button" title="Copy the full path" @click="copy(f.absolute)">
            <IconCheck v-if="copied === f.absolute" /><IconCopy v-else />
          </button>
        </div>
        <pre v-for="s in f.signatures" :key="s" class="code code--sig"><code v-html="highlight(s, ex.track === 'typescript' ? 'ts' : ex.track)" /></pre>
      </div>
      <p v-if="!ex.toolFound" class="warn"><IconAlert /> {{ ex.toolLabel }} isn't installed, so tests can't run. <code>learn doctor</code> shows how to install it.</p>
      <button class="btn btn--primary btn--block" type="button" :disabled="run?.running || !ex.toolFound" @click="check">
        <IconPlay class="btn__arrow" /> {{ run?.running ? 'Running…' : 'Run tests' }}
      </button>
      <p class="muted small">Saving the file also runs them{{ ex.track === state?.track ? '' : ' (for your active track)' }}.</p>
    </section>

    <section class="panel pad stack-md" aria-live="polite">
      <div class="results__head">
        <h2 class="h-card">Tests</h2>
        <span v-if="run?.final" class="pill" :class="run.final.passed ? 'pill--good' : 'pill--unsure'">
          {{ passing }}/{{ run.final.cases.length }} pass
        </span>
      </div>
      <p v-if="run?.error" class="error-box">{{ run.error }}</p>
      <p v-if="run?.final?.timedOut" class="warn">The tests took too long and were stopped. Look for an infinite loop.</p>
      <p v-if="run?.final?.setupProblem" class="warn">{{ run.final.setupProblem }}</p>
      <div v-else-if="run?.final?.noneRan && !run.final.passed && !run.final.timedOut" class="stack-sm">
        <p class="warn">No test cases ran, which usually means the code doesn't compile:</p>
        <pre class="output">{{ run.final.output }}</pre>
      </div>
      <CaseList :cases="ex.cases" :run="run" />
      <div v-if="run?.final?.passed" class="success">
        <span class="check check--sm"><IconCheck /></span>
        <span>All cases pass.</span>
        <NuxtLink v-if="run.final.next" class="text-link" :to="`/exercise/${run.final.next}?track=${ex.track}`">
          Next exercise <IconArrowRight />
        </NuxtLink>
      </div>
    </section>

    <section v-if="ex.hintsTotal" class="panel pad stack-sm">
      <h2 class="h-card">Hints</h2>
      <ol class="hints">
        <li v-for="(h, i) in ex.hints" :key="i"><span class="tag">{{ i + 1 }}</span> {{ h }}</li>
      </ol>
      <button v-if="ex.hints.length < ex.hintsTotal" class="btn btn--secondary btn--sm" type="button" @click="nextHint">
        <IconLightbulb /> Show hint {{ ex.hints.length + 1 }} of {{ ex.hintsTotal }}
      </button>
    </section>
  </div>
</template>
