<script setup lang="ts">
// A track's path: stages that fold open, each reading and exercise with its
// mark. `active` is the item on screen; the stage holding it (or the current
// exercise) starts open. `full` shows every stage open, for the home page.
import IconCheck from '~icons/lucide/check'
import IconArrowRight from '~icons/lucide/arrow-right'
import IconBook from '~icons/lucide/book-open'

const props = defineProps<{ track: string; active?: string | null; full?: boolean }>()
const emit = defineEmits<{ navigate: [] }>()
const { paths, loadPath } = useLearn()
const error = ref<string | null>(null)

watch(
  () => props.track,
  async (t) => {
    error.value = null
    try {
      if (!paths.value[t]) await loadPath(t)
    } catch (e) {
      error.value = (e as Error).message
    }
  },
  { immediate: true },
)

const path = computed(() => paths.value[props.track] ?? null)
const holds = (stage: PathView['stages'][number], id: string | null | undefined) => !!id && stage.items.some((i) => i.id === id)
const startsOpen = (stage: PathView['stages'][number]) =>
  props.full || holds(stage, props.active) || (!props.active && holds(stage, path.value?.current))
const stageDone = (stage: PathView['stages'][number]) => stage.items.filter((i) => i.done).length
</script>

<template>
  <nav class="path" :class="{ 'path--full': full }" :aria-label="`${path?.trackName ?? track} path`">
    <p v-if="error" class="error-box">{{ error }}</p>
    <template v-else-if="path">
      <div v-if="!full" class="path__head">
        <NuxtLink to="/" class="h-card">{{ path.trackName }} path</NuxtLink>
        <span class="tag">{{ path.done }}/{{ path.total }}</span>
      </div>
      <details v-for="stage in path.stages" :key="stage.id" class="path__stage" :open="startsOpen(stage)">
        <summary>
          <span class="path__stage-title">{{ stage.title }}</span>
          <span v-if="stage.own" class="pill">{{ path.trackName }}</span>
          <span class="tag path__count">{{ stageDone(stage) }}/{{ stage.items.length }}</span>
        </summary>
        <ol class="path__items">
          <li v-for="item in stage.items" :key="item.id">
            <NuxtLink
              :to="itemLink(track, item)"
              class="path__item"
              :class="{ 'path__item--active': item.id === active, 'path__item--done': item.done }"
              :aria-current="item.id === active ? 'page' : undefined"
              @click="emit('navigate')"
            >
              <span v-if="item.done" class="check check--sm"><IconCheck /></span>
              <span v-else-if="item.id === path.current" class="dot dot--next"><IconArrowRight /></span>
              <span v-else-if="item.kind === 'reading'" class="path__icon"><IconBook /></span>
              <span v-else class="dot" />
              <span class="path__label">{{ item.title }}</span>
            </NuxtLink>
          </li>
        </ol>
      </details>
    </template>
    <p v-else class="muted small">Loading the path…</p>
  </nav>
</template>
