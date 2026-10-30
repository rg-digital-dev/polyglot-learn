// Syntax highlighting with the same Lezer parsers the editor uses, so code in
// lessons and code in the editor look identical. Tokens get `tok-*` classes,
// coloured in app.css with the RG Digital editor palette.

import { highlightCode, classHighlighter } from '@lezer/highlight'
import { LanguageSupport, StreamLanguage } from '@codemirror/language'
import { rust } from '@codemirror/lang-rust'
import { go } from '@codemirror/lang-go'
import { php } from '@codemirror/lang-php'
import { javascript } from '@codemirror/lang-javascript'
import { lua } from '@codemirror/legacy-modes/mode/lua'

const luaLanguage = StreamLanguage.define(lua)

/** Editor language support for a fence tag or file extension. */
export function languageFor(tag: string, source = ''): LanguageSupport | null {
  switch (tag.toLowerCase()) {
    case 'rust':
    case 'rs':
      return rust()
    case 'go':
      return go()
    case 'php':
      return php({ plain: !source.trimStart().startsWith('<?php') })
    case 'ts':
    case 'typescript':
      return javascript({ typescript: true })
    case 'js':
    case 'javascript':
    case 'mjs':
    case 'json':
    // QML's JavaScript parts highlight well enough as JS.
    case 'qml':
      return javascript()
    case 'lua':
      return new LanguageSupport(luaLanguage)
    default:
      return null
  }
}

const escape = (s: string) => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;')

export function highlight(code: string, tag: string): string {
  const support = languageFor(tag, code)
  if (!support) return escape(code)
  const parser = support.language.parser
  let html = ''
  highlightCode(
    code,
    parser.parse(code),
    classHighlighter,
    (text, classes) => {
      html += classes ? `<span class="${classes}">${escape(text)}</span>` : escape(text)
    },
    () => {
      html += '\n'
    },
  )
  return html
}
