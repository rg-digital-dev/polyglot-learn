<script setup lang="ts">
// First run (and "change track or goal"): the track, the language you already
// know, and what you're here for. The same three answers as `learn start` and
// `learn init`.
const { state, refresh } = useLearn()
const track = ref<string | null>(state.value?.track ?? null)
const from = ref<string>(state.value?.from ?? 'none')
const goal = ref<string>(state.value?.goal ?? 'learn')
const compare = ref<string[]>(state.value?.compare.length ? [...state.value.compare] : [])
const busy = ref(false)
const error = ref<string | null>(null)

const firstRun = computed(() => !state.value?.track)
const ready = computed(() => !!track.value && (goal.value !== 'compare' || compare.value.length >= 2))

function toggleCompare(id: string) {
  compare.value = compare.value.includes(id) ? compare.value.filter((t) => t !== id) : [...compare.value, id]
}

async function start() {
  if (!track.value) return
  busy.value = true
  error.value = null
  try {
    await api('/api/track', { method: 'POST', body: JSON.stringify({ track: track.value }) })
    await api('/api/from', { method: 'POST', body: JSON.stringify({ from: from.value }) })
    await api('/api/goal', { method: 'POST', body: JSON.stringify({ goal: goal.value, compare: goal.value === 'compare' ? compare.value : [] }) })
    await refresh()
    await navigateTo('/')
  } catch (e) {
    error.value = (e as Error).message
  } finally {
    busy.value = false
  }
}
</script>

<template>
  <div v-if="state" class="stack-xl welcome">
    <section class="intro">
      <p class="eyebrow">{{ firstRun ? 'Welcome' : 'Settings' }}</p>
      <h1 class="h-title">{{ firstRun ? 'Pick a language to learn' : 'Change your track or goal' }}</h1>
      <p class="lead">
        Each track follows its own path: small exercises with tests, then a project that shows what the language is for. You can
        switch tracks at any time; your work on each is kept.
      </p>
    </section>

    <section class="stack-md">
      <h2 class="h-card-lg section-title"><span class="tag">1</span> Track</h2>
      <div class="tracks">
        <TrackCard
          v-for="t in state.tracks"
          :key="t.id"
          :track="t"
          :total="exercisesOn(state, t.id)"
          :selected="t.id === track"
          @pick="track = $event"
        />
      </div>
    </section>

    <section class="stack-md">
      <h2 class="h-card-lg section-title"><span class="tag">2</span> A language you already know</h2>
      <label class="field">
        <span class="tag">Notes add "coming from" sections for it</span>
        <select v-model="from">
          <option v-for="b in state.backgrounds" :key="b.id" :value="b.id">{{ b.label }}</option>
        </select>
      </label>
    </section>

    <section class="stack-md">
      <h2 class="h-card-lg section-title"><span class="tag">3</span> What you're here for</h2>
      <div class="goals" role="radiogroup" aria-label="Goal">
        <label v-for="g in state.goals" :key="g.id" class="goal" :class="{ 'goal--on': goal === g.id }">
          <input v-model="goal" type="radio" name="goal" :value="g.id" />
          <span class="h-card">{{ g.label }}</span>
          <span v-if="g.id === 'learn'" class="muted small">Follow one track's path.</span>
          <span v-else-if="g.id === 'switch'" class="muted small">Notes start with what's different from the language you know.</span>
          <span v-else class="muted small">Compare views start with the tracks you pick.</span>
        </label>
      </div>
      <div v-if="goal === 'compare'" class="stack-sm">
        <p class="tag">Tracks to compare (two or more)</p>
        <div class="chips" role="group" aria-label="Tracks to compare">
          <button
            v-for="t in state.tracks"
            :key="t.id"
            type="button"
            class="chip"
            :class="{ 'chip--on': compare.includes(t.id) }"
            :aria-pressed="compare.includes(t.id)"
            @click="toggleCompare(t.id)"
          >
            {{ t.name }}
          </button>
        </div>
      </div>
      <p v-if="goal === 'switch' && from === 'none'" class="warn">Pick the language you know above, so notes can lead with what's different.</p>
    </section>

    <p v-if="error" class="error-box">{{ error }}</p>
    <div class="intro__actions">
      <button class="btn btn--primary" type="button" :disabled="!ready || busy" @click="start">
        {{ firstRun ? 'Start' : 'Save' }}
      </button>
      <NuxtLink v-if="!firstRun" to="/" class="text-link">Back to your path</NuxtLink>
    </div>
  </div>
</template>
