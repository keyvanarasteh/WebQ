<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
  import * as m from '$lib/paraglide/messages';
  import { Search, UploadCloud, HelpCircle } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import ValidationStatsBar from '$lib/components/recon/domain-validator/ValidationStatsBar.svelte';
  import ValidationDataGrid from '$lib/components/recon/domain-validator/ValidationDataGrid.svelte';
  import ValidatorGuide from '$lib/components/recon/guides/ValidatorGuide.svelte';

  let targetDomains = $state('');
  let scanResult = $state<any>(null); // BulkValidationResult
  let showGuide = $state(false);
  
  async function performBulkScan() {
      if (!targetDomains) return;
      const domains = targetDomains.split(',').map(d => d.trim()).filter(Boolean);
      
      appState.setScanning(true, 'BULK VALIDATION');
      try {
          scanResult = await invoke('validate_bulk_domains', { domains });
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
              <h1 class="text-3xl font-black text-primary-text tracking-widest uppercase">{m.recon_validator_title()}</h1>
              <p class="text-muted mt-2">{m.recon_validator_desc()}</p>
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
              <UploadCloud class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-accent" />
              <input 
                  type="text" 
                  list="historic-domains"
                  bind:value={targetDomains} 
                  placeholder="example.com, example2.com"
                  class="w-full bg-background border border-base rounded-lg py-2 pl-10 pr-4 text-primary-text focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all font-mono text-sm"
              />
          </div>
          <button 
              onclick={performBulkScan}
              disabled={appState.isScanning}
              class="px-6 py-2 rounded-lg bg-cyan-500 text-on-accent font-bold uppercase tracking-wider hover:bg-cyan-400 disabled:opacity-50 transition-colors shrink-0"
          >
              {appState.isScanning ? m.recon_validator_running() : m.recon_validator_btn()}
          </button>
      </div>
  </div>

  <div class="space-y-6">
      <ValidationStatsBar stats={scanResult?.stats} isLoading={appState.isScanning} />
      <ValidationDataGrid results={scanResult?.results} isLoading={appState.isScanning} />
  </div>
</div>

<ValidatorGuide bind:isOpen={showGuide} />
