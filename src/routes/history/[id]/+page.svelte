<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  // @ts-expect-error SvelteKit virtual modules generated dynamically
  import { page } from '$app/stores';
  import { ArrowLeft, Database, Download } from 'lucide-svelte';
  // @ts-expect-error SvelteKit virtual modules generated dynamically
  import { resolve } from '$app/paths';
  import { onMount } from 'svelte';

  let scanId = $derived($page.params.id);
  let rawBlob = $state<any>(null);
  let isLoading = $state(true);
  let errorMsg = $state<string | null>(null);

  onMount(async () => {
    try {
      rawBlob = await invoke('get_scan_blob_details', { scanId });
    } catch (e) {
      errorMsg = typeof e === 'string' ? e : String(e);
    } finally {
      isLoading = false;
    }
  });

  function exportJson() {
    if (!rawBlob) return;
    const blob = new Blob([JSON.stringify(rawBlob, null, 2)], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `webq-scan-${scanId}.json`;
    a.click();
    URL.revokeObjectURL(url);
  }

  function titleize(key: string) {
    return key.replace(/_/g, ' ').replace(/\b\w/g, (letter) => letter.toUpperCase());
  }

  const summaryEntries = $derived(
    rawBlob && typeof rawBlob === 'object'
      ? Object.entries(rawBlob)
          .filter(([, value]) => value === null || ['string', 'number', 'boolean'].includes(typeof value))
          .slice(0, 12)
      : []
  );

  const objectSections = $derived(
    rawBlob && typeof rawBlob === 'object'
      ? Object.entries(rawBlob)
          .filter(([, value]) => value && typeof value === 'object')
          .slice(0, 8)
      : []
  );
</script>

<div class="space-y-6">
    <div class="flex items-center gap-4">
        <a href={resolve('/history')} class="p-2 hover:bg-surface rounded-md text-muted transition-colors">
            <ArrowLeft size={20} />
        </a>
        <h1 class="text-3xl font-black text-primary-text tracking-tight flex items-center gap-3">
            <Database class="text-accent size-8" />
            Scan Diagnostics: <span class="font-mono text-xl text-cyan-400 font-normal">{scanId}</span>
        </h1>
    </div>

    {#if isLoading}
        <div class="bg-surface/50 border border-base rounded-xl p-12 text-center text-muted animate-pulse font-mono tracking-widest uppercase shadow-xl">
            Extracting JSON Blob from Telemetry...
        </div>
    {:else if errorMsg}
        <div class="bg-red-500/10 border border-red-500/30 rounded-xl p-6 text-red-400 shadow-xl">
            <h3 class="font-bold mb-2">Extraction Failed</h3>
            <p class="font-mono text-sm">{errorMsg}</p>
        </div>
    {:else}
        <div class="flex justify-end gap-2 mb-4">
            <button onclick={exportJson} class="flex items-center gap-2 bg-surface hover:bg-surface/80 border border-base px-4 py-2 rounded-lg text-sm transition-colors text-primary-text shadow-sm">
                <Download size={16} class="text-accent" /> Export JSON
            </button>
        </div>

        {#if summaryEntries.length > 0}
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                {#each summaryEntries as [key, value]}
                    <div class="bg-surface/60 border border-base rounded-xl p-4">
                        <div class="text-xs uppercase tracking-widest text-muted mb-1">{titleize(key)}</div>
                        <div class="text-primary-text font-mono text-sm break-all">{String(value)}</div>
                    </div>
                {/each}
            </div>
        {/if}

        {#if objectSections.length > 0}
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-4 mb-6">
                {#each objectSections as [key, value]}
                    <div class="bg-surface/50 border border-base rounded-xl overflow-hidden">
                        <div class="px-4 py-3 border-b border-base bg-surface/70 flex items-center justify-between">
                            <h2 class="text-sm font-bold text-primary-text">{titleize(key)}</h2>
                            <span class="text-xs text-muted font-mono">
                                {Array.isArray(value) ? `${value.length} items` : `${Object.keys(value as Record<string, unknown>).length} fields`}
                            </span>
                        </div>
                        <pre class="p-4 text-xs text-cyan-200 font-mono overflow-auto max-h-72 custom-scrollbar">{JSON.stringify(value, null, 2)}</pre>
                    </div>
                {/each}
            </div>
        {/if}

        <div class="bg-[#0b0c10] border border-base rounded-xl overflow-hidden shadow-xl">
            <div class="bg-surface/80 border-b border-base px-4 py-2 flex items-center justify-between">
                <span class="text-xs uppercase tracking-widest text-muted font-bold">raw_json_blob</span>
                <span class="text-xs text-secondary-text font-mono">Size: {JSON.stringify(rawBlob).length} bytes</span>
            </div>
            <pre class="p-6 text-sm text-cyan-300 font-mono overflow-auto max-h-[70vh] custom-scrollbar selection:bg-cyan-500/30 line-numbers">{JSON.stringify(rawBlob, null, 2)}</pre>
        </div>
    {/if}
</div>

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 8px;
        height: 8px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background: rgba(34, 211, 238, 0.2);
        border-radius: 4px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background: rgba(34, 211, 238, 0.4);
    }
</style>
