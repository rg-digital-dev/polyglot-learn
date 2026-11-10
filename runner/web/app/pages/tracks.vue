<script setup lang="ts">
// "All tracks": every track's progress and every exercise against every track.
const { state, refresh } = useLearn()
const busy = ref(false)

const totalFor = (track: string) => exercisesOn(state.value, track)

async function pickTrack(id: string) {
  busy.value = true
  try {
    await api('/api/track', { method: 'POST', body: JSON.stringify({ track: id }) })
    await refresh()
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <div v-if="state" class="stack-xl">
    <section class="intro">
      <p class="eyebrow">All tracks</p>
      <h1 class="h-title">Every track, side by side</h1>
      <p class="lead">For learning several languages at once. Each track follows its own path; this page shows every exercise against every track it's on.</p>
    </section>

    <section>
      <h2 class="h-card-lg section-title">Tracks</h2>
      <div class="tracks">
        <TrackCard
          v-for="t in state.tracks"
          :key="t.id"
          :track="t"
          :total="totalFor(t.id)"
          :active="t.id === state.track"
          :disabled="busy"
          @pick="pickTrack"
        />
      </div>
    </section>

    <ExerciseGrid />

    <p class="muted small">Your work lives in <code>{{ state.workspace }}</code>, outside the course files.</p>
  </div>
</template>
