<script setup lang="ts">
import IconSun from '~icons/lucide/sun'
import IconMoon from '~icons/lucide/moon'
import IconPower from '~icons/lucide/power'
import IconWifiOff from '~icons/lucide/wifi-off'

const { state, connected, stopped, refresh } = useLearn()
const loadError = ref<string | null>(null)

// Theme: follows the OS until the learner picks one; the choice is kept in this browser.
const theme = ref<'light' | 'dark' | null>(null)
function applyTheme(t: 'light' | 'dark' | null) {
  theme.value = t
  if (t) document.documentElement.dataset.theme = t
  else delete document.documentElement.dataset.theme
}
function toggleTheme() {
  const dark = theme.value ? theme.value === 'dark' : matchMedia('(prefers-color-scheme: dark)').matches
  const next = dark ? 'light' : 'dark'
  applyTheme(next)
  try {
    localStorage.setItem('learn-theme', next)
  } catch {}
}

onMounted(async () => {
  try {
    const saved = localStorage.getItem('learn-theme')
    if (saved === 'light' || saved === 'dark') applyTheme(saved)
  } catch {}
  connectEvents()
  try {
    await refresh()
  } catch (e) {
    loadError.value = (e as Error).message
  }
})

async function quit() {
  if (!confirm('Stop learn serve? This page will stop working until you run it again.')) return
  stopped.value = true
  try {
    await api('/api/quit', { method: 'POST' })
  } catch {}
}

const route = useRoute()
const navActive = (prefix: string) => (prefix === '/' ? route.path === '/' : route.path.startsWith(prefix))
</script>

<template>
  <div class="shell">
    <header class="topbar">
      <div class="wrap topbar__inner">
        <NuxtLink to="/" class="brandname">polyglot-learn</NuxtLink>
        <nav class="topnav" aria-label="Main">
          <NuxtLink to="/" :class="{ active: navActive('/') }">Course</NuxtLink>
          <NuxtLink to="/compare/errors" :class="{ active: navActive('/compare') }">Compare</NuxtLink>
        </nav>
        <div class="topbar__actions">
          <span v-if="state?.track" class="tag">track: {{ trackName(state, state.track) }}</span>
          <button class="iconbtn" type="button" :aria-label="'Switch theme'" title="Switch light / dark" @click="toggleTheme">
            <IconSun class="only-dark" /><IconMoon class="only-light" />
          </button>
          <button class="iconbtn" type="button" title="Stop learn serve" aria-label="Quit" @click="quit">
            <IconPower />
          </button>
        </div>
      </div>
    </header>

    <div v-if="stopped" class="notice notice--stopped">
      <div class="wrap">Stopped. Run <code>learn serve</code> again to come back; you can close this tab.</div>
    </div>
    <div v-else-if="!connected" class="notice">
      <div class="wrap"><IconWifiOff /> Lost the connection to <code>learn serve</code>. Is it still running?</div>
    </div>

    <main class="wrap main">
      <p v-if="loadError" class="error-box">{{ loadError }}</p>
      <NuxtPage v-else-if="state" />
      <p v-else class="muted">Loading…</p>
    </main>

    <footer class="footer">
      <div class="wrap footer__inner">
        <span class="tag">Free and open source · MIT code, CC BY-SA lessons</span>
        <a class="tag credit" href="https://rg-digital.dev" target="_blank" rel="noopener">Built by RG Digital</a>
      </div>
    </footer>
  </div>
</template>
