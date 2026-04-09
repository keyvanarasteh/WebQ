<script lang="ts">
  import { appState } from '$lib/stores/AppState.svelte';
  import DomainOverview from '$lib/components/intelligence/domain-info/DomainOverview.svelte';
  import SslStatus from '$lib/components/intelligence/domain-info/SslStatus.svelte';
  import PortSecurityMatrix from '$lib/components/intelligence/domain-info/PortSecurityMatrix.svelte';
  import DomainInfoGuide from '$lib/components/intelligence/domain-info/DomainInfoGuide.svelte';
  import { Search, HelpCircle } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';

  // State: Using Svelte 5 Runes for reactive data
  let targetDomain = $state('');
  let scanResult = $state<any>(null); // To be typed as DomainInfoResult
  let showGuide = $state(false);
  
  async function performScan() {
      if (!targetDomain) return;
      appState.setScanning(true, 'DOMAIN INFO');
      
      try {
          // Calling rust backend API
          scanResult = await invoke('scan_domain_info', { domain: targetDomain });
      } catch (e) {
          // Toast Error Handling
      } finally {
          appState.setScanning(false, '');
      }
  }
</script>

<div class="space-y-6 max-w-7xl mx-auto w-full">
  <!-- Header & Input Area -->
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-[#27272a] pb-6">
      <div class="flex items-center gap-3">
          <div>
              <h1 class="text-3xl font-black text-white tracking-widest uppercase">Domain Intelligence</h1>
              <p class="text-gray-400 mt-2">Comprehensive WHOIS, DNS, SSL, and basic security footprinting.</p>
          </div>
          <button
              onclick={() => showGuide = true}
              class="p-2 ml-2 transition-colors border rounded-lg bg-gray-900 border-gray-800 text-gray-400 hover:text-white"
              title="View SecOps Guide"
          >
              <HelpCircle class="w-4 h-4" />
          </button>
      </div>

      <div class="flex items-center gap-2 w-full md:w-96">
          <div class="relative w-full">
              <Search class="absolute left-3 top-1/2 -translate-y-1/2 size-4 text-cyan-500" />
              <input 
                  type="text" 
                  bind:value={targetDomain} 
                  placeholder="Enter domain (e.g. example.com)"
                  class="w-full bg-[#09090b] border border-[#27272a] rounded-lg py-2 pl-10 pr-4 text-white focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all font-mono text-sm"
              />
          </div>
          <button 
              onclick={performScan}
              disabled={appState.isScanning}
              class="px-6 py-2 rounded-lg bg-cyan-500 text-black font-bold uppercase tracking-wider hover:bg-cyan-400 disabled:opacity-50 transition-colors shrink-0"
          >
              {appState.isScanning ? 'Scanning...' : 'Scan'}
          </button>
      </div>
  </div>

  <!-- Adaptive Grid Layout -->
  {#if scanResult || appState.isScanning}
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <div class="lg:col-span-2">
              <DomainOverview isLoading={appState.isScanning} result={scanResult} />
          </div>
          <div class="lg:col-span-1 space-y-6">
              <SslStatus isLoading={appState.isScanning} ssl={scanResult?.ssl} />
              <PortSecurityMatrix isLoading={appState.isScanning} ports={scanResult?.open_ports} score={scanResult?.security_score} />
          </div>
      </div>
  {/if}
</div>

<DomainInfoGuide bind:isOpen={showGuide} />
