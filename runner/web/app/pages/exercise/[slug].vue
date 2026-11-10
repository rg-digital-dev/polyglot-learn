<script setup lang="ts">
// One exercise in three columns: the track's path, then Lesson / Task / Compare,
// then the tests. Narrower screens fold the path into a drawer, then stack.
import IconArrowLeft from '~icons/lucide/arrow-left'
import IconArrowRight from '~icons/lucide/arrow-right'
import IconExternal from '~icons/lucide/external-link'
import IconCode from '~icons/lucide/square-code'
import IconBook from '~icons/lucide/book-open'
import IconColumns from '~icons/lucide/columns-3'
import IconList from '~icons/lucide/list'

const route = useRoute()
const router = useRouter()
const { state, onEvent } = useLearn()
const slug = computed(() => route.params.slug as string)
const track = computed(() => (route.query.track as string) || state.value?.track || 'typescript')
const ex = ref<Exercise | null>(null)
const error = ref<string | null>(null)
const view = ref<'lesson' | 'task' | 'compare'>('lesson')
const drawer = ref(false)
const editorFile = ref<string | null>(null)
const tests = ref<{ check: () => Promise<void> } | null>(null)
const { drafts, hasDraft } = useDrafts()
const unsaved = computed(() => !!ex.value && hasDraft(ex.value.track, ex.value.slug))
const compareTracks = ref<string[]>(defaultCompareTracks(state.value))

async function load() {
  error.value = null
  try {
    ex.value = await api<Exercise>(`/api/exercise/${slug.value}?track=${track.value}`)
    editorFile.value = ex.value.files[ex.value.files.length - 1]?.path ?? null
    if (ex.value.native && view.value === 'compare') view.value = 'lesson'
  } catch (e) {
    error.value = (e as Error).message
  }
}
watch([slug, track], load, { immediate: true })

// A finished check can change "done"; keep the header honest.
onEvent((e) => {
  if (e.type === 'check-finished' && e.slug === slug.value && e.track === track.value && ex.value) ex.value.done = ex.value.done || e.passed
})

function setTrack(ev: Event) {
  router.replace({ query: { ...route.query, track: (ev.target as HTMLSelectElement).value } })
}

// The watcher re-runs tests for the active track on save; for other tracks the
// page runs them after an in-page save.
function onSaved() {
  if (track.value !== state.value?.track) tests.value?.check()
}
</script>

<template>
  <p v-if="error" class="error-box">{{ error }}</p>
  <div v-else-if="ex" class="learn" :class="{ 'learn--drawer': drawer }">
    <aside class="learn__path" :aria-hidden="undefined">
      <PathSidebar :track="ex.track" :active="ex.slug" @navigate="drawer = false" />
    </aside>
    <button v-if="drawer" type="button" class="learn__scrim" aria-label="Close the path" @click="drawer = false" />

    <article class="learn__main exercise">
      <nav class="crumbs" aria-label="Exercise navigation">
        <button type="button" class="iconlink learn__path-toggle" :aria-expanded="drawer" @click="drawer = !drawer"><IconList /> Path</button>
        <NuxtLink to="/" class="tag">{{ ex.module }}</NuxtLink>
        <span class="crumbs__spacer" />
        <NuxtLink v-if="ex.prev" class="iconlink" :to="`/exercise/${ex.prev}?track=${ex.track}`"><IconArrowLeft /> Previous</NuxtLink>
        <NuxtLink v-if="ex.next" class="iconlink" :to="`/exercise/${ex.next}?track=${ex.track}`">Next <IconArrowRight /></NuxtLink>
      </nav>

      <header class="exercise__head">
        <div class="exercise__titles">
          <p class="eyebrow">
            {{ ex.concept }}<span v-if="ex.native"> · {{ ex.trackName }} only</span>
            <span v-if="ex.done" class="eyebrow--accent"> · done</span>
          </p>
          <h1 class="h-title">{{ ex.title }}</h1>
          <p class="lead">{{ ex.summary }}</p>
        </div>
        <label class="field">
          <span class="tag">Track</span>
          <select :value="ex.track" :disabled="ex.native" @change="setTrack">
            <option v-for="t in (state?.tracks ?? []).filter((t) => ex.tracks.includes(t.id))" :key="t.id" :value="t.id">{{ t.name }}</option>
          </select>
        </label>
      </header>

      <div class="tabs" role="tablist">
        <button role="tab" type="button" :aria-selected="view === 'lesson'" @click="view = 'lesson'"><IconBook /> Lesson</button>
        <button role="tab" type="button" :aria-selected="view === 'task'" @click="view = 'task'">
          <IconCode /> Task<span v-if="unsaved" class="unsaved-dot" title="Unsaved changes" aria-label="(unsaved changes)" />
        </button>
        <button v-if="!ex.native" role="tab" type="button" :aria-selected="view === 'compare'" @click="view = 'compare'"><IconColumns /> Compare</button>
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

      <!-- Task: the editor, with the track's note beside it on wide screens. -->
      <div v-if="view === 'task'" class="task">
        <section class="panel pad task__lesson">
          <Markdown :source="ex.native ? ex.lesson : (ex.note ?? ex.lesson)" />
        </section>
        <div class="stack-md task__editor">
          <div v-if="ex.files.length > 1" class="filetabs">
            <button
              v-for="f in ex.files"
              :key="f.path"
              type="button"
              class="chip"
              :class="{ 'chip--on': editorFile === f.path }"
              @click="editorFile = f.path"
            >
              {{ f.path }}<span v-if="drafts.has(draftKey(ex.track, ex.slug, f.path))" class="unsaved-dot" title="Unsaved changes" aria-label="(unsaved changes)" />
            </button>
          </div>
          <ClientOnly>
            <CodeEditor v-if="editorFile" :track="ex.track" :slug="ex.slug" :path="editorFile" @saved="onSaved" />
          </ClientOnly>
        </div>
      </div>

      <ComparePanel
        v-if="view === 'compare' && !ex.native"
        :concept="ex.concept"
        :selected="compareTracks"
        @toggle="compareTracks = toggled(compareTracks, $event)"
      />
    </article>

    <aside class="learn__tests">
      <TestsPanel ref="tests" :ex="ex" @hints="ex.hints = $event" @error="error = $event" />
    </aside>
  </div>
</template>
