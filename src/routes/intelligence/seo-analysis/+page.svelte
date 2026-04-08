<script lang="ts">
  import { appState } from '$lib/stores/AppState.svelte';
  import { Search } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';

  let targetDomain = $state('');
  let scanResult = $state<any>(null); // SeoAnalysisResult
  
  async function performScan() {
      if (!targetDomain) return;
      appState.setScanning(true, 'SEO ANALYSIS');
      
      try {
          scanResult = await invoke('scan_seo_analysis', { url: targetDomain });
      } catch (e) {
          console.error(e);
      } finally {
          appState.setScanning(false, '');
      }
  }
</script>

<div class="space-y-6 max-w-7xl mx-auto w-full">
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-[#27272a] pb-6">
      <div>
          <h1 class="text-3xl font-black text-white tracking-widest uppercase">SEO Analysis</h1>
          <p class="text-gray-400 mt-2">13-category SEO scanner & optimization diagnostics.</p>
      </div>

      <div class="flex items-center gap-2 w-full md:w-96">
          <div class="relative w-full">
              <Search class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-cyan-500" />
              <input 
                  type="text" 
                  bind:value={targetDomain} 
                  placeholder="Enter URL (e.g. https://example.com)"
                  class="w-full bg-[#09090b] border border-[#27272a] rounded-lg py-2 pl-10 pr-4 text-white focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all font-mono text-sm"
              />
          </div>
          <button 
              onclick={performScan}
              disabled={appState.isScanning}
              class="px-6 py-2 rounded-lg bg-cyan-500 text-black font-bold uppercase tracking-wider hover:bg-cyan-400 disabled:opacity-50 transition-colors shrink-0"
          >
              {appState.isScanning ? 'Auditing...' : 'Audit'}
          </button>
      </div>
  </div>

  {#if scanResult || appState.isScanning}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <!-- placeholder for BasicSeoOverview -->
          <div class="bg-[#09090b] border border-[#27272a] p-6 rounded-xl text-gray-500">
              Basic SEO Overview Placeholder
          </div>
          
          <!-- placeholder for TechnicalSeoCard -->
          <div class="bg-[#09090b] border border-[#27272a] p-6 rounded-xl text-gray-500">
              Technical SEO Card Placeholder
          </div>

          <!-- placeholder for SocialMediaCard -->
          <div class="bg-[#09090b] border border-[#27272a] p-6 rounded-xl text-gray-500">
              Social Media Graph Placeholder
          </div>
      </div>
  {/if}
</div>
