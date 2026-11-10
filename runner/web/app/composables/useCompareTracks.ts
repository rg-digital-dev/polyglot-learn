// Which tracks a compare view starts with, the same rule as `learn compare`:
// the tracks chosen for the compare goal, else the active track's family, else
// a default set. Toggling keeps at least one track selected.

export function defaultCompareTracks(state: CourseState | null): string[] {
  if (state?.goal === 'compare' && state.compare.length >= 2) return state.compare
  const family = state?.tracks.find((t) => t.id === state?.track)?.family
  const members = state?.families.find((f) => f.id === family)?.tracks
  return members?.length ? members : ['rust', 'go', 'lua', 'typescript']
}

export function toggled(list: string[], id: string): string[] {
  const next = list.includes(id) ? list.filter((t) => t !== id) : [...list, id]
  return next.length ? next : list
}
