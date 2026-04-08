<script lang="ts">
  import { appState } from '$lib/stores/AppState.svelte';
</script>

<div class="space-y-6 max-w-4xl mx-auto">
  <div class="border-b border-[#27272a] pb-4">
      <h1 class="text-3xl font-black tracking-widest text-white uppercase shadow-cyan-400 drop-shadow-sm">Settings</h1>
      <p class="mt-2 text-gray-400">Configure core scanning behaviors, API keys, and UI preferences.</p>
  </div>

  <section class="bg-[#27272a]/20 border border-[#27272a] rounded-xl p-6 shadow-lg">
    <h2 class="text-xl font-semibold mb-4 text-cyan-400">Application Preferences</h2>
    
    <div class="space-y-4">
      <div class="flex items-center justify-between p-4 bg-[#09090b] rounded-lg border border-[#27272a]">
        <div>
          <p class="font-medium text-white">Display Theme</p>
          <p class="text-sm text-gray-400">Toggle between deep obsidian and light modes.</p>
        </div>
        <button class="px-5 py-2 rounded-md bg-cyan-500/10 border border-cyan-500/30 text-cyan-400 font-bold hover:bg-cyan-500/20 hover:shadow-[0_0_10px_rgba(34,211,238,0.3)] transition-all" onclick={() => appState.toggleTheme()}>
          {appState.theme === 'dark' ? 'Activate Light' : 'Activate Dark'}
        </button>
      </div>

      <div class="flex items-center justify-between p-4 bg-[#09090b] rounded-lg border border-[#27272a]">
        <div>
          <p class="font-medium text-white cursor-not-allowed">Export Path (WIP)</p>
          <p class="text-sm text-gray-400">Default directory for Web Analyzer reports.</p>
        </div>
        <button class="px-5 py-2 rounded-md bg-[#27272a]/50 text-gray-500 font-bold cursor-not-allowed border border-[#27272a]">
          Select Path
        </button>
      </div>
    </div>
  </section>
</div>
