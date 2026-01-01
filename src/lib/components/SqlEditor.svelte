<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import { Compartment, EditorState } from '@codemirror/state'
  import {
    EditorView,
    highlightActiveLine,
    keymap,
    lineNumbers,
    placeholder as cmPlaceholder,
    type ViewUpdate,
  } from '@codemirror/view'
  import { defaultKeymap, history, historyKeymap } from '@codemirror/commands'
  import { autocompletion, startCompletion } from '@codemirror/autocomplete'
  import { PostgreSQL, sql } from '@codemirror/lang-sql'
  import { linter } from '@codemirror/lint'
  import { HighlightStyle, syntaxHighlighting } from '@codemirror/language'
  import { tags as t } from '@lezer/highlight'

  let {
    value = $bindable(''),
    placeholder = '',
    onRun = null,
    onChange = null,
    completions = [],
    tableName: tableNameProp = '',
    columns: columnsProp = [],
    tableToColumns = {},
    aliasToTable = {},
    warnDoubleQuotedStrings = true,
    // UI sizing
    minRows = 4,
    resizable = true,
    visible = false,
  } = $props<{
    value?: string
    placeholder?: string
    onRun?: (() => void) | null
    onChange?: ((val: string) => void) | null
    completions?: { label: string; type?: string }[]
    tableName?: string | string[]
    columns?: { name?: string; label?: string }[] | string[]
    tableToColumns?: Record<string, string[]>
    aliasToTable?: Record<string, string>
    warnDoubleQuotedStrings?: boolean
    minRows?: number
    resizable?: boolean
    visible?: boolean
  }>()

  let host: HTMLDivElement
  let view: EditorView
  const completionComp = new Compartment()
  const placeholderComp = new Compartment()
  const uppercaseComp = new Compartment()
  const sqlHighlightStyle = HighlightStyle.define([
    {
      tag: [
        t.special(t.name),
        t.processingInstruction,
        t.inserted,
        t.macroName,
        t.constant(t.name),
      ],
      color: 'var(--accent)',
    },
    {
      tag: [t.keyword, t.function, t.standard(t.name)],
      color: 'var(--accent)',
      fontWeight: '600',
    },
    {
      tag: [t.atom, t.bool, t.url, t.contentSeparator, t.labelName],
      color: 'var(--accent)',
    },
    {
      tag: [t.literal, t.number],
      color:
        'light-dark(oklch(from var(--accent) 0.55 c h), oklch(from var(--accent) 0.85 c h))',
    },
    {
      tag: [t.string, t.regexp, t.escape, t.special(t.string)],
      color:
        'light-dark(oklch(from var(--accent) 0.65 c h), oklch(from var(--accent) 0.95 c h))',
      fontStyle: 'italic',
    },
    {
      tag: [t.comment, t.meta],
      color:
        'light-dark(oklch(from var(--accent) 0.45 c h), oklch(from var(--accent) 0.65 c h))',
    },
    {
      tag: [t.variableName, t.typeName, t.namespace, t.className, t.changed],
      color: 'var(--accent)',
      fontStyle: 'italic',
    },
    {
      tag: [t.bracket, t.punctuation, t.separator],
      color: 'var(--ink-2)',
    },
  ])

  function completionSource(context: any) {
    // Check if we are typing after a dot: e.g. "alias.col"
    const dotWord = context.matchBefore(/\b\w+\.\w*/)
    if (dotWord) {
      const parts = dotWord.text.split('.')
      const prefix = parts[0]
      const actualTableName = aliasToTable[prefix] || prefix
      const tableColumns = tableToColumns[actualTableName]

      if (tableColumns) {
        return {
          from: dotWord.from + prefix.length + 1, // Start after the dot
          options: tableColumns.map((c: string) => ({
            label: c,
            type: 'variable' as const,
          })),
        }
      }
    }

    // Default completion for keywords, tables, and columns
    const word = context.matchBefore(/\w+/)
    if (!word && !context.explicit) return null

    // Build options dynamically for context
    const provided = completions.map((c: any) => ({
      ...c,
      label: String((c as any).label ?? ''),
    }))

    const tableOpt = (
      Array.isArray(tableNameProp)
        ? tableNameProp
        : tableNameProp
          ? [tableNameProp]
          : []
    ).map((t) => ({
      label: String(t),
      type: 'class' as const,
    }))

    const colOpts = (Array.isArray(columnsProp) ? columnsProp : []).map(
      (c: any) => ({
        label: String(typeof c === 'string' ? c : (c?.label ?? c?.name ?? '')),
        type: 'variable' as const,
      })
    )

    let opts = [...provided, ...tableOpt, ...colOpts]

    return {
      from: word ? word.from : context.pos,
      options: opts,
    }
  }

  function doubleQuoteStringLinter() {
    return linter((view: EditorView) => {
      if (!warnDoubleQuotedStrings) return []
      const text = view.state.doc.toString()
      const diags: any[] = []
      const re = /\bWHERE\b[^\n]*?=\s*"[^"]*"/gi
      for (const m of text.matchAll(re)) {
        const from = m.index ?? 0
        const to = from + m[0].length
        diags.push({
          from,
          to,
          severity: 'warning',
          message: 'Use single quotes for string literals in Polars SQL',
        })
      }
      return diags
    })
  }

  function buildExtensions() {
    const base = [
      lineNumbers(),
      history(),
      keymap.of([
        ...defaultKeymap,
        ...historyKeymap,
        {
          key: 'Tab',
          run: (view: EditorView) => {
            // Insert 2 spaces instead of moving focus
            view.dispatch({
              changes: {
                from: view.state.selection.main.from,
                to: view.state.selection.main.to,
                insert: '  ',
              },
              selection: {
                anchor: view.state.selection.main.from + 2,
              },
            })
            return true
          },
        },
        {
          key: 'Ctrl-Space',
          run: startCompletion,
        },
        {
          key: 'F5',
          run: () => {
            onRun?.()
            return true
          },
        },
      ]),
      sql({
        dialect: PostgreSQL,
        upperCaseKeywords: true,
      }),
      completionComp.of(
        autocompletion({
          override: [completionSource],
          activateOnTyping: true,
          closeOnBlur: false,
          maxRenderedOptions: 200,
        })
      ),
      highlightActiveLine(),
      syntaxHighlighting(sqlHighlightStyle, { fallback: true }),
      // Auto-uppercase SQL keywords before onChange runs
      uppercaseComp.of(keywordUppercaseExtension()),
      EditorView.updateListener.of((u: ViewUpdate) => {
        if (u.docChanged) {
          const val = u.state.doc.toString()
          value = val
          onChange?.(val)
        }
      }),
      EditorView.editorAttributes.of({ 'aria-label': 'SQL editor' }),
      EditorView.theme({
        '&': {
          fontFamily: 'var(--font-mono), monospace',
          fontSize: 'var(--font-size-mono, 14px)',
        },
        // Make the editor stretch to its container height; container controls min/max and resize
        '.cm-editor': { height: '100%', overflow: 'visible' },
        '.cm-scroller': { overflow: 'visible', color: 'var(--ink-5)' },
        '.cm-tooltip': {
          zIndex: 10000,
          backgroundColor: 'var(--surface-2)',
          border: '1px solid var(--surface-5)',
          borderRadius: '0.25rem',
          overflow: 'hidden',
          color: 'var(--ink-5)',
        },
        '.cm-tooltip-autocomplete': {
          '& > ul': {
            fontFamily: 'var(--font-mono), monospace',
          },
          '& > ul > li[aria-selected]': {
            backgroundColor: 'var(--accent)',
            color: 'white',
          },
        },
        '.cm-content': { caretColor: 'var(--ink-5)' },
      }),
    ] as any[]

    base.push(doubleQuoteStringLinter())
    // Placeholder is configured via a Compartment so it updates when prop changes
    // Use CodeMirror's placeholder only; ensure plain text
    const plain = String(placeholder || '')
    base.push(placeholderComp.of(cmPlaceholder(plain)))
    return base
  }

  onMount(() => {
    const state = EditorState.create({
      doc: value,
      extensions: buildExtensions(),
    })
    view = new EditorView({ state, parent: host })
  })

  onDestroy(() => view?.destroy())

  function handleContainerClick() {
    // Focus editor when clicking anywhere in the container
    if (view) {
      view.focus()
    }
  }

  function handleContainerKeyDown(event: KeyboardEvent) {
    // Only handle keyboard events on the container itself, not from the editor
    if (event.target !== host && !host.contains(event.target as Node)) {
      // Focus editor when pressing Enter or Space on the container
      if ((event.key === 'Enter' || event.key === ' ') && view) {
        event.preventDefault()
        view.focus()
      }
    }
  }

  // Sync external value changes to the editor
  $effect(() => {
    if (!view) return

    const currentValue = value
    const editorValue = view.state.doc.toString()

    if (currentValue !== editorValue) {
      view.dispatch({
        changes: {
          from: 0,
          to: view.state.doc.length,
          insert: currentValue || '',
        },
      })
    }
  })

  // Reconfigure autocompletion when completion-related props change
  $effect(() => {
    if (!view) return

    // Explicitly track these dependencies by reading them
    completions
    tableNameProp
    columnsProp
    tableToColumns
    aliasToTable

    view.dispatch({
      effects: completionComp.reconfigure(
        autocompletion({
          override: [completionSource],
          activateOnTyping: true,
          closeOnBlur: false,
          maxRenderedOptions: 200,
        })
      ),
    })
  })

  // Reconfigure placeholder when prop changes
  $effect(() => {
    if (!view) return

    const currentPlaceholder = placeholder || ''
    view.dispatch({
      effects: placeholderComp.reconfigure(cmPlaceholder(currentPlaceholder)),
    })
  })

  // Focus the editor when it becomes visible
  $effect(() => {
    if (!view) return

    if (visible) {
      // Use requestAnimationFrame to ensure the editor is fully visible before focusing
      requestAnimationFrame(() => {
        view?.focus()
      })
    }
  })

  // Extension: auto-uppercase SQL keywords as user types, excluding strings
  function keywordUppercaseExtension() {
    const KEYWORDS = new Set([
      // Core SQL keywords
      'SELECT',
      'FROM',
      'WHERE',
      'AND',
      'OR',
      'ORDER',
      'BY',
      'LIMIT',
      'OFFSET',
      'GROUP',
      'HAVING',
      'ASC',
      'DESC',
      'AS',
      'IN',
      'IS',
      'NOT',
      'NULL',
      'BETWEEN',
      'LIKE',
      'ILIKE',
      'JOIN',
      'ON',
      'INNER',
      'LEFT',
      'RIGHT',
      'FULL',
      'OUTER',
      'UNION',
      'INTERSECT',
      'EXCEPT',
      'USING',
      'ALL',
      'DISTINCT',
      'CROSS',
      'NATURAL',
      'EXISTS',
      'ANY',
      'SOME',
      'TRUE',
      'FALSE',
      'INTO',
      'VALUES',
      'INSERT',
      'UPDATE',
      'DELETE',
      'SET',
      'CREATE',
      'DROP',
      'ALTER',
      'TABLE',
      'VIEW',
      'INDEX',
      'PRIMARY',
      'FOREIGN',
      'KEY',
      'UNIQUE',
      'CONSTRAINT',
      'REFERENCES',
      'DEFAULT',
      'CHECK',

      // Control flow
      'CASE',
      'WHEN',
      'THEN',
      'ELSE',
      'END',
      'IF',

      // CTEs and subqueries
      'WITH',
      'RECURSIVE',
      'LATERAL',

      // Aggregate functions
      'COUNT',
      'SUM',
      'AVG',
      'MIN',
      'MAX',
      'STDDEV',
      'STDDEV_POP',
      'STDDEV_SAMP',
      'VARIANCE',
      'VAR_POP',
      'VAR_SAMP',
      'EVERY',
      'BOOL_AND',
      'BOOL_OR',
      'STRING_AGG',
      'ARRAY_AGG',

      // String functions
      'REPLACE',
      'SUBSTRING',
      'SUBSTR',
      'UPPER',
      'LOWER',
      'TRIM',
      'LTRIM',
      'RTRIM',
      'BTRIM',
      'CONCAT',
      'CONCAT_WS',
      'LENGTH',
      'CHAR_LENGTH',
      'CHARACTER_LENGTH',
      'POSITION',
      'TRANSLATE',
      'OVERLAY',
      'SPLIT_PART',
      'REPEAT',
      'LPAD',
      'RPAD',
      'INITCAP',
      'REVERSE',
      'ASCII',
      'CHR',
      'MD5',
      'REGEXP_REPLACE',
      'REGEXP_MATCH',
      'REGEXP_MATCHES',
      'REGEXP_SPLIT_TO_ARRAY',
      'REGEXP_SPLIT_TO_TABLE',

      // Date/time functions
      'NOW',
      'CURRENT_DATE',
      'CURRENT_TIME',
      'CURRENT_TIMESTAMP',
      'DATE_TRUNC',
      'DATE_PART',
      'EXTRACT',
      'AGE',
      'TO_CHAR',
      'TO_DATE',
      'TO_TIMESTAMP',
      'INTERVAL',
      'EPOCH',
      'YEAR',
      'MONTH',
      'DAY',
      'HOUR',
      'MINUTE',
      'SECOND',

      // Math functions
      'ABS',
      'CEIL',
      'CEILING',
      'FLOOR',
      'ROUND',
      'TRUNC',
      'TRUNCATE',
      'POWER',
      'SQRT',
      'EXP',
      'LN',
      'LOG',
      'LOG10',
      'MOD',
      'SIGN',
      'RANDOM',
      'GREATEST',
      'LEAST',

      // Type conversion and casting
      'CAST',
      'CONVERT',
      'COALESCE',
      'NULLIF',

      // Window functions
      'OVER',
      'PARTITION',
      'ROW_NUMBER',
      'RANK',
      'DENSE_RANK',
      'PERCENT_RANK',
      'CUME_DIST',
      'LAG',
      'LEAD',
      'FIRST_VALUE',
      'LAST_VALUE',
      'NTH_VALUE',
      'NTILE',
      'ROWS',
      'RANGE',
      'UNBOUNDED',
      'PRECEDING',
      'FOLLOWING',
      'CURRENT',
      'ROW',

      // Data types
      'INTEGER',
      'INT',
      'BIGINT',
      'SMALLINT',
      'NUMERIC',
      'DECIMAL',
      'REAL',
      'DOUBLE',
      'PRECISION',
      'FLOAT',
      'TEXT',
      'VARCHAR',
      'CHAR',
      'CHARACTER',
      'VARYING',
      'BOOLEAN',
      'BOOL',
      'DATE',
      'TIME',
      'TIMESTAMP',
      'TIMESTAMPTZ',
      'INTERVAL',
      'JSON',
      'JSONB',
      'ARRAY',
      'UUID',

      // Conditional and comparison
      'FILTER',
      'SIMILAR',
      'TO',
    ])

    return EditorState.transactionFilter.of((tr) => {
      // Only apply when the user is typing (not on programmatic changes or deletes that don't add text)
      if (!tr.docChanged || !tr.isUserEvent('input.type')) return tr
      const changes: { from: number; to: number; insert: string }[] = []

      tr.changes.iterChangedRanges((fromA, toA, fromB, toB) => {
        // Only scan if something was added (fromB < toB means something was inserted)
        // If fromB === toB, it's a pure deletion, so we skip to avoid feedback loops.
        if (fromB >= toB) return

        // Expand to the line to re-scan safely
        const line = tr.newDoc.lineAt(fromB)
        const toLine = tr.newDoc.lineAt(toB)
        const scanFrom = line.from
        const scanTo = toLine.to
        const text = tr.newDoc.sliceString(scanFrom, scanTo)

        // Simple lexer: skip content inside single quotes
        let inString = false
        let tokenStart = -1
        const flush = (end: number) => {
          if (tokenStart === -1) return
          const token = text.slice(tokenStart, end)
          const upper = token.toUpperCase()
          if (KEYWORDS.has(upper) && token !== upper) {
            const absFrom = scanFrom + tokenStart
            const absTo = scanFrom + end
            changes.push({ from: absFrom, to: absTo, insert: upper })
          }
          tokenStart = -1
        }

        for (let i = 0; i < text.length; i++) {
          const ch = text[i]
          if (ch === "'") {
            inString = !inString
            // boundary: flush any token before entering/leaving a string
            flush(i)
            continue
          }
          if (inString) continue
          if (/[A-Za-z0-9_]/.test(ch)) {
            if (tokenStart === -1) tokenStart = i
          } else {
            flush(i)
          }
        }
        flush(text.length)
      })

      if (changes.length === 0) return tr
      return [tr, { changes, sequential: true }]
    })
  }
</script>

<div
  class="sql-editor-container"
  class:resizable
  style={`min-height: ${Math.max(1, Math.floor(minRows)) * 1.5}rem;`}
  onclick={handleContainerClick}
  onkeydown={handleContainerKeyDown}
  role="textbox"
  tabindex="-1"
>
  <div bind:this={host} class="editor-host"></div>
</div>

<style>
  .sql-editor-container {
    width: 100%;
    background: var(--surface-2);
    border: 1px solid var(--surface-4);
    border-radius: 0.25rem;
    padding: 0.25rem;
    max-height: 24rem;
    overflow: auto;
    display: flex;
    flex-direction: column;
  }

  .sql-editor-container.resizable {
    resize: vertical;
  }

  .editor-host {
    flex: 1;
    min-height: 0;
  }
</style>
