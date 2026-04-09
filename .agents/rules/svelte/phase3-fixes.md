---
trigger: always_on
description: Svelte 5 & Tauri phase 3 compilation fixes and integration rules to prevent regression errors.
---

# WebQ Integration & Build Checkup Rules (Phase 3 Lessons)

During the Phase 3 (Reconnaissance) and Phase 2 integrations, several critical SvelteKit and TypeScript structural errors were encountered and resolved. Agents MUST obey the following rules globally during development:

## 1. Avoid Undefined Indexing in Svelte 5 Snippets (`typescript-eslint`)
**Problem:** Passing deeply nested recursive object properties (e.g. `items[i]`) into Svelte 5 snippets throws TypeScript `noUncheckedIndexedAccess` errors if the array is potentially undefined or the index could be out of bounds.
**Rule:** Always safely access arrays in `#each` blocks and provide null-coalescing fallbacks.
**Correct Implementation:**
```svelte
{#each items as item, i (i)}
   {@render TreeNode({ node: items[i] ?? item })}
{/each}
```

## 2. Inlining Layout Structures over Missing Components
**Problem:** Blindly importing standard layout components (e.g. `PageHeader.svelte`) across different routes will crash `bun run check` if their path is incorrect or the component isn't ported.
**Rule:** Always verify the existence of imported Layout components. If `PageHeader` is not universally implemented in `$lib/components/layout/`, use inline HTML headers mimicking the obsidian/neon visual styling.

## 3. Strict Paraglide.js i18n Linking
**Problem:** Using unverified translation keys (e.g. `m.start_scan()`) crashes `svelte-check` because Paraglide strictly types exported `messages` objects.
**Rule:** Whenever new modules are added, immediately add EN/TR keys to `messages/en.json` and `messages/tr.json`. Run `npx @inlang/paraglide-js compile --project ./project.inlang` BEFORE running `bun run check` or `bun run build`. Only use keys present in the JSON.

## 4. UI/UX A11y Svelte Warnings (`svelte-ignore`)
**Problem:** Binding `onclick` events to `<div>` elements inside guide overlays generates `a11y_click_events_have_key_events` and `a11y_no_static_element_interactions` warnings.
**Rule:** Explicitly suppress these on modal backgrounds with standard svelte-ignore directives if the semantic structure is intentionally an interactive backdrop overlay:
```svelte
<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="..." onclick={close}> ... </div>
```

## 5. Lucide Svelte Export Mismatches
**Problem:** Guessing icon names (e.g. `CodeRefactoring` vs `Code`) breaks the SvelteKit build.
**Rule:** Ensure you select standard lucide-svelte icons (e.g., `ShieldAlert`, `Globe`, `Search`, `Code`, `Users`). Validate imports by running a build check.
