<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { Shield, Trash2, Search, ExternalLink, Calendar, Star, Clock, BarChart3, CheckSquare, Square, X } from 'lucide-svelte';
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

  type UsageCount = {
    name: string;
    count: number;
  };

  type GlobalStatistics = {
    total_scans: number;
    completed_scans: number;
    failed_scans: number;
    favorite_scans: number;
    average_security_score: number | null;
    critical_vulns: number;
    high_vulns: number;
    medium_vulns: number;
    low_vulns: number;
    top_domains: UsageCount[];
    module_usage: UsageCount[];
  };

  let scans = $state<ScanRow[]>([]);
  let stats = $state<GlobalStatistics | null>(null);
  let isLoading = $state(true);
  let page = $state(0);
  let selectedIds = $state<string[]>([]);
  let filterDomain = $state('');
  let filterModule = $state('');
  let filterStatus = $state('');
  let dateFrom = $state('');
  let dateTo = $state('');
  let sortBy = $state('started_at');
  let sortDir = $state<'asc' | 'desc'>('desc');
  const limit = 20;

  $effect(() => {
    loadScans();
  });

  async function loadScans() {
    try {
      isLoading = true;
      const res = await invoke<ScanRow[]>('get_scans_paginated', {
        limit,
        offset: page * limit,
        filterDomain: filterDomain || null,
        filterModule: filterModule || null,
        filterStatus: filterStatus || null,
        dateFrom: dateFrom ? new Date(dateFrom).toISOString() : null,
        dateTo: dateTo ? new Date(`${dateTo}T23:59:59`).toISOString() : null,
        sortBy,
        sortDir
      });
      scans = res;
      selectedIds = selectedIds.filter((id) => res.some((scan) => scan.id === id));
      stats = await invoke<GlobalStatistics>('get_global_statistics');
    } catch (e) {
      console.error("Failed to fetch scans", e);
    } finally {
      isLoading = false;
    }
  }

  async function toggleFavorite(scan: ScanRow) {
    try {
      const next = await invoke<boolean>('toggle_favorite', { id: scan.id });
      scan.is_favorite = next;
      scans = [...scans];
      stats = await invoke<GlobalStatistics>('get_global_statistics');
    } catch (e) {
      console.error(e);
    }
  }

  function toggleSelected(id: string) {
    selectedIds = selectedIds.includes(id)
      ? selectedIds.filter((selected) => selected !== id)
      : [...selectedIds, id];
  }

  function toggleSelectPage() {
    selectedIds = selectedIds.length === scans.length ? [] : scans.map((scan) => scan.id);
  }

  async function bulkDeleteSelected() {
    if (selectedIds.length === 0) return;
    if (!confirm(`Delete ${selectedIds.length} selected scan logs?`)) return;
    try {
      await invoke('bulk_delete_scans', { ids: selectedIds });
      selectedIds = [];
      await loadScans();
    } catch (e) {
      console.error(e);
    }
  }

  function resetFilters() {
    filterDomain = '';
    filterModule = '';
    filterStatus = '';
    dateFrom = '';
    dateTo = '';
    sortBy = 'started_at';
    sortDir = 'desc';
    page = 0;
    loadScans();
  }

  async function deleteScan(id: string) {
    if (!confirm('Are you sure you want to permanently delete this scan log?')) return;
    try {
      await invoke('delete_scan', { id });
      scans = scans.filter(s => s.id !== id);
      selectedIds = selectedIds.filter((selected) => selected !== id);
      stats = await invoke<GlobalStatistics>('get_global_statistics');
    } catch (e) {
      console.error(e);
    }
  }

  function formatModule(mod: string) {
    return mod.replace(/([A-Z])/g, ' $1').trim();
  }

  const modules = $derived(
    Array.from(
      new Set([...(stats?.module_usage.map((item) => item.name) ?? []), ...scans.map((scan) => scan.scan_module)])
    ).sort()
  );
  const isPageSelected = $derived(scans.length > 0 && selectedIds.length === scans.length);
</script>

<div class="space-y-6">
    <div class="flex items-center justify-between">
        <h1 class="text-3xl font-black text-primary-text tracking-tight flex items-center gap-3">
            <Shield class="text-accent size-8" />
            Scanner History
        </h1>
        <div class="flex items-center gap-2">
            {#if selectedIds.length > 0}
                <button onclick={bulkDeleteSelected} class="px-4 py-2 bg-red-500/10 hover:bg-red-500/20 border border-red-500/30 rounded-lg text-sm text-red-300 transition-colors flex items-center gap-2">
                    <Trash2 size={14} /> Delete {selectedIds.length}
                </button>
            {/if}
            <div class="px-4 py-2 bg-surface border border-base rounded-lg text-sm text-muted">
                Viewing last {limit} operations
            </div>
        </div>
    </div>

    {#if stats}
        <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
            <div class="bg-surface/60 border border-base rounded-xl p-4">
                <div class="text-xs uppercase tracking-widest text-muted mb-1">Total Scans</div>
                <div class="text-2xl font-black text-primary-text">{stats.total_scans}</div>
                <div class="text-xs text-emerald-400 mt-1">{stats.completed_scans} completed</div>
            </div>
            <div class="bg-surface/60 border border-base rounded-xl p-4">
                <div class="text-xs uppercase tracking-widest text-muted mb-1">Avg Score</div>
                <div class="text-2xl font-black text-primary-text">{stats.average_security_score !== null ? stats.average_security_score.toFixed(1) : 'n/a'}</div>
                <div class="text-xs text-muted mt-1">{stats.favorite_scans} favorites</div>
            </div>
            <div class="bg-surface/60 border border-base rounded-xl p-4">
                <div class="text-xs uppercase tracking-widest text-muted mb-1">Findings</div>
                <div class="text-2xl font-black text-primary-text">{stats.critical_vulns + stats.high_vulns + stats.medium_vulns + stats.low_vulns}</div>
                <div class="text-xs text-red-300 mt-1">{stats.critical_vulns} critical / {stats.high_vulns} high</div>
            </div>
            <div class="bg-surface/60 border border-base rounded-xl p-4">
                <div class="text-xs uppercase tracking-widest text-muted mb-1 flex items-center gap-2"><BarChart3 size={12} /> Top Module</div>
                <div class="text-lg font-black text-primary-text truncate">{stats.module_usage[0]?.name ? formatModule(stats.module_usage[0].name) : 'n/a'}</div>
                <div class="text-xs text-muted mt-1">{stats.top_domains[0]?.name ?? 'No domains yet'}</div>
            </div>
        </div>
    {/if}

    <div class="bg-surface/50 border border-base rounded-xl p-4 grid grid-cols-1 md:grid-cols-7 gap-3">
        <input bind:value={filterDomain} placeholder="Filter domain" class="md:col-span-2 bg-background border border-base rounded-lg px-3 py-2 text-sm text-primary-text" />
        <select bind:value={filterModule} class="bg-background border border-base rounded-lg px-3 py-2 text-sm text-primary-text">
            <option value="">All modules</option>
            {#each modules as module}
                <option value={module}>{formatModule(module)}</option>
            {/each}
        </select>
        <select bind:value={filterStatus} class="bg-background border border-base rounded-lg px-3 py-2 text-sm text-primary-text">
            <option value="">All statuses</option>
            <option value="Completed">Completed</option>
            <option value="Failed">Failed</option>
        </select>
        <input bind:value={dateFrom} type="date" class="bg-background border border-base rounded-lg px-3 py-2 text-sm text-primary-text" />
        <input bind:value={dateTo} type="date" class="bg-background border border-base rounded-lg px-3 py-2 text-sm text-primary-text" />
        <div class="flex gap-2">
            <button onclick={() => { page = 0; loadScans(); }} class="flex-1 bg-cyan-500/10 hover:bg-cyan-500/20 border border-cyan-500/30 rounded-lg px-3 py-2 text-sm text-cyan-300">Apply</button>
            <button onclick={resetFilters} class="bg-base hover:bg-surface rounded-lg px-3 py-2 text-muted" title="Clear filters"><X size={16} /></button>
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
                        <th class="p-4 font-semibold">
                            <button onclick={toggleSelectPage} class="text-muted hover:text-primary-text" title="Select page">
                                {#if isPageSelected}
                                    <CheckSquare size={16} />
                                {:else}
                                    <Square size={16} />
                                {/if}
                            </button>
                        </th>
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
                                <button onclick={() => toggleSelected(scan.id)} class="text-muted hover:text-cyan-300" title="Select scan">
                                    {#if selectedIds.includes(scan.id)}
                                        <CheckSquare size={16} />
                                    {:else}
                                        <Square size={16} />
                                    {/if}
                                </button>
                            </td>
                            <td class="p-4">
                                <div class="flex items-center gap-3">
                                    <button onclick={() => toggleFavorite(scan)} title={scan.is_favorite ? 'Remove favorite' : 'Favorite scan'}>
                                        <Star class="size-4 {scan.is_favorite ? 'text-amber-400 fill-amber-400' : 'text-slate-600'} cursor-pointer" />
                                    </button>
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
