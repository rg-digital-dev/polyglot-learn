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
  backgrounds: { id: string; label: string }[]
  tracks: TrackInfo[]
  /** `track` is set for a track-only module. */
  modules: { id: string; title: string; track: string | null; exercises: ExerciseSummary[] }[]
  concepts: string[]
  workspace: string
}

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

  async function refresh() {
    state.value = await api<CourseState>('/api/state')
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

  return { state, runs, connected, stopped, refresh, onEvent, handle }
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

export function trackName(state: CourseState | null, id: string | null | undefined) {
  return state?.tracks.find((t) => t.id === id)?.name ?? id ?? ''
}
