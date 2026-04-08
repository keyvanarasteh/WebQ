<?xml version="1.0" encoding="UTF-8"?>
<rules project="WebQ" version="1.0.0">

  <!-- ═══════════════════════════════════════════════════════════
       PROJECT IDENTITY & GITHUB DEVOPS
  ═══════════════════════════════════════════════════════════ -->
  <section id="project-identity" source="github-config">
    <rule id="PI-01" severity="required">Project name is WebQ. A high-performance web intelligence and cybersecurity platform built with Rust, Svelte 5, and Tauri, powered by the web-analyzer crate.</rule>
    <rule id="PI-02" severity="required">Default branch is `main`. Never push to or interact with `master`.</rule>
    <rule id="PI-03" severity="required">Branch Protection is ENABLED for `main`. Direct pushes are restricted. You MUST create a branch, submit a Pull Request, and pass status validations before merge.</rule>
    <rule id="PI-04" severity="required">GitHub PRs require "Linear History". Do not pollute the commit tree with ugly merge commits. Squash & Merge or Rebase patterns are enforced.</rule>
    <rule id="PI-05" severity="info">GitHub Features enabled: Discussions, Wikis, Projects, and Standardized Issue Templates (Bug & Feature). Suggest users to open standard issues.</rule>
  </section>

  <!-- ═══════════════════════════════════════════════════════════
       BACKEND: RUST, TAURI V2 & WEB-ANALYZER
  ═══════════════════════════════════════════════════════════ -->
  <section id="backend-architecture">
    <rule id="BE-01" severity="required">Backend is strictly Rust using Tauri v2. Memory-safety is paramount; completely avoid `unsafe` blocks.</rule>
    <rule id="BE-02" severity="required">All security scans (WHOIS, DNS, Nmap, API analysis) MUST utilize the `web-analyzer` crate features asynchronously via `tokio`.</rule>
    <rule id="BE-03" severity="required">Scan states and results must never block the Tauri event loop or main thread.</rule>
    <rule id="BE-04" severity="required">All Tauri commands must return strongly typed `Result<T, AppError>` utilizing Serde for pristine frontend transmission.</rule>
  </section>

  <!-- ═══════════════════════════════════════════════════════════
       FRONTEND: SVELTE 5
  ═══════════════════════════════════════════════════════════ -->
  <section id="frontend-architecture" source="svelte-runes">
    <rule id="FE-01" severity="required">Svelte 5 Runes ($state, $derived, $effect) are strictly enforced. Never use legacy Svelte 4 reactivity syntax (`let x = 0`, `$: x = y`).</rule>
    <rule id="FE-02" severity="required">TailwindCSS must be used for layout and animations. Aesthetics prioritize Obsidian/Glassmorphism layouts, hacking/cyber-neon themes, and extreme data density.</rule>
    <rule id="FE-03" severity="required">Every `{#each}` block MUST use a key: `{#each items as item (item.id)}`. Svelte compiler arrays will warn otherwise.</rule>
    <rule id="FE-04" severity="required">Type safety: Use TypeScript strictly. No `any` types allowed for props, snippet parameters or event handlers.</rule>
    <rule id="FE-05" severity="required">Navigation: Strictly wrap internal path logic using SvelteKit's `$app/paths` `resolve()` when routing.</rule>
    <rule id="FE-06" severity="required">Avoid `@const` invalid placement: Refer to `file:///home/drvoid/ISU/WebQ/.agents/rules/svelte/const_invalid_placement.md` to prevent compilation errors.</rule>
    <rule id="FE-07" severity="required">Avoid module resolution errors: Refer to `file:///home/drvoid/ISU/WebQ/.agents/rules/svelte/sveltekit_tsconfig_paths.md` for tsconfig alias handling constraints.</rule>
  </section>

  <!-- ═══════════════════════════════════════════════════════════
       DEVELOPMENT PROGRESS REPORTING & PLANNING
  ═══════════════════════════════════════════════════════════ -->
  <section id="dev-progress" source="gemini-directives">
    <rule id="DEV-01" severity="required">All progress MUST include structured Tasks and an implementation plan. Agents are required to rigorously break down UI and backend features before coding.</rule>
    <rule id="DEV-02" severity="required">Throughout the development process, `TODO.md` MUST be updated continuously with detailed architectures and planning to help the user decide the next steps.</rule>
  </section>

</rules>
