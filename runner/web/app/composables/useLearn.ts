// Shared client state: the course overview, live check runs (fed by server-sent
// events) and small API helpers. The server owns all logic; the page only shows it.

export interface ToolInfo {
  label: string
  found: boolean
  path: string | null
  version: string | null
  warning: string | null
}

export interface TrackInfo {
  id: string
  name: string
  /** compiled, javascript or scripting (families.toml). */
  family: string | null
  /** What the language is for, in one line. */
  for: string | null
  tool: ToolInfo
  done: number
  current: string | null
}

export interface ExerciseSummary {
  slug: string
  title: string
  concept: string
  done: string[]
}

export interface CourseState {
  title: string
  track: string | null
  from: string | null
  /** learn, switch or compare. */
  goal: string
  /** For the compare goal: the tracks compare views start with. */
  compare: string[]
  backgrounds: { id: string; label: string }[]
  goals: { id: string; label: string }[]
  tracks: TrackInfo[]
  /** `track` is set for a track-only module. */
  /** `track`: a track-only module's track. `tracks`: every track the module is on. */
  modules: {
    id: string
    title: string
    track: string | null
    tracks: string[]
    exercises: ExerciseSummary[]
    readings: ReadingSummary[]
  }[]
  families: { id: string; title: string; about: string; tracks: string[] }[]
  concepts: string[]
  /** Nothing done on the active track and nothing read yet: point at the primer. */
  new: boolean
  workspace: string
}

export interface ReadingSummary {
  id: string
  title: string
  read: boolean
}

/** One exercise on one track (`/api/exercise`). */
export interface Exercise {
  slug: string
  title: string
  concept: string
  summary: string
  module: string | null
  track: string
  trackName: string
  toolFound: boolean
  toolLabel: string
  done: boolean
  /** A track-only exercise: one lesson, no cross-track note or comparison. */
  native: boolean
  /** The tracks this exercise is on, for the track picker. */
  tracks: string[]
  files: { path: string; absolute: string; signatures: string[] }[]
  lesson: string | null
  note: string | null
  links: { kind: string; label: string; title: string; url: string }[]
  hints: string[]
  hintsTotal: number
  cases: { index: number; description: string; lines: string[] }[]
  prev: string | null
  next: string | null
}

/** One track's path: its stages in order, each reading and exercise with its mark. */
export interface PathView {
  track: string
  trackName: string
  for: string | null
  stages: { id: string; title: string; own: boolean; items: PathItem[] }[]
  /** The first unfinished exercise. */
  current: string | null
  done: number
  total: number
  read: number
  readings: number
}

export interface PathItem {
  kind: 'reading' | 'exercise'
  id: string
  title: string
  concept: string | null
  done: boolean
}

/** Where a path item's page is. */
export const itemLink = (track: string, item: { kind: string; id: string }) =>
  item.kind === 'reading' ? `/read/${item.id}?track=${track}` : `/exercise/${item.id}?track=${track}`

export interface CaseResult {
  index: number
  description: string
  passed: boolean | null
  message: string | null
}

export interface Run {
  running: boolean
  total: number
  /** index -> passed, filled in live as results stream in */
  live: Record<number, boolean>
  /** The final result, identical to what `learn check` reports. */
  final: {
    passed: boolean
    timedOut: boolean
    noneRan: boolean
    /** A missing toolchain piece (e.g. a Qt module) when no case ran. */
    setupProblem: string | null
    command: string
    cases: CaseResult[]
    output: string
    next: string | null
  } | null
  error: string | null
}

export const runKey = (track: string, slug: string) => `${track}/${slug}`

export async function api<T>(path: string, init: RequestInit = {}): Promise<T> {
  const headers: Record<string, string> = { 'x-learn': '1' }
  if (init.body) headers['content-type'] = 'application/json'
  const res = await fetch(path, { ...init, headers: { ...headers, ...(init.headers as Record<string, string>) } })
  const body = await res.json().catch(() => ({}))
  if (!res.ok) {
    const err = new Error(body.error ?? `HTTP ${res.status}`) as Error & { status: number; body: unknown }
    err.status = res.status
    err.body = body
    throw err
  }
  return body as T
}

type Listener = (event: Record<string, any>) => void

export function useLearn() {
  const state = useState<CourseState | null>('course', () => null)
  const runs = useState<Record<string, Run>>('runs', () => ({}))
  const connected = useState<boolean>('connected', () => true)
  const stopped = useState<boolean>('stopped', () => false)
  const listeners = useState<Listener[]>('listeners', () => [])
  const paths = useState<Record<string, PathView>>('paths', () => ({}))

  async function refresh() {
    state.value = await api<CourseState>('/api/state')
    // Paths that are on screen follow progress too.
    await Promise.all(Object.keys(paths.value).map((t) => loadPath(t)))
  }

  async function loadPath(track: string) {
    paths.value[track] = await api<PathView>(`/api/path?track=${track}`)
    return paths.value[track]
  }

  function onEvent(fn: Listener) {
    listeners.value.push(fn)
    onBeforeUnmount(() => {
      listeners.value = listeners.value.filter((l) => l !== fn)
    })
  }

  function handle(e: Record<string, any>) {
    const key = e.track && e.slug ? runKey(e.track, e.slug) : ''
    switch (e.type) {
      case 'check-started':
        runs.value[key] = { running: true, total: e.total, live: {}, final: null, error: null }
        break
      case 'case': {
        const run = runs.value[key]
        if (run) run.live[e.index] = e.passed
        break
      }
      case 'check-finished':
        runs.value[key] = { running: false, total: e.cases.length, live: {}, final: e, error: null }
        if (e.passed && paths.value[e.track]) loadPath(e.track).catch(() => {})
        break
      case 'check-error':
        runs.value[key] = { running: false, total: 0, live: {}, final: null, error: e.error }
        break
      case 'progress':
        refresh().catch(() => {})
        break
    }
    for (const l of listeners.value) l(e)
  }

  return { state, runs, paths, connected, stopped, refresh, loadPath, onEvent, handle }
}

/** Open the event stream once, for the whole app. */
export function connectEvents() {
  const { connected, stopped, handle } = useLearn()
  const source = new EventSource('/api/events')
  source.onopen = () => (connected.value = true)
  source.onmessage = (m) => handle(JSON.parse(m.data))
  source.onerror = () => {
    connected.value = false
    if (stopped.value) source.close()
  }
}

export async function startCheck(track: string, slug: string) {
  const { runs } = useLearn()
  runs.value[runKey(track, slug)] = { running: true, total: 0, live: {}, final: null, error: null }
  await api(`/api/check/${slug}?track=${track}`, { method: 'POST' })
}

/** Exercises on a track: the modules its path uses. */
export function exercisesOn(state: CourseState | null, track: string) {
  return state?.modules.filter((m) => m.tracks.includes(track)).reduce((n, m) => n + m.exercises.length, 0) ?? 0
}

export function trackName(state: CourseState | null, id: string | null | undefined) {
  return state?.tracks.find((t) => t.id === id)?.name ?? id ?? ''
}
