<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { Shield, Trash2, Search, ExternalLink, Calendar, Star, Clock } from 'lucide-svelte';
  // @ts-expect-error
  import { resolve } from '$app/paths';

  type ScanRow = {
    id: string;
    target_domain: string;
    scan_module: string;
    status: string;
    error_message: string | null;
    is_favorite: boolean;
    duration_ms: number;
    started_at: string;
    finished_at: string | null;
  };

  let scans = $state<ScanRow[]>([]);
  let isLoading = $state(true);
  let page = $state(0);
  const limit = 20;

  $effect(() => {
    loadScans();
  });

  async function loadScans() {
    try {
      isLoading = true;
      const res = await invoke<ScanRow[]>('get_scans_paginated', {
        limit,
        offset: page * limit
      });
      scans = res;
    } catch (e) {
      console.error("Failed to fetch scans", e);
    } finally {
      isLoading = false;
    }
  }

  async function deleteScan(id: string) {
    if (!confirm('Are you sure you want to permanently delete this scan log?')) return;
    try {
      await invoke('delete_scan', { id });
      scans = scans.filter(s => s.id !== id);
    } catch (e) {
      console.error(e);
    }
  }

  function formatModule(mod: string) {
    return mod.replace(/([A-Z])/g, ' $1').trim();
  }
</script>

<div class="space-y-6">
    <div class="flex items-center justify-between">
        <h1 class="text-3xl font-black text-primary-text tracking-tight flex items-center gap-3">
            <Shield class="text-accent size-8" />
            Scanner History
        </h1>
        <div class="px-4 py-2 bg-surface border border-base rounded-lg text-sm text-muted">
            Viewing last {limit} operations
        </div>
    </div>

    <div class="bg-surface/50 border border-base rounded-xl overflow-hidden shadow-xl">
        {#if isLoading && scans.length === 0}
            <div class="p-12 text-center text-muted animate-pulse font-mono tracking-widest uppercase">
                Querying Telemetry Database...
            </div>
        {:else if scans.length === 0}
            <div class="p-12 text-center flex flex-col items-center gap-4">
                <Search class="size-12 text-muted opacity-50" />
                <h3 class="text-xl font-bold text-primary-text">No Scans Recorded</h3>
                <p class="text-secondary-text">Launch a module from the dashboard to populate the analysis database.</p>
            </div>
        {:else}
            <table class="w-full text-left border-collapse">
                <thead>
                    <tr class="bg-base border-b border-base/50 text-xs uppercase tracking-widest text-muted">
                        <th class="p-4 font-semibold">Target Domain</th>
                        <th class="p-4 font-semibold">Module</th>
                        <th class="p-4 font-semibold">Status</th>
                        <th class="p-4 font-semibold">Timestamp</th>
                        <th class="p-4 font-semibold text-right">Actions</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-base/30">
                    {#each scans as scan (scan.id)}
                        <tr class="hover:bg-surface/80 transition-colors group">
                            <td class="p-4">
                                <div class="flex items-center gap-3">
                                    <Star class="size-4 {scan.is_favorite ? 'text-amber-400 fill-amber-400' : 'text-slate-600'} cursor-pointer" />
                                    <span class="font-bold text-primary-text">{scan.target_domain}</span>
                                </div>
                            </td>
                            <td class="p-4 text-secondary-text text-sm flex items-center gap-2">
                                {formatModule(scan.scan_module)}
                                {#if scan.duration_ms > 0}
                                    <span class="text-[10px] bg-base px-2 py-0.5 rounded-full text-muted flex items-center gap-1">
                                        <Clock size={10} /> {(scan.duration_ms / 1000).toFixed(1)}s
                                    </span>
                                {/if}
                            </td>
                            <td class="p-4">
                                {#if scan.status === 'Completed'}
                                    <span class="bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 px-2.5 py-1 rounded-md text-xs font-bold uppercase tracking-wider">
                                        Success
                                    </span>
                                {:else if scan.status === 'Running'}
                                    <span class="bg-cyan-500/10 text-cyan-400 border border-cyan-500/20 px-2.5 py-1 rounded-md text-xs font-bold uppercase tracking-wider animate-pulse">
                                        Scanning
                                    </span>
                                {:else}
                                    <span class="bg-red-500/10 text-red-400 border border-red-500/20 px-2.5 py-1 rounded-md text-xs font-bold uppercase tracking-wider" title={scan.error_message}>
                                        Failed
                                    </span>
                                {/if}
                            </td>
                            <td class="p-4 text-sm text-secondary-text font-mono flex items-center gap-2">
                                <Calendar size={14} class="opacity-50" />
                                {new Date(scan.started_at).toLocaleString()}
                            </td>
                            <td class="p-4 text-right">
                                <div class="flex items-center justify-end gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                                    <a href={resolve(`/history/${scan.id}`)} class="p-2 bg-base hover:bg-cyan-500/20 hover:text-cyan-400 rounded-md text-muted transition-colors" title="View Report">
                                        <ExternalLink size={16} />
                                    </a>
                                    <button onclick={() => deleteScan(scan.id)} class="p-2 bg-base hover:bg-red-500/20 hover:text-red-400 rounded-md text-muted transition-colors" title="Delete Log">
                                        <Trash2 size={16} />
                                    </button>
                                </div>
                            </td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
    </div>
</div>
