<script lang="ts">
  import type { PerformanceResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Gauge } from 'lucide-svelte';

  type Props = {
      data: PerformanceResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <Gauge class="size-5" /> {m.seo_performance_title()}
  </h3>

  {#if isLoading}
    <div class="space-y-3 animate-pulse">
        <div class="h-16 bg-surface rounded"></div>
        <div class="h-8 bg-surface rounded"></div>
    </div>
  {:else if data}
    <div class="grid grid-cols-2 gap-2 mb-3">
        <div class="p-3 bg-sunken rounded-lg border border-base text-center">
            <p class="text-2xl font-black {data.load_time_status === 'Excellent' ? 'text-green-400' : data.load_time_status === 'Good' ? 'text-cyan-400' : 'text-red-400'}">{data.load_time_secs}s</p>
            <p class="text-xs text-muted">{data.load_time_status}</p>
        </div>
        <div class="p-3 bg-sunken rounded-lg border border-base text-center">
            <p class="text-2xl font-black text-primary-text">{data.content_size_kb}<span class="text-sm text-muted ml-1">KB</span></p>
            <p class="text-xs text-muted">Content Size</p>
        </div>
    </div>
    <div class="space-y-1.5">
        <div class="flex justify-between p-2 bg-sunken rounded border border-base">
            <span class="text-xs text-muted">Compression</span>
            <span class="text-xs font-mono text-primary-text">{data.compression}</span>
        </div>
        <div class="flex justify-between p-2 bg-sunken rounded border border-base">
            <span class="text-xs text-muted">Server</span>
            <span class="text-xs font-mono text-primary-text">{data.server}</span>
        </div>
        <div class="flex justify-between p-2 bg-sunken rounded border border-base">
            <span class="text-xs text-muted">Cache-Control</span>
            <span class="text-xs font-mono text-primary-text break-all max-w-[60%] text-right">{data.cache_control}</span>
        </div>
        <div class="flex justify-between p-2 bg-sunken rounded border border-base">
            <span class="text-xs text-muted">ETag</span>
            <span class="text-xs font-mono {data.etag ? 'text-green-400' : 'text-muted'}">{data.etag ? 'Present' : 'Missing'}</span>
        </div>
    </div>
  {/if}
</div>
