<script setup lang="ts">
import IconCheck from '~icons/lucide/check'
import IconArrowRight from '~icons/lucide/arrow-right'
import IconAlert from '~icons/lucide/triangle-alert'
import type { CourseState } from '~/composables/useLearn'

const { state, refresh } = useLearn()
/** Exercises on a track: the shared modules plus that track's own. */
const totalFor = (track: string) =>
  state.value?.modules.filter((m) => !m.track || m.track === track).reduce((n, m) => n + m.exercises.length, 0) ?? 0
const tracksOf = (m: CourseState['modules'][number]) => state.value?.tracks.filter((t) => !m.track || t.id === m.track) ?? []
const busy = ref(false)

async function pickTrack(id: string) {
  busy.value = true
  try {
    await api('/api/track', { method: 'POST', body: JSON.stringify({ track: id }) })
    await refresh()
  } finally {
    busy.value = false
  }
}

async function pickFrom(ev: Event) {
  const from = (ev.target as HTMLSelectElement).value
  await api('/api/from', { method: 'POST', body: JSON.stringify({ from }) })
  await refresh()
}

/** "cargo 1.99.1": the tool and its version number, without the banner text. */
const toolText = (tool: { label: string; version: string | null; path: string | null }) => {
  const v = tool.version?.match(/\d+\.\d+(\.\d+)?/)?.[0] ?? tool.path?.match(/\b6\.\d+(\.\d+)?\b/)?.[0]
  return v ? `${tool.label} ${v}` : tool.label
}
const isCurrent =(track: string, slug: string) => state.value?.tracks.find((t) => t.id === track)?.current === slug
const active = computed(() => state.value?.tracks.find((t) => t.id === state.value?.track))
const resumeSlug = computed(() => active.value?.current ?? null)
</script>

<template>
  <div v-if="state" class="stack-xl">
    <section class="intro">
      <p class="eyebrow">Learn a language by fixing small files</p>
      <h1 class="h-title">Pick a track, then work through the exercises</h1>
      <p class="lead">
        Each exercise is one function, with tests generated for every language. Edit the file in your own editor (or here); the
        tests re-run when you save.
      </p>
      <div class="intro__actions">
        <NuxtLink v-if="state.track && resumeSlug" class="btn btn--primary" :to="`/exercise/${resumeSlug}?track=${state.track}`">
          Continue in {{ trackName(state, state.track) }} <IconArrowRight class="btn__arrow" />
        </NuxtLink>
        <label class="field">
          <span class="tag">Notes compare against</span>
          <select :value="state.from ?? 'none'" @change="pickFrom">
            <option v-for="b in state.backgrounds" :key="b.id" :value="b.id">{{ b.label }}</option>
          </select>
        </label>
      </div>
    </section>

    <section>
      <h2 class="h-card-lg section-title">Tracks</h2>
      <div class="tracks">
        <button
          v-for="t in state.tracks"
          :key="t.id"
          type="button"
          class="track"
          :class="{ 'track--active': t.id === state.track }"
          :disabled="busy"
          @click="pickTrack(t.id)"
        >
          <span class="track__top">
            <span class="h-card">{{ t.name }}</span>
            <span v-if="t.id === state.track" class="pill pill--good">active</span>
          </span>
          <span class="track__progress">
            <span class="bar"><span :style="{ width: `${(t.done / totalFor(t.id)) * 100}%` }" /></span>
            <span class="tag">{{ t.done }}/{{ totalFor(t.id) }}</span>
          </span>
          <span v-if="t.tool.found" class="track__tool tag" :title="t.tool.path ?? ''">{{ toolText(t.tool) }}</span>
          <span v-else class="track__tool track__tool--missing">
            <IconAlert /> {{ t.tool.label }} not found. Run <code>learn doctor</code>
          </span>
          <span v-if="t.tool.warning" class="track__warn">{{ t.tool.warning }}</span>
        </button>
      </div>
    </section>

    <section v-for="m in state.modules" :key="m.id">
      <h2 class="h-card-lg section-title">
        {{ m.title }}
        <span v-if="m.track" class="pill">{{ trackName(state, m.track) }} only</span>
      </h2>
      <div class="panel grid-wrap">
        <table class="grid">
          <thead>
            <tr>
              <th scope="col" class="tag">Exercise</th>
              <th scope="col" class="tag">Concept</th>
              <th v-for="t in tracksOf(m)" :key="t.id" scope="col" class="tag grid__track" :class="{ active: t.id === state.track }">
                {{ t.id === 'typescript' ? 'ts' : t.id }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="e in m.exercises" :key="e.slug">
              <th scope="row">
                <NuxtLink :to="`/exercise/${e.slug}?track=${m.track ?? state.track ?? 'typescript'}`" class="grid__title">{{ e.title }}</NuxtLink>
              </th>
              <td>
                <span v-if="m.track" class="chip chip--static">{{ e.concept }}</span>
                <NuxtLink v-else class="chip" :to="`/compare/${e.concept}`">{{ e.concept }}</NuxtLink>
              </td>
              <td v-for="t in tracksOf(m)" :key="t.id" class="grid__cell">
                <NuxtLink
                  :to="`/exercise/${e.slug}?track=${t.id}`"
                  :aria-label="`${e.title} in ${t.name}: ${e.done.includes(t.id) ? 'done' : isCurrent(t.id, e.slug) ? 'up next' : 'not done'}`"
                >
                  <span v-if="e.done.includes(t.id)" class="check check--sm"><IconCheck /></span>
                  <span v-else-if="t.id === state.track && isCurrent(t.id, e.slug)" class="dot dot--next"><IconArrowRight /></span>
                  <span v-else class="dot" />
                </NuxtLink>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </section>

    <p class="muted small">Your work lives in <code>{{ state.workspace }}</code>, outside the course files.</p>
  </div>
</template>
