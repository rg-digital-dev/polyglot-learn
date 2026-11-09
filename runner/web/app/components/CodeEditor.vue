<script setup lang="ts">
// Edits the same workspace file your own editor uses. The server refuses a
// save if the file changed on disk since it was loaded, so nothing is ever
// silently overwritten; the page then offers to reload or overwrite. Unsaved
// text is kept in useDrafts, so it survives the editor being recreated.
import { EditorView, basicSetup } from 'codemirror'
import { EditorState } from '@codemirror/state'
import { keymap } from '@codemirror/view'
import { indentWithTab } from '@codemirror/commands'
import { syntaxHighlighting } from '@codemirror/language'
import { classHighlighter } from '@lezer/highlight'
import { languageFor } from '~/composables/highlight'
import IconSave from '~icons/lucide/save'
import IconRefresh from '~icons/lucide/refresh-cw'

const props = defineProps<{ track: string; slug: string; path: string }>()
const emit = defineEmits<{ saved: [] }>()

const host = ref<HTMLElement | null>(null)
let view: EditorView | null = null
const baseMtime = ref(0)
const savedText = ref('')
const dirty = ref(false)
const status = ref('')
const conflict = ref<{ content: string; mtime: number } | null>(null)
const changedOnDisk = ref<{ content: string; mtime: number } | null>(null)

const url = () => `/api/file/${props.slug}?track=${props.track}&path=${encodeURIComponent(props.path)}`
const ext = () => props.path.split('.').pop() ?? ''
const { drafts } = useDrafts()
const key = () => draftKey(props.track, props.slug, props.path)

/** Recompute `dirty` and keep this file's draft in step with it. */
function sync() {
  if (!view) return
  const text = view.state.doc.toString()
  dirty.value = text !== savedText.value
  if (dirty.value) drafts.set(key(), { text, saved: savedText.value, mtime: baseMtime.value })
  else drafts.delete(key())
}

/** Show `text` in the editor; `saved` is the file's content it is compared with. */
function show(text: string, saved: string) {
  savedText.value = saved
  if (view) view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: text } })
  else create(text)
  sync()
}

function create(text: string) {
  view?.destroy()
  const lang = languageFor(ext() === 'rs' ? 'rust' : ext())
  view = new EditorView({
    parent: host.value!,
    state: EditorState.create({
      doc: text,
      extensions: [
        basicSetup,
        keymap.of([indentWithTab, { key: 'Mod-s', preventDefault: true, run: () => (save(), true) }]),
        syntaxHighlighting(classHighlighter),
        ...(lang ? [lang] : []),
        EditorView.updateListener.of((u) => {
          if (u.docChanged) sync()
        }),
      ],
    }),
  })
}

/** Show the file, or your unsaved draft of it. `drop` discards the draft first. */
async function load(drop = false) {
  conflict.value = null
  changedOnDisk.value = null
  if (drop) drafts.delete(key())
  const draft = drafts.get(key())
  if (draft) {
    baseMtime.value = draft.mtime
    show(draft.text, draft.saved)
    status.value = 'Restored your unsaved changes.'
    // If the file changed on disk meanwhile, this offers the usual choices.
    await checkDisk()
    return
  }
  const f = await api<{ content: string; mtime: number }>(url())
  baseMtime.value = f.mtime
  show(f.content, f.content)
  status.value = ''
}

async function save(force = false) {
  if (!view) return
  const content = view.state.doc.toString()
  const base = force && conflict.value ? conflict.value.mtime : baseMtime.value
  try {
    const r = await api<{ mtime: number }>(url(), { method: 'PUT', body: JSON.stringify({ content, base_mtime: base }) })
    baseMtime.value = r.mtime
    savedText.value = content
    sync()
    conflict.value = null
    changedOnDisk.value = null
    status.value = 'Saved. The tests run automatically.'
    emit('saved')
  } catch (e) {
    const err = e as Error & { status?: number; body?: any }
    if (err.status === 409) conflict.value = { content: err.body.content, mtime: err.body.mtime }
    else status.value = err.message
  }
}

/** Called when the file may have changed on disk (watcher event, window focus, timer). */
async function checkDisk() {
  if (!view || conflict.value) return
  try {
    const f = await api<{ content: string; mtime: number }>(url())
    if (f.mtime === baseMtime.value) return
    if (f.content === view.state.doc.toString()) {
      baseMtime.value = f.mtime
      savedText.value = f.content
      sync()
      return
    }
    if (!dirty.value) {
      // Nothing unsaved here: follow the file.
      baseMtime.value = f.mtime
      show(f.content, f.content)
      status.value = 'Reloaded: the file changed on disk.'
    } else {
      changedOnDisk.value = f
    }
  } catch {}
}

const { onEvent } = useLearn()
onEvent((e) => {
  if (e.type === 'file-changed' && e.slug === props.slug && e.track === props.track) checkDisk()
})
let timer: ReturnType<typeof setInterval> | undefined
onMounted(() => {
  load()
  timer = setInterval(checkDisk, 3000)
  window.addEventListener('focus', checkDisk)
})
onBeforeUnmount(() => {
  clearInterval(timer)
  window.removeEventListener('focus', checkDisk)
  view?.destroy()
})
watch(() => [props.path, props.track, props.slug], load)
</script>

<template>
  <div class="editor">
    <div class="editor__bar">
      <span class="tag">{{ path }}<span v-if="dirty"> · unsaved</span></span>
      <span class="editor__status">{{ status }}</span>
      <button class="btn btn--secondary btn--sm" type="button" :disabled="!dirty" @click="save()">
        <IconSave /> Save
      </button>
    </div>
    <div v-if="conflict || changedOnDisk" class="editor__conflict" role="alert">
      <p>
        This file changed on disk (probably in your own editor) since it was loaded here.
        {{ conflict ? 'Your save was not written.' : 'You have unsaved changes here.' }}
      </p>
      <div class="editor__choices">
        <button class="btn btn--secondary btn--sm" type="button" @click="load(true)">
          <IconRefresh /> Reload from disk (drop my changes here)
        </button>
        <button
          class="btn btn--secondary btn--sm"
          type="button"
          @click="conflict = conflict ?? changedOnDisk; save(true)"
        >
          Overwrite the file with my version
        </button>
      </div>
    </div>
    <div ref="host" class="editor__host" />
  </div>
</template>
