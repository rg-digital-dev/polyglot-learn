// Unsaved editor text, kept per file for as long as the page is open. The editor
// is recreated when you switch to the lesson, another file or another exercise;
// it restores the draft from here instead of reloading the file from disk.

export interface Draft {
  /** The unsaved text. */
  text: string
  /** The file's content when the draft started, to tell later whether it's still unsaved. */
  saved: string
  /** The file's mtime when the draft started, so a save still detects changes made on disk. */
  mtime: number
}

const drafts = reactive(new Map<string, Draft>())
let guarded = false

export function draftKey(track: string, slug: string, path: string) {
  return `${track}/${slug}/${path}`
}

export function useDrafts() {
  // Closing or reloading the tab would still lose them: ask first.
  if (import.meta.client && !guarded) {
    guarded = true
    window.addEventListener('beforeunload', (e) => {
      if (drafts.size) e.preventDefault()
    })
  }
  return {
    drafts,
    /** Whether any file of this exercise has unsaved text. */
    hasDraft: (track: string, slug: string) => [...drafts.keys()].some((k) => k.startsWith(`${track}/${slug}/`)),
  }
}
