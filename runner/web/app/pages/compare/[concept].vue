<script setup lang="ts">
import IconExternal from '~icons/lucide/external-link'
import IconArrowRight from '~icons/lucide/arrow-right'

interface Compare {
  concept: string
  lesson: string | null
  exercise: string | null
  /** The page's title when the concept is a reading. */
  reading: string | null
  columns: { track: string; name: string; note: string | null; links: { label: string; title: string; url: string }[] }[]
  /** Why the shown tracks' families differ, one note per pair. */
  why: { between: [string, string]; text: string }[]
}

const route = useRoute()
const router = useRouter()
const { state } = useLearn()
const concept = computed(() => route.params.concept as string)
const selected = computed<string[]>(() => {
  const q = route.query.tracks as string | undefined
  if (q) return q.split(',').filter(Boolean)
  if (state.value?.goal === 'compare' && state.value.compare.length >= 2) return state.value.compare
  // Start with the active track's family; other tracks are one click away.
  const family = state.value?.tracks.find((t) => t.id === state.value?.track)?.family
  const members = state.value?.families.find((f) => f.id === family)?.tracks
  return members?.length ? members : ['rust', 'go', 'lua', 'typescript']
})
const data = ref<Compare | null>(null)
const error = ref<string | null>(null)
const showLesson = ref(false)

async function load() {
  error.value = null
  try {
    data.value = await api<Compare>(`/api/compare/${concept.value}?tracks=${selected.value.join(',')}`)
  } catch (e) {
    error.value = (e as Error).message
  }
}
watch([concept, selected], load, { immediate: true })

function toggle(id: string) {
  const next = selected.value.includes(id) ? selected.value.filter((t) => t !== id) : [...selected.value, id]
  if (next.length) router.replace({ query: { tracks: next.join(',') } })
}
</script>

<template>
  <div class="stack-lg">
    <header class="stack-sm">
      <p class="eyebrow">Compare</p>
      <h1 class="h-title">{{ data?.reading ?? concept }}, side by side</h1>
      <div class="chips" role="group" aria-label="Concept">
        <NuxtLink v-for="c in state?.concepts ?? []" :key="c" class="chip" :class="{ 'chip--on': c === concept }" :to="{ path: `/compare/${c}`, query: route.query }">
          {{ c }}
        </NuxtLink>
      </div>
      <div class="chips" role="group" aria-label="Tracks">
        <button
          v-for="t in state?.tracks ?? []"
          :key="t.id"
          type="button"
          class="chip"
          :class="{ 'chip--on': selected.includes(t.id) }"
          :aria-pressed="selected.includes(t.id)"
          @click="toggle(t.id)"
        >
          {{ t.name }}
        </button>
      </div>
    </header>

    <p v-if="error" class="error-box">{{ error }}</p>
    <template v-else-if="data">
      <section class="panel pad stack-md">
        <div class="compare__intro">
          <button type="button" class="disclosure" :aria-expanded="showLesson" @click="showLesson = !showLesson">
            {{ showLesson ? 'Hide' : 'Show' }} the {{ data.reading ? 'page' : 'lesson' }} for {{ data.reading ?? concept }}
          </button>
          <NuxtLink v-if="data.exercise" class="text-link" :to="`/exercise/${data.exercise}?track=${state?.track ?? 'typescript'}`">
            Try the exercise <IconArrowRight />
          </NuxtLink>
        </div>
        <Markdown v-if="showLesson" :source="data.lesson" />
      </section>
      <aside v-for="w in data.why" :key="w.between.join('-')" class="panel pad compare__why">
        <p class="tag">Why {{ w.between.join(' and ') }} differ</p>
        <p>{{ w.text }}</p>
      </aside>
      <div class="compare">
        <section v-for="col in data.columns" :key="col.track" class="panel pad compare__col">
          <p class="tag tag--accent">{{ col.name }}</p>
          <Markdown v-if="col.note" :source="col.note" />
          <p v-else class="muted">No {{ col.name }} note for {{ concept }} yet.</p>
          <ul v-if="col.links.length" class="links links--compact">
            <li v-for="l in col.links" :key="l.url">
              <a :href="l.url" target="_blank" rel="noopener noreferrer" class="linkrow">
                <span class="tag">{{ l.label }}</span><span>{{ l.title }}</span><IconExternal class="linkrow__icon" />
              </a>
            </li>
          </ul>
        </section>
      </div>
    </template>
  </div>
</template>
