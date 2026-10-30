// Copy the static build to runner/web/dist, which rust-embed compiles into the
// learn binary. dist/ is committed so `cargo install --locked --path runner` works
// without Node or pnpm.
import { cpSync, rmSync, existsSync, readFileSync, writeFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'

const web = fileURLToPath(new URL('..', import.meta.url))
const from = `${web}.output/public`
const to = `${web}dist`

if (!existsSync(from)) {
  console.error(`No build output in ${from}. Did \`nuxt generate\` fail?`)
  process.exit(1)
}
rmSync(to, { recursive: true, force: true })
cpSync(from, to, { recursive: true })
// The SPA shell is index.html; Nuxt's 200.html/404.html fallbacks aren't needed.
for (const extra of ['200.html', '404.html']) rmSync(`${to}/${extra}`, { force: true })
// Nuxt stamps the build time into the page; zero it so identical sources give
// an identical dist/ (CI compares them).
const index = `${to}/index.html`
writeFileSync(index, readFileSync(index, 'utf8').replace(/("prerenderedAt":1,"serverRendered":2\},)\d+/, '$10'))
console.log(`Copied the web UI to ${to}`)
