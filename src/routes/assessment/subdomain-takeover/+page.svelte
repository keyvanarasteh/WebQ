<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import * as m from '$lib/paraglide/messages';
  import { ShieldAlert, Search, Loader2, AlertCircle, RefreshCw } from 'lucide-svelte';
  import TakeoverGrid from '$lib/components/assessment/subdomain-takeover/TakeoverGrid.svelte';
  import TakeoverGuide from '$lib/components/assessment/subdomain-takeover/TakeoverGuide.svelte';
  import { fade } from 'svelte/transition';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import ScanTerminal from '$lib/components/ui/ScanTerminal.svelte';
  import type { ScanProgressEvent } from '$lib/types/intelligence';
  
  let targetDomain = $state('');
  let subdomainsRaw = $state('');
  let isScanning = $state(false);
  let scanError = $state<string | null>(null);
  let scanResult = $state<any>(null);
  let showGuide = $state(false);
  
  let logs = $state<ScanProgressEvent[]>([]);
  let progressPercent = $state(0);

  $effect(() => {
      let unlistenP: UnlistenFn | null = null;
      listen<ScanProgressEvent>('scan-progress', (event) => {
          logs.push(event.payload);
          progressPercent = event.payload.percentage;
      }).then(u => unlistenP = u);
      return () => { if (unlistenP) unlistenP(); };
  });
  
  async function startScan() {
    if (!targetDomain || !subdomainsRaw.trim()) return;
    
    isScanning = true;
    scanError = null;
    scanResult = null;
    logs = [];
    progressPercent = 0;
    logs = [];
    progressPercent = 0;
    
    try {
      const subdomainsList = subdomainsRaw.split('\n')
        .map(s => s.trim())
        .filter(s => s.length > 0);
        
      scanResult = await invoke('scan_subdomain_takeover', {
        domain: targetDomain,
        subdomains: subdomainsList
      });
    } catch (err: unknown) {
      scanError = err instanceof Error ? err.message : String(err);
    } finally {
      isScanning = false;
    }
  }
</script>

<div class="space-y-6">
  <!-- Header -->
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-2xl font-semibold text-primary-text tracking-tight flex items-center gap-2">
        <ShieldAlert size={24} class="text-rose-500" />
        {m.sec_takeover_title()}
      </h1>
      <p class="text-muted text-sm mt-1">
        {m.sec_takeover_desc()}
      </p>
    </div>
    
    <button 
      onclick={() => showGuide = true}
      class="flex items-center gap-2 px-3 py-1.5 text-sm font-medium text-secondary-text bg-glass border border-glass rounded-lg hover:bg-glass-hover transition-colors"
    >
      <AlertCircle size={16} class="text-indigo-400" />
      {m.sec_takeover_guide_title()}
    </button>
  </div>

  <!-- Input Form -->
  <div class="p-6 rounded-xl border border-subtle bg-[#14171C] space-y-4">
    <div class="space-y-2">
      <label for="domainInput" class="text-sm font-medium text-secondary-text">{m.val_target_domain()}</label>
      <input
        id="domainInput"
        type="text"
        bind:value={targetDomain}
        placeholder="example.com"
        class="w-full bg-sunken border border-glass text-primary-text rounded-lg px-4 py-2.5 focus:outline-none focus:border-rose-500/50 focus:ring-1 focus:ring-rose-500/50 transition-all font-mono text-sm placeholder:text-tertiary-text"
        disabled={isScanning}
      />
    </div>
    
    <div class="space-y-2">
      <label for="subdomainsInput" class="text-sm font-medium text-secondary-text">Subdomains List</label>
      <textarea
        id="subdomainsInput"
        bind:value={subdomainsRaw}
        placeholder="api.example.com&#10;dev.example.com&#10;staging.example.com..."
        class="w-full h-40 bg-sunken border border-glass text-primary-text rounded-lg px-4 py-3 focus:outline-none focus:border-rose-500/50 focus:ring-1 focus:ring-rose-500/50 transition-all font-mono text-sm placeholder:text-tertiary-text resize-y"
        disabled={isScanning}
      ></textarea>
      <div class="text-xs text-muted flex items-center justify-between">
        <span>Paste one subdomain per line.</span>
        <span>{subdomainsRaw.split('\n').filter(s => s.trim().length > 0).length} subdomains detected</span>
      </div>
    </div>

    <div class="pt-2">
      <button
        onclick={startScan}
        disabled={isScanning || !targetDomain || !subdomainsRaw.trim()}
        class="flex items-center justify-center gap-2 w-full py-3 rounded-lg bg-rose-600 hover:bg-rose-500 text-primary-text font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {#if isScanning}
          <Loader2 size={18} class="animate-spin" />
          Scanning Targets...
        {:else}
          <Search size={18} />
          {m.sec_takeover_title()}
        {/if}
      </button>
    </div>
  </div>

  <!-- Errors -->
  {#if scanError}
    <div class="p-4 rounded-xl bg-rose-500/10 border border-rose-500/20 text-rose-400 text-sm flex items-start gap-3">
      <AlertCircle size={18} class="mt-0.5 shrink-0" />
      <div>
        <span class="font-semibold block mb-1">Scan Failed</span>
        {scanError}
      </div>
    </div>
  {/if}

  <!-- Results -->
  {#if scanResult}
    <div class="space-y-4">
      <!-- Stats Banner -->
      <div class="flex items-center justify-between p-4 rounded-xl border border-subtle bg-[#14171C]">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-rose-500/10 text-rose-400">
            <RefreshCw size={20} />
          </div>
          <div>
            <div class="text-primary-text font-medium">Scan Complete</div>
            <div class="text-sm text-muted">
              {m.sec_takeover_stats({ count: scanResult.statistics.subdomains_scanned, vuln: scanResult.statistics.vulnerable_count })}
              <span class="ml-2 text-muted">
                (Time: {scanResult.statistics.scan_time_secs.toFixed(2)}s)
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- Takeover Grid -->
      <TakeoverGrid vulnerableSubdomains={scanResult.vulnerable} />
    </div>
  {/if}
</div>

<TakeoverGuide bind:isOpen={showGuide} />
