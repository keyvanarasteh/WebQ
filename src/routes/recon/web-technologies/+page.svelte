<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
    import * as m from '$lib/paraglide/messages';
    import { Search, HelpCircle } from 'lucide-svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import type { ScanProgressEvent } from '$lib/types/intelligence';
    import TechStackGrid from '$lib/components/recon/web-technologies/TechStackGrid.svelte';
    import WordPressScanner from '$lib/components/recon/web-technologies/WordPressScanner.svelte';
    import SecurityHeadersList from '$lib/components/recon/web-technologies/SecurityHeadersList.svelte';
    import TechStackGuide from '$lib/components/recon/guides/TechStackGuide.svelte';
    import ScanTerminal from '$lib/components/ui/ScanTerminal.svelte';

  let targetDomain = $state('');
  let scanResult = $state<any>(null); // WebTechResult
  let showGuide = $state(false);
  let scanLogs = $state<ScanProgressEvent[]>([]);
  let scanProgress = $state(0);
  let unlistenProgress: UnlistenFn | null = null;
  
  async function performScan() {
      if (!targetDomain) return;
      appState.setScanning(true, 'WEB TECHNOLOGIES');
      scanLogs = [];
      scanProgress = 0;
      unlistenProgress = await listen<ScanProgressEvent>('scan-progress', (event) => {
          scanLogs.push(event.payload);
          scanProgress = event.payload.percentage;
      });
      
      try {
          scanResult = await invoke('scan_web_technologies', { url: targetDomain });
      } catch (e) {
          console.error(e);
      } finally {
          if (unlistenProgress) {
              unlistenProgress();
              unlistenProgress = null;
          }
          appState.setScanning(false, '');
      }
  }
</script>

<div class="space-y-6 max-w-7xl mx-auto w-full">
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-base pb-6">
      <div class="flex items-center gap-3">
          <div>
              <h1 class="text-3xl font-black text-primary-text tracking-widest uppercase">{m.recon_tech_title()}</h1>
              <p class="text-muted mt-2">{m.recon_tech_desc()}</p>
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
                  list="historic-domains"
                  bind:value={targetDomain} 
                  placeholder="Enter target URL..."
                  class="w-full bg-background border border-base rounded-lg py-2 pl-10 pr-4 text-primary-text focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all font-mono text-sm"
              />
          </div>
          <button 
              onclick={performScan}
              disabled={appState.isScanning}
              class="px-6 py-2 rounded-lg bg-cyan-500 text-on-accent font-bold uppercase tracking-wider hover:bg-cyan-400 disabled:opacity-50 transition-colors shrink-0"
          >
              {appState.isScanning ? m.recon_tech_detecting() : m.recon_tech_detect_btn()}
          </button>
      </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      {#if appState.isScanning || scanLogs.length > 0}
          <div class="lg:col-span-2">
              <ScanTerminal logs={scanLogs} progressPercent={scanProgress} />
          </div>
      {/if}
      <TechStackGrid data={scanResult ?? undefined} isLoading={appState.isScanning} />
      <WordPressScanner data={scanResult?.wp_analysis ?? undefined} isLoading={appState.isScanning} />
      
      <!-- SecurityHeadersList already has col-span-full in its container -->
      <SecurityHeadersList data={scanResult?.security_headers ?? undefined} isLoading={appState.isScanning} />
  </div>
</div>

<TechStackGuide bind:isOpen={showGuide} />
