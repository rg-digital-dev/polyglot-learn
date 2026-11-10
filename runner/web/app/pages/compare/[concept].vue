<script setup lang="ts">
// One concept, several tracks. Starts with your compare goal's tracks or your
// track's family (as `learn compare` does); the URL keeps your selection.
const route = useRoute()
const router = useRouter()
const { state } = useLearn()
const concept = computed(() => route.params.concept as string)
const selected = computed<string[]>(() => {
  const q = route.query.tracks as string | undefined
  return q ? q.split(',').filter(Boolean) : defaultCompareTracks(state.value)
})
const title = computed(() => state.value?.modules.flatMap((m) => m.readings).find((r) => r.id === concept.value)?.title ?? concept.value)

function toggle(id: string) {
  router.replace({ query: { tracks: toggled(selected.value, id).join(',') } })
}
</script>

<template>
  <div class="stack-lg">
    <header class="stack-sm">
      <p class="eyebrow">Compare</p>
      <h1 class="h-title">{{ title }}, side by side</h1>
      <div class="chips" role="group" aria-label="Concept">
        <NuxtLink v-for="c in state?.concepts ?? []" :key="c" class="chip" :class="{ 'chip--on': c === concept }" :to="{ path: `/compare/${c}`, query: route.query }">
          {{ c }}
        </NuxtLink>
      </div>
    </header>
    <ComparePanel :concept="concept" :selected="selected" lesson @toggle="toggle" />
  </div>
</template>
