---
trigger: always_on
description: Svelte components must wrap imports and state logic inside `<script lang="ts">` tags.
---

# Svelte: Missing Script Tags

**Context**: In a `.svelte` file, all JavaScript or TypeScript logic (imports, state definitions, functions) must reside within a `<script>` tag.

**Problem:** Sometimes during fast scaffolding or copy-pasting, the `<script lang="ts">` wrapper is accidentally omitted. The Svelte compiler will completely fail to parse the file or interpret `import` statements as raw text, breaking the application.

**Correct Implementation:**
1. Always start your `.svelte` logic blocks with `<script lang="ts">`.
2. Ensure you close it with `</script>` before beginning your HTML/Template block.

**Code Examples:**

❌ **INCORRECT (Do not generate):**
```svelte
import { appState } from '$lib/stores/AppState.svelte';
import { Search } from 'lucide-svelte';

<div class="container">
    <Search />
</div>
```

✅ **CORRECT:**
```svelte
<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
    import { Search } from 'lucide-svelte';
</script>

<div class="container">
    <Search />
</div>
```
