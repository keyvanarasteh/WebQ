---
trigger: always_on
description: Prevent manual paths array in SvelteKit tsconfig.json that overrides generated modules like $app/paths.
---

# SvelteKit: tsconfig.json `paths` Override Bug

**Context**: In a SvelteKit application, SvelteKit automatically generates essential types and module resolution alias definitions (e.g. `$app/paths`, `$lib`, `$env/static/public`) inside `.svelte-kit/tsconfig.json`.

**Problem:** If you manually specify the `paths` object in the root `tsconfig.json`, TypeScript will **override and ignore** SvelteKit's auto-generated paths from `.svelte-kit/tsconfig.json`. This leads to cryptic compiler errors such as:
`Cannot find module '$app/paths' or its corresponding type declarations. (ts)`

**Correct Implementation:**
1. The root `tsconfig.json` MUST contain `"extends": "./.svelte-kit/tsconfig.json"`.
2. You MUST NOT define a root `paths` (or `baseUrl`) in the `tsconfig.json` directly.
3. Instead, define any custom aliases exclusively in `svelte.config.js` via `kit.alias`. SvelteKit will automatically read them and append them into the generated `.svelte-kit/tsconfig.json` to ensure everything merges smoothly.

**Code Examples:**

❌ **INCORRECT (Do not generate):**
```json
// tsconfig.json
{
  "compilerOptions": {
    "paths": {
      "$lib": ["./src/lib"],
      "$components": ["./src/lib/components"]
    }
  }
}
```

✅ **CORRECT:**
```json
// tsconfig.json
{
  "extends": "./.svelte-kit/tsconfig.json",
  "compilerOptions": {
    // Other options except paths/baseUrl
  }
}
```

```javascript
// svelte.config.js
export default {
  kit: {
    alias: {
      $components: "src/lib/components",
      $stores: "src/lib/stores"
    }
  }
};
```
