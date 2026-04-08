  import { appState } from '$lib/stores/AppState.svelte';
  import { Search, UploadCloud } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import ValidationStatsBar from '$lib/components/recon/domain-validator/ValidationStatsBar.svelte';
  import ValidationDataGrid from '$lib/components/recon/domain-validator/ValidationDataGrid.svelte';

  let targetDomains = $state('');
  let scanResult = $state<any>(null); // BulkValidationResult
  
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
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-[#27272a] pb-6">
      <div>
          <h1 class="text-3xl font-black text-white tracking-widest uppercase">Domain Validator</h1>
          <p class="text-gray-400 mt-2">High-speed concurrent scanning for list of domains.</p>
      </div>

      <div class="flex items-center gap-2 w-full md:w-96">
          <div class="relative w-full">
              <UploadCloud class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-cyan-500" />
              <input 
                  type="text" 
                  bind:value={targetDomains} 
                  placeholder="example.com, example2.com"
                  class="w-full bg-[#09090b] border border-[#27272a] rounded-lg py-2 pl-10 pr-4 text-white focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all font-mono text-sm"
              />
          </div>
          <button 
              onclick={performBulkScan}
              disabled={appState.isScanning}
              class="px-6 py-2 rounded-lg bg-cyan-500 text-black font-bold uppercase tracking-wider hover:bg-cyan-400 disabled:opacity-50 transition-colors shrink-0"
          >
              {appState.isScanning ? 'Running...' : 'Validate'}
          </button>
      </div>
  </div>

  {#if scanResult || appState.isScanning}
      <div class="space-y-6">
          <ValidationStatsBar stats={scanResult?.stats} isLoading={appState.isScanning} />
          <ValidationDataGrid results={scanResult?.results} isLoading={appState.isScanning} />
      </div>
  {/if}
</div>
