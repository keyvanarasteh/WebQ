<script lang="ts">
  import { appState } from '$lib/stores/AppState.svelte';
  import SegmentedControl from '$lib/components/ui/SegmentedControl.svelte';
  import DatabaseManager from '$lib/components/settings/DatabaseManager.svelte';
  import EngineHealth from '$lib/components/settings/EngineHealth.svelte';

  const themeOptions = [
    { label: 'Obsidian Dark', value: 'dark' },
    { label: 'Cyber Light', value: 'light' }
  ];
</script>

<div class="space-y-6 max-w-5xl mx-auto pb-10">
  <div class="border-b border-base pb-4">
      <h1 class="text-3xl font-black tracking-widest text-primary-text uppercase shadow-cyan-400 drop-shadow-sm">Settings</h1>
      <p class="mt-2 text-muted">Configure core scanning behaviors, API keys, database history, and UI preferences.</p>
  </div>

  <section class="bg-surface/20 border border-base rounded-xl p-6 shadow-lg">
    <h2 class="text-xl font-semibold mb-4 text-accent">Application Preferences</h2>
    
    <div class="space-y-4">
      <div class="flex items-center justify-between p-4 bg-background rounded-lg border border-base flex-wrap gap-4">
        <div>
          <p class="font-medium text-primary-text">Display Theme</p>
          <p class="text-sm text-muted">Toggle between deep obsidian and light modes.</p>
        </div>
        <SegmentedControl options={themeOptions} bind:value={appState.theme} />
      </div>

      <div class="flex items-center justify-between p-4 bg-background rounded-lg border border-base flex-wrap gap-4">
        <div>
          <p class="font-medium text-primary-text cursor-not-allowed">Export Path (WIP)</p>
          <p class="text-sm text-muted">Default directory for Web Analyzer reports.</p>
        </div>
        <button class="px-5 py-2 rounded-md bg-surface/50 text-muted font-bold cursor-not-allowed border border-base">
          Select Path
        </button>
      </div>
    </div>
  </section>

  <!-- Database Manager Component -->
  <DatabaseManager />

  <!-- Engine Health Scanner Module -->
  <div class="mt-10 pt-8 border-t border-base border-dashed">
      <EngineHealth />
  </div>
</div>
