<script lang="ts">
    import {onDestroy, onMount} from 'svelte'
    import {Compartment, EditorState} from '@codemirror/state'
    import {
        drawSelection,
        EditorView,
        highlightActiveLine,
        keymap,
        lineNumbers,
        placeholder as cmPlaceholder,
        rectangularSelection,
        type ViewUpdate,
    } from '@codemirror/view'
    import {
        copyLineDown,
        copyLineUp,
        defaultKeymap,
        history,
        historyKeymap,
        indentLess,
        indentMore,
        moveLineDown,
        moveLineUp,
    } from '@codemirror/commands'
    import {autocompletion, startCompletion} from '@codemirror/autocomplete'
    import {search, searchKeymap, selectNextOccurrence, selectSelectionMatches} from '@codemirror/search'
    import {PostgreSQL, sql} from '@codemirror/lang-sql'
    import {linter} from '@codemirror/lint'
    import {HighlightStyle, indentUnit, syntaxHighlighting} from '@codemirror/language'
    import {tags as t} from '@lezer/highlight'

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
            tag: [t.keyword, t.function(t.name), t.standard(t.name)],
            color: 'var(--accent)',
            fontWeight: '600',
            textTransform: 'uppercase',
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

    const tableNames = $derived.by(() => {
        if (Array.isArray(tableNameProp)) return tableNameProp
        return tableNameProp ? [tableNameProp] : []
    })
    const columnNames = $derived(columnsProp.map((c: any) => typeof c === 'string' ? c : (c.name || c.label || '')))

    function buildExtensions() {
        const base = [
            lineNumbers(),
            history(),
            drawSelection(),
            EditorState.allowMultipleSelections.of(true),
            rectangularSelection(),
            search({top: true}),
            indentUnit.of('  '),
            EditorView.clickAddsSelectionRange.of((event) => event.altKey),
            keymap.of([
                ...defaultKeymap,
                ...historyKeymap,
                ...searchKeymap,
                {
                    key: 'Mod-d',
                    run: selectNextOccurrence,
                    preventDefault: true,
                },
                {
                    key: 'Shift-Mod-l',
                    run: selectSelectionMatches,
                    preventDefault: true,
                },
                {
                    key: 'Alt-ArrowUp',
                    run: moveLineUp,
                },
                {
                    key: 'Alt-ArrowDown',
                    run: moveLineDown,
                },
                {
                    key: 'Shift-Alt-ArrowUp',
                    run: copyLineUp,
                },
                {
                    key: 'Shift-Alt-ArrowDown',
                    run: copyLineDown,
                },
                {
                    key: 'Mod-Enter',
                    run: () => {
                        onRun?.()
                        return true
                    },
                },
                {
                    key: 'Tab',
                    run: indentMore,
                    shift: indentLess,
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
            syntaxHighlighting(sqlHighlightStyle, {fallback: true}),
            EditorView.updateListener.of((u: ViewUpdate) => {
                if (u.docChanged) {
                    const val = u.state.doc.toString()
                    value = val
                    onChange?.(val)
                }
            }),
            EditorView.editorAttributes.of({'aria-label': 'SQL editor'}),
            EditorView.theme({
                '&': {
                    fontFamily: 'var(--font-mono), monospace',
                    fontSize: 'var(--font-size-mono, 14px)',
                },
                // Make the editor stretch to its container height; container controls min/max and resize
                '.cm-editor': {height: '100%', overflow: 'visible'},
                '.cm-scroller': {overflow: 'visible', color: 'var(--ink-5)'},
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
                '.cm-content': {caretColor: 'var(--ink-5)'},
                '.cm-selectionBackground, .cm-content ::selection': {
                    backgroundColor: 'var(--accent) !important',
                    color: 'black !important',
                },
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
        view = new EditorView({state, parent: host})
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

    // Update completions, placeholder and uppercase extension when props change
    $effect(() => {
        if (!view) return

        // Explicitly track these dependencies by reading them
        completions
        tableNameProp
        columnsProp
        tableToColumns
        aliasToTable

        const plain = String(placeholder || '')
        view.dispatch({
            effects: [
                completionComp.reconfigure(
                    autocompletion({
                        override: [completionSource],
                        activateOnTyping: true,
                        closeOnBlur: false,
                        maxRenderedOptions: 200,
                    })
                ),
                placeholderComp.reconfigure(cmPlaceholder(plain)),
            ]
        })
    })
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
