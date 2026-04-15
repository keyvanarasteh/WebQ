<script lang="ts">
  import { appState } from '$lib/stores/AppState.svelte';
  import * as m from '$lib/paraglide/messages';
  import type { DomainDnsResult, ScanProgressEvent } from '$lib/types/intelligence';
  import { Search, HelpCircle, Clock } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import DnsRecordsBoard from '$lib/components/intelligence/domain-dns/DnsRecordsBoard.svelte';
  import DnsSecurityCheck from '$lib/components/intelligence/domain-dns/DnsSecurityCheck.svelte';
  import DnsGuide from '$lib/components/recon/guides/DnsGuide.svelte';
  import ScanTerminal from '$lib/components/ui/ScanTerminal.svelte';

  let targetDomain = $state('');
  let scanResult = $state<DomainDnsResult | null>(null);
  let scanError = $state<string | null>(null);
  let showGuide = $state(false);

  let scanLogs = $state<ScanProgressEvent[]>([]);
  let scanProgress = $state(0);
  let unlistenProgress: UnlistenFn | null = null;
  
  async function performScan() {
      if (!targetDomain) return;
      appState.setScanning(true, 'DOMAIN DNS');
      scanError = null;
      scanLogs = [];
      scanProgress = 0;
      
      unlistenProgress = await listen<ScanProgressEvent>('scan-progress', (event) => {
          scanLogs.push(event.payload);
          scanProgress = event.payload.percentage;
      });

      try {
          scanResult = await invoke<DomainDnsResult>('scan_domain_dns', { domain: targetDomain });
          scanProgress = 100;
      } catch (e) {
          scanError = e instanceof Error ? e.message : String(e);
          console.error('DNS scan failed:', e);
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
              <h1 class="text-3xl font-black text-primary-text tracking-widest uppercase">{m.intel_dns_title()}</h1>
              <p class="text-muted mt-2">{m.intel_dns_desc()}</p>
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
              {appState.isScanning ? m.intel_dns_resolving() : m.intel_domain_info_scan_btn()}
          </button>
      </div>
  </div>

  <!-- Scan Metadata -->
  {#if scanResult && !appState.isScanning}
      <div class="flex items-center gap-4 text-xs text-muted font-mono">
          <span>Domain: <span class="text-accent">{scanResult.domain}</span></span>
          <span class="flex items-center gap-1"><Clock class="size-3" /> {scanResult.response_time_ms} ms</span>
          <span>@ {new Date(scanResult.timestamp).toLocaleString()}</span>
      </div>
  {/if}

  <!-- Error State -->
  {#if scanError}
      <div class="p-4 bg-red-500/10 border border-red-500/30 rounded-lg text-red-400 text-sm font-mono">
          ⚠ DNS Scan Error: {scanError}
      </div>
  {/if}

  {#if appState.isScanning}
      <div class="mt-8 animate-fade-in">
          <ScanTerminal logs={scanLogs} progressPercent={scanProgress} />
      </div>
  {:else}
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 animate-fade-in">
          <div class="lg:col-span-2">
              <DnsRecordsBoard records={scanResult?.records} isLoading={appState.isScanning} />
          </div>
          <div class="lg:col-span-1 space-y-6">
              <DnsSecurityCheck records={scanResult?.records} isLoading={appState.isScanning} />
          </div>
      </div>
  {/if}
</div>

<DnsGuide bind:isOpen={showGuide} />
