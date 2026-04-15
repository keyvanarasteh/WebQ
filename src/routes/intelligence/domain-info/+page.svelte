<script lang="ts">
  import { appState } from '$lib/stores/AppState.svelte';
  import * as m from '$lib/paraglide/messages';
  import type { DomainInfoResult, ScanProgressEvent } from '$lib/types/intelligence';
  import DomainOverview from '$lib/components/intelligence/domain-info/DomainOverview.svelte';
  import SslStatus from '$lib/components/intelligence/domain-info/SslStatus.svelte';
  import PortSecurityMatrix from '$lib/components/intelligence/domain-info/PortSecurityMatrix.svelte';
  import SecurityDetails from '$lib/components/intelligence/domain-info/SecurityDetails.svelte';
  import DomainInfoGuide from '$lib/components/intelligence/domain-info/DomainInfoGuide.svelte';
  import ScanTerminal from '$lib/components/ui/ScanTerminal.svelte';
  import { Search, HelpCircle } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';

  let targetDomain = $state('');
  let scanResult = $state<DomainInfoResult | null>(null);
  let scanError = $state<string | null>(null);
  let showGuide = $state(false);

  let scanLogs = $state<ScanProgressEvent[]>([]);
  let scanProgress = $state(0);
  let unlistenProgress: UnlistenFn | null = null;
  
  async function performScan() {
      if (!targetDomain) return;
      appState.setScanning(true, 'DOMAIN INFO');
      scanError = null;
      scanLogs = [];
      scanProgress = 0;
      
      unlistenProgress = await listen<ScanProgressEvent>('scan-progress', (event) => {
          scanLogs.push(event.payload);
          scanProgress = event.payload.percentage;
      });

      try {
          scanResult = await invoke<DomainInfoResult>('scan_domain_info', { domain: targetDomain });
          scanProgress = 100;
      } catch (e) {
          scanError = e instanceof Error ? e.message : String(e);
          console.error('Domain Info scan failed:', e);
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
  <!-- Header & Input Area -->
  <div class="flex flex-col md:flex-row md:items-end justify-between gap-4 border-b border-base pb-6">
      <div class="flex items-center gap-3">
          <div>
              <h1 class="text-3xl font-black text-primary-text tracking-widest uppercase">{m.intel_domain_info_title()}</h1>
              <p class="text-muted mt-2">{m.intel_domain_info_desc()}</p>
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
                  onkeydown={(e) => e.key === 'Enter' && performScan()}
                  placeholder="Enter domain (e.g. example.com)"
                  class="w-full bg-background border border-base rounded-lg py-2 pl-10 pr-4 text-primary-text focus:outline-none focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 transition-all font-mono text-sm"
              />
          </div>
          <button 
              onclick={performScan}
              disabled={appState.isScanning}
              class="px-6 py-2 rounded-lg bg-cyan-500 text-on-accent font-bold uppercase tracking-wider hover:bg-cyan-400 disabled:opacity-50 transition-colors shrink-0"
          >
              {appState.isScanning ? m.intel_domain_info_scanning() : m.intel_domain_info_scan_btn()}
          </button>
      </div>
  </div>

  <!-- Error State -->
  {#if scanError}
      <div class="p-4 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-sm font-mono">
          ⚠ Scan Error: {scanError}
      </div>
  {/if}

  {#if appState.isScanning}
      <div class="mt-8 animate-fade-in">
          <ScanTerminal logs={scanLogs} progressPercent={scanProgress} />
      </div>
  {:else if scanResult}
      <!-- Adaptive Grid Layout -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 animate-fade-in">
          <div class="lg:col-span-2">
              <DomainOverview isLoading={appState.isScanning} result={scanResult} />
          </div>
          <div class="lg:col-span-1 space-y-6">
              <SslStatus isLoading={appState.isScanning} ssl={scanResult?.ssl ?? null} />
              <SecurityDetails isLoading={appState.isScanning} security={scanResult?.security ?? null} score={scanResult?.security_score} />
              <PortSecurityMatrix isLoading={appState.isScanning} ports={scanResult?.open_ports} score={scanResult?.security_score} />
          </div>
      </div>
  {/if}
</div>

<DomainInfoGuide bind:isOpen={showGuide} />
