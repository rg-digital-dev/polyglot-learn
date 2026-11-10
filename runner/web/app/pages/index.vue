<script setup lang="ts">
// Home: your track's path, with where to continue. No track yet: the first-run screen.
import IconArrowRight from '~icons/lucide/arrow-right'
import IconBook from '~icons/lucide/book-open'

const { state, paths, loadPath } = useLearn()

watchEffect(() => {
  if (state.value && !state.value.track) navigateTo('/welcome', { replace: true })
})
watch(
  () => state.value?.track,
  (t) => t && loadPath(t).catch(() => {}),
  { immediate: true },
)

const path = computed(() => (state.value?.track ? paths.value[state.value.track] : null))
const items = computed(() => path.value?.stages.flatMap((s) => s.items.map((i) => ({ ...i, stage: s.title }))) ?? [])
const current = computed(() => items.value.find((i) => i.kind === 'exercise' && i.id === path.value?.current) ?? null)
/** New learners get the primer first; it stays optional. */
const firstUnread = computed(() => (state.value?.new ? items.value.find((i) => i.kind === 'reading' && !i.done) : null) ?? null)
const goalLabel = computed(() => state.value?.goals.find((g) => g.id === state.value?.goal)?.label ?? '')
</script>

<template>
  <div v-if="state?.track && path" class="stack-xl">
    <section class="intro">
      <p class="eyebrow">Your path · {{ goalLabel }}</p>
      <h1 class="h-title">{{ path.trackName }}</h1>
      <p v-if="path.for" class="lead">{{ path.for }}</p>
      <div class="home__cards">
        <NuxtLink v-if="current" class="panel pad continue" :to="itemLink(path.track, current)">
          <span class="tag">Continue · {{ current.stage }}</span>
          <span class="h-card-lg">{{ current.title }}</span>
          <span class="muted small">{{ current.concept }}</span>
          <span class="btn btn--primary continue__btn">Open <IconArrowRight class="btn__arrow" /></span>
        </NuxtLink>
        <div v-else class="panel pad continue">
          <span class="tag">Done</span>
          <span class="h-card-lg">You've finished every {{ path.trackName }} exercise.</span>
          <NuxtLink to="/welcome" class="text-link">Pick another track <IconArrowRight /></NuxtLink>
        </div>
        <NuxtLink v-if="firstUnread" class="panel pad continue continue--soft" :to="itemLink(path.track, firstUnread)">
          <span class="tag"><IconBook /> New to this?</span>
          <span class="h-card">Read the {{ firstUnread.stage }} first</span>
          <span class="muted small">Short pages on how each language writes the basics. Optional.</span>
        </NuxtLink>
        <div class="panel pad progress-card">
          <span class="tag">Progress</span>
          <span class="track__progress">
            <span class="bar"><span :style="{ width: `${path.total ? (path.done / path.total) * 100 : 0}%` }" /></span>
            <span class="tag">{{ path.done }}/{{ path.total }}</span>
          </span>
          <span v-if="path.readings" class="muted small">{{ path.read }} of {{ path.readings }} reading pages read</span>
          <NuxtLink to="/welcome" class="text-link">Change track or goal</NuxtLink>
        </div>
      </div>
    </section>

    <section class="panel pad">
      <PathSidebar :track="path.track" full />
    </section>

    <p class="muted small">Your work lives in <code>{{ state.workspace }}</code>, outside the course files.</p>
  </div>
  <p v-else class="muted">Loading your path…</p>
</template>
