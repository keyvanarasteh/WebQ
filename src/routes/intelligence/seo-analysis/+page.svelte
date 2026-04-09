<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
  import * as m from '$lib/paraglide/messages';
  import { Search, HelpCircle } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import BasicSeoOverview from '$lib/components/intelligence/seo-analysis/BasicSeoOverview.svelte';
  import TechnicalSeoCard from '$lib/components/intelligence/seo-analysis/TechnicalSeoCard.svelte';
  import SocialMediaCard from '$lib/components/intelligence/seo-analysis/SocialMediaCard.svelte';
  import SeoGuide from '$lib/components/recon/guides/SeoGuide.svelte';

  let targetDomain = $state('');
  let scanResult = $state<any>(null); // SeoAnalysisResult
  let showGuide = $state(false);
  
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
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-base pb-6">
      <div class="flex items-center gap-3">
          <div>
              <h1 class="text-3xl font-black text-primary-text tracking-widest uppercase">{m.intel_seo_title()}</h1>
              <p class="text-muted mt-2">{m.intel_seo_desc()}</p>
          </div>
          <button
              onclick={() => showGuide = true}
              class="p-2 ml-2 transition-colors border rounded-lg bg-surface border-base text-muted hover:text-primary-text"
              title="View SecOps Guide"
          >
              <HelpCircle class="w-4 h-4" />
          </button>
      </div>

      <div class="flex items-center gap-2 w-full md:w-96">
          <div class="relative w-full">
              <Search class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-accent" />
              <input 
                  type="text" 
                  bind:value={targetDomain} 
                  placeholder="Enter URL (e.g. https://example.com)"
                  class="w-full bg-background border border-base rounded-lg py-2 pl-10 pr-4 text-primary-text focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all font-mono text-sm"
              />
          </div>
          <button 
              onclick={performScan}
              disabled={appState.isScanning}
              class="px-6 py-2 rounded-lg bg-cyan-500 text-black font-bold uppercase tracking-wider hover:bg-cyan-400 disabled:opacity-50 transition-colors shrink-0"
          >
              {appState.isScanning ? m.intel_seo_auditing() : m.intel_seo_audit_btn()}
          </button>
      </div>
  </div>

  {#if scanResult || appState.isScanning}
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          <BasicSeoOverview data={scanResult?.basic} isLoading={appState.isScanning} />
          
          <TechnicalSeoCard data={scanResult?.technical} isLoading={appState.isScanning} />

          <SocialMediaCard data={scanResult?.social} isLoading={appState.isScanning} />
      </div>
  {/if}
</div>

<SeoGuide bind:isOpen={showGuide} />
