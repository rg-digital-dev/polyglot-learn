<script setup lang="ts">
// One concept in several tracks: track chips, a "why they differ" note when
// families are mixed, then each track's note. The compare page and the
// exercise page's Compare tab both use it; the parent owns the selection.
import IconExternal from '~icons/lucide/external-link'
import IconArrowRight from '~icons/lucide/arrow-right'

interface Compare {
  concept: string
  lesson: string | null
  exercise: string | null
  columns: { track: string; name: string; note: string | null; links: { label: string; title: string; url: string }[] }[]
  reading: string | null
  /** Why the shown tracks' families differ, one note per pair. */
  why: { between: [string, string]; text: string }[]
}

const props = defineProps<{ concept: string; selected: string[]; lesson?: boolean }>()
const emit = defineEmits<{ toggle: [id: string] }>()
const { state } = useLearn()
const data = ref<Compare | null>(null)
const error = ref<string | null>(null)
const showLesson = ref(false)

async function load() {
  error.value = null
  try {
    data.value = await api<Compare>(`/api/compare/${props.concept}?tracks=${props.selected.join(',')}`)
  } catch (e) {
    error.value = (e as Error).message
  }
}
watch(() => [props.concept, props.selected.join(',')], load, { immediate: true })

const familyTitle = (id: string) => state.value?.families.find((f) => f.id === id)?.title ?? id
</script>

<template>
  <div class="stack-lg">
    <div class="chips" role="group" aria-label="Tracks to compare">
      <button
        v-for="t in state?.tracks ?? []"
        :key="t.id"
        type="button"
        class="chip"
        :class="{ 'chip--on': selected.includes(t.id) }"
        :aria-pressed="selected.includes(t.id)"
        @click="emit('toggle', t.id)"
      >
        {{ t.name }}
      </button>
    </div>

    <p v-if="error" class="error-box">{{ error }}</p>
    <template v-else-if="data">
      <section v-if="lesson" class="panel pad stack-md">
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
        <p class="tag">Why {{ familyTitle(w.between[0]) }} and {{ familyTitle(w.between[1]) }} differ</p>
        <p>{{ w.text }}</p>
      </aside>
      <p v-if="!data.columns.length" class="muted">Pick a track above. {{ concept }} isn't on the ones selected.</p>
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
