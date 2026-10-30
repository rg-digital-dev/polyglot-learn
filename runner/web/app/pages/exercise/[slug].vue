<script setup lang="ts">
import IconPlay from '~icons/lucide/play'
import IconCopy from '~icons/lucide/copy'
import IconLightbulb from '~icons/lucide/lightbulb'
import IconArrowLeft from '~icons/lucide/arrow-left'
import IconArrowRight from '~icons/lucide/arrow-right'
import IconExternal from '~icons/lucide/external-link'
import IconCode from '~icons/lucide/square-code'
import IconBook from '~icons/lucide/book-open'
import IconAlert from '~icons/lucide/triangle-alert'
import IconCheck from '~icons/lucide/check'

interface Exercise {
  slug: string
  title: string
  concept: string
  summary: string
  module: string | null
  track: string
  trackName: string
  toolFound: boolean
  toolLabel: string
  done: boolean
  /** A track-only exercise: one lesson, no cross-track note or comparison. */
  native: boolean
  files: { path: string; absolute: string; signatures: string[] }[]
  lesson: string | null
  note: string | null
  links: { kind: string; label: string; title: string; url: string }[]
  hints: string[]
  hintsTotal: number
  cases: { index: number; description: string; lines: string[] }[]
  prev: string | null
  next: string | null
}

const route = useRoute()
const router = useRouter()
const { state, runs, onEvent } = useLearn()
const slug = computed(() => route.params.slug as string)
const track = computed(() => (route.query.track as string) || state.value?.track || 'typescript')
const ex = ref<Exercise | null>(null)
const error = ref<string | null>(null)
const view = ref<'lesson' | 'editor'>('lesson')
const editorFile = ref<string | null>(null)

async function load() {
  error.value = null
  try {
    ex.value = await api<Exercise>(`/api/exercise/${slug.value}?track=${track.value}`)
    editorFile.value = ex.value.files[ex.value.files.length - 1]?.path ?? null
  } catch (e) {
    error.value = (e as Error).message
  }
}
watch([slug, track], load, { immediate: true })

const run = computed(() => runs.value[runKey(track.value, slug.value)])
const passing = computed(() => run.value?.final?.cases.filter((c) => c.passed).length ?? 0)

async function check() {
  try {
    await startCheck(track.value, slug.value)
  } catch (e) {
    error.value = (e as Error).message
  }
}

// A finished check can change "done" and hints; keep the header honest.
onEvent((e) => {
  if (e.type === 'check-finished' && e.slug === slug.value && e.track === track.value && ex.value) ex.value.done = ex.value.done || e.passed
})

async function nextHint() {
  const r = await api<{ hints: string[] }>(`/api/hint/${slug.value}`, { method: 'POST' })
  if (ex.value) ex.value.hints = r.hints
}

const copied = ref<string | null>(null)
async function copy(text: string) {
  try {
    await navigator.clipboard.writeText(text)
    copied.value = text
    setTimeout(() => (copied.value = null), 1500)
  } catch {}
}

function setTrack(ev: Event) {
  router.replace({ query: { ...route.query, track: (ev.target as HTMLSelectElement).value } })
}

// The watcher re-runs tests for the active track on save; for other tracks the
// page runs them after an in-page save.
function onSaved() {
  if (track.value !== state.value?.track) check()
}
</script>

<template>
  <p v-if="error" class="error-box">{{ error }}</p>
  <article v-else-if="ex" class="exercise">
    <nav class="crumbs" aria-label="Exercise navigation">
      <NuxtLink to="/" class="tag">{{ ex.module }}</NuxtLink>
      <span class="crumbs__spacer" />
      <NuxtLink v-if="ex.prev" class="iconlink" :to="`/exercise/${ex.prev}?track=${ex.track}`"><IconArrowLeft /> Previous</NuxtLink>
      <NuxtLink v-if="ex.next" class="iconlink" :to="`/exercise/${ex.next}?track=${ex.track}`">Next <IconArrowRight /></NuxtLink>
    </nav>

    <header class="exercise__head">
      <div class="exercise__titles">
        <p class="eyebrow">
          <span v-if="ex.native">{{ ex.concept }} · {{ ex.trackName }} only</span>
          <NuxtLink v-else :to="`/compare/${ex.concept}`">{{ ex.concept }}</NuxtLink>
          <span v-if="ex.done" class="eyebrow--accent"> · done</span>
        </p>
        <h1 class="h-title">{{ ex.title }}</h1>
        <p class="lead">{{ ex.summary }}</p>
      </div>
      <label class="field">
        <span class="tag">Track</span>
        <select :value="ex.track" :disabled="ex.native" @change="setTrack">
          <option v-for="t in state?.tracks ?? []" :key="t.id" :value="t.id">{{ t.name }}</option>
        </select>
      </label>
    </header>

    <div class="exercise__body">
      <div class="exercise__main">
        <div class="tabs" role="tablist">
          <button role="tab" type="button" :aria-selected="view === 'lesson'" @click="view = 'lesson'"><IconBook /> Lesson</button>
          <button role="tab" type="button" :aria-selected="view === 'editor'" @click="view = 'editor'"><IconCode /> Editor</button>
        </div>

        <div v-show="view === 'lesson'" class="stack-lg">
          <section class="panel pad">
            <Markdown :source="ex.lesson" />
          </section>
          <section v-if="!ex.native || ex.links.length" class="panel pad">
            <template v-if="!ex.native">
              <p class="tag tag--accent">{{ ex.trackName }}</p>
              <Markdown v-if="ex.note" :source="ex.note" />
              <p v-else class="muted">No {{ ex.trackName }} note for this concept yet.</p>
            </template>
            <div v-if="ex.links.length" class="links">
              <h2 class="h-card">Go deeper</h2>
              <ul>
                <li v-for="l in ex.links" :key="l.url">
                  <a :href="l.url" target="_blank" rel="noopener noreferrer" class="linkrow">
                    <span class="tag">{{ l.label }}</span>
                    <span>{{ l.title }}</span>
                    <IconExternal class="linkrow__icon" />
                  </a>
                </li>
              </ul>
            </div>
          </section>
        </div>

        <div v-if="view === 'editor'" class="stack-md">
          <div v-if="ex.files.length > 1" class="filetabs">
            <button
              v-for="f in ex.files"
              :key="f.path"
              type="button"
              class="chip"
              :class="{ 'chip--on': editorFile === f.path }"
              @click="editorFile = f.path"
            >
              {{ f.path }}
            </button>
          </div>
          <ClientOnly>
            <CodeEditor v-if="editorFile" :track="ex.track" :slug="ex.slug" :path="editorFile" @saved="onSaved" />
          </ClientOnly>
        </div>
      </div>

      <aside class="exercise__side">
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
          <div v-if="run?.final?.noneRan && !run.final.passed" class="stack-sm">
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
      </aside>
    </div>
  </article>
</template>
