<script setup lang="ts">
// One track as a card: what it's for, its family, progress and toolchain.
import IconAlert from '~icons/lucide/triangle-alert'

const props = defineProps<{ track: TrackInfo; total: number; active?: boolean; selected?: boolean; disabled?: boolean }>()
defineEmits<{ pick: [id: string] }>()
const { state } = useLearn()
const family = computed(() => state.value?.families.find((f) => f.id === props.track.family)?.title ?? null)

/** "cargo 1.99.1": the tool and its version number, without the banner text. */
const toolText = (tool: TrackInfo['tool']) => {
  const v = tool.version?.match(/\d+\.\d+(\.\d+)?/)?.[0] ?? tool.path?.match(/\b6\.\d+(\.\d+)?\b/)?.[0]
  return v ? `${tool.label} ${v}` : tool.label
}
</script>

<template>
  <button
    type="button"
    class="track"
    :class="{ 'track--active': active || selected }"
    :aria-pressed="selected"
    :disabled="disabled"
    @click="$emit('pick', track.id)"
  >
    <span class="track__top">
      <span class="h-card">{{ track.name }}</span>
      <span v-if="active" class="pill pill--good">active</span>
      <span v-else-if="family" class="pill">{{ family }}</span>
    </span>
    <span v-if="track.for" class="track__for muted small">{{ track.for }}</span>
    <span class="track__progress">
      <span class="bar"><span :style="{ width: `${total ? (track.done / total) * 100 : 0}%` }" /></span>
      <span class="tag">{{ track.done }}/{{ total }}</span>
    </span>
    <span v-if="track.tool.found" class="track__tool tag" :title="track.tool.path ?? ''">{{ toolText(track.tool) }}</span>
    <span v-else class="track__tool track__tool--missing"><IconAlert /> {{ track.tool.label }} not found. Run <code>learn doctor</code></span>
    <span v-if="track.tool.warning" class="track__warn">{{ track.tool.warning }}</span>
  </button>
</template>
