<script setup lang="ts">
// Every module with its readings and its exercises against every track it's on:
// the "All tracks" overview, for people learning several languages.
import IconCheck from '~icons/lucide/check'
import IconArrowRight from '~icons/lucide/arrow-right'
import type { CourseState } from '~/composables/useLearn'

const { state } = useLearn()
type Module = CourseState['modules'][number]
const tracksOf = (m: Module) => state.value?.tracks.filter((t) => m.tracks.includes(t.id)) ?? []
/** Tracks a shared module leaves out, for its "not on" label. */
const missingFrom = (m: Module) => (m.track ? [] : (state.value?.tracks.filter((t) => !m.tracks.includes(t.id)) ?? []))
/** Where an exercise's title links to: the active track if the module is on it. */
const linkTrack = (m: Module) => m.track ?? (state.value?.track && m.tracks.includes(state.value.track) ? state.value.track : m.tracks[0])
const isCurrent = (track: string, slug: string) => state.value?.tracks.find((t) => t.id === track)?.current === slug
const short = (id: string) => (id === 'typescript' ? 'ts' : id === 'python' ? 'py' : id)
</script>

<template>
  <div v-if="state" class="stack-xl">
    <section v-for="m in state.modules" :key="m.id">
      <h2 class="h-card-lg section-title">
        {{ m.title }}
        <span v-if="m.track" class="pill">{{ trackName(state, m.track) }} only</span>
        <span v-else-if="missingFrom(m).length" class="pill">not on {{ missingFrom(m).map((t) => t.name).join(', ') }}</span>
      </h2>
      <div v-if="m.readings.length" class="panel grid-wrap">
        <table class="grid">
          <thead>
            <tr>
              <th scope="col" class="tag">Reading</th>
              <th scope="col" class="tag grid__track">read</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="r in m.readings" :key="r.id">
              <th scope="row">
                <NuxtLink :to="`/read/${r.id}`" class="grid__title">{{ r.title }}</NuxtLink>
              </th>
              <td class="grid__cell">
                <NuxtLink :to="`/read/${r.id}`" :aria-label="`${r.title}: ${r.read ? 'read' : 'not read yet'}`">
                  <span v-if="r.read" class="check check--sm"><IconCheck /></span>
                  <span v-else class="dot" />
                </NuxtLink>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-if="m.exercises.length" class="panel grid-wrap">
        <table class="grid">
          <thead>
            <tr>
              <th scope="col" class="tag">Exercise</th>
              <th scope="col" class="tag">Concept</th>
              <th v-for="t in tracksOf(m)" :key="t.id" scope="col" class="tag grid__track" :class="{ active: t.id === state.track }">
                {{ short(t.id) }}
              </th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="e in m.exercises" :key="e.slug">
              <th scope="row">
                <NuxtLink :to="`/exercise/${e.slug}?track=${linkTrack(m)}`" class="grid__title">{{ e.title }}</NuxtLink>
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
  </div>
</template>
