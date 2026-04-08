---
trigger: always_on
description: Prevent invalid @const placement in Svelte 5 loops or conditions.
---

# Svelte 5: `@const` Invalid Placement

**Context**: In Svelte 5, the Svelte compiler enforces stricter valid locations for the `{@const}` tag compared to Svelte 4.

**Problem:** Placing `{@const}` immediately inside a generic HTML element (like `<div>` or `<button>`) results in a compiler error:
`{@const} must be the immediate child of {#snippet}, {#if}, {:else if}, {:else}, {#each}, {:then}, {:catch}, <svelte:fragment>, <svelte:boundary> or <Component>`

**Correct Implementation:**
1. **Lift the `@const` tag up**: If iterating over an `{#each}` block, make sure the `{@const}` tag is an immediate child of the `{#each}` rather than nested under an HTML tag.
2. **Move to `<script>` as `$derived`**: If the `{@const}` is inside a normal HTML element (e.g. `<div>`) to calculate a reactive value based on component state, move the calculation entirely into the `<script>` tag using the `$derived()` rune.

**Code Examples:**

❌ **INCORRECT (Do not generate):**
```svelte
<!-- Error 1: Inside HTML tag nested in an {#each} -->
{#each items as item}
  <div>
    {@const active = item.id === selectedId}
    <p>{active}</p>
  </div>
{/each}

<!-- Error 2: Just sitting inside an HTML tag -->
<div>
    {@const filtered = data.filter(d => d.active)}
    <List items={filtered} />
</div>
```

✅ **CORRECT:**
```svelte
<!-- Fix 1: Immediate child of {#each} -->
{#each items as item}
  {@const active = item.id === selectedId}
  <div>
    <p>{active}</p>
  </div>
{/each}

<!-- Fix 2: Moved to $derived in script block -->
<script lang="ts">
  let data = $props();
  let filtered = $derived(data.filter(d => d.active));
</script>

<div>
    <List items={filtered} />
</div>
```
