<script setup lang="ts">
import IconArrowRight from '~icons/lucide/arrow-right'
import IconArrowLeft from '~icons/lucide/arrow-left'
import IconList from '~icons/lucide/list'

interface Brief {
  id: string
  title: string
}

interface ReadingPage {
  id: string
  title: string
  module: string
  index: number
  count: number
  prev: Brief | null
  next: Brief | null
  track: string | null
  lesson: string | null
  note: string | null
  read: boolean
}

const route = useRoute()
const router = useRouter()
const { state, refresh } = useLearn()
const id = computed(() => route.params.id as string)
const track = computed(() => (route.query.track as string | undefined) ?? state.value?.track ?? null)
const data = ref<ReadingPage | null>(null)
const error = ref<string | null>(null)
const drawer = ref(false)

async function load() {
  error.value = null
  try {
    const q = track.value ? `?track=${track.value}` : ''
    data.value = await api<ReadingPage>(`/api/reading/${id.value}${q}`)
    // Opening a page counts as reading it, as `learn read` does.
    if (!data.value.read) {
      await api(`/api/read/${data.value.id}`, { method: 'POST' })
      await refresh()
    }
  } catch (e) {
    error.value = (e as Error).message
  }
}
watch([id, track], load, { immediate: true })

function pickTrack(t: string) {
  router.replace({ query: { ...route.query, track: t } })
}

/** After the last page: the active track's first exercise, or the course page. */
const after = computed(() => {
  const t = state.value?.track
  const slug = state.value?.tracks.find((x) => x.id === t)?.current
  return t && slug ? `/exercise/${slug}?track=${t}` : '/'
})
</script>

<template>
  <p v-if="error" class="error-box">{{ error }}</p>
  <div v-else-if="data" class="learn learn--two" :class="{ 'learn--drawer': drawer }">
    <aside v-if="data.track" class="learn__path">
      <PathSidebar :track="data.track" :active="data.id" @navigate="drawer = false" />
    </aside>
    <button v-if="drawer" type="button" class="learn__scrim" aria-label="Close the path" @click="drawer = false" />
    <div class="learn__main stack-lg">
      <header class="stack-sm">
        <div class="crumbs">
          <button v-if="data.track" type="button" class="iconlink learn__path-toggle" :aria-expanded="drawer" @click="drawer = !drawer"><IconList /> Path</button>
          <p class="eyebrow">{{ data.module }} · {{ data.index }} of {{ data.count }}</p>
        </div>
        <div class="chips" role="group" aria-label="Your track's note">
          <button
            v-for="t in state?.tracks ?? []"
            :key="t.id"
            type="button"
            class="chip"
            :class="{ 'chip--on': t.id === data.track }"
            :aria-pressed="t.id === data.track"
            @click="pickTrack(t.id)"
          >
            {{ t.name }}
          </button>
        </div>
      </header>

      <section class="panel pad">
        <Markdown :source="data.lesson" />
      </section>

      <section class="panel pad stack-md">
        <template v-if="data.track">
          <p class="tag tag--accent">In {{ trackName(state, data.track) }}</p>
          <Markdown v-if="data.note" :source="data.note" />
          <p v-else class="muted">No {{ trackName(state, data.track) }} note for this page yet.</p>
        </template>
        <p v-else class="muted">Pick a track above to see how it writes this.</p>
        <NuxtLink class="text-link" :to="`/compare/${data.id}`">See every track side by side <IconArrowRight /></NuxtLink>
      </section>

      <nav class="intro__actions" aria-label="Pages">
        <NuxtLink v-if="data.prev" class="btn btn--secondary" :to="{ path: `/read/${data.prev.id}`, query: route.query }">
          <IconArrowLeft class="btn__arrow" /> {{ data.prev.title }}
        </NuxtLink>
        <NuxtLink v-if="data.next" class="btn btn--primary" :to="{ path: `/read/${data.next.id}`, query: route.query }">
          Next: {{ data.next.title }} <IconArrowRight class="btn__arrow" />
        </NuxtLink>
        <NuxtLink v-else class="btn btn--primary" :to="after">
          On to the exercises <IconArrowRight class="btn__arrow" />
        </NuxtLink>
      </nav>
    </div>
  </div>
</template>
