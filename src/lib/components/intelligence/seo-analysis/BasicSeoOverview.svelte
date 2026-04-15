<script lang="ts">
  import { Info } from 'lucide-svelte';
  import SeoGuide from '$lib/components/recon/guides/SeoGuide.svelte';
  import * as m from '$lib/paraglide/messages';
  import type { BasicSeoResult } from '$lib/types/intelligence';

  type Props = {
      data: BasicSeoResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let isGuideOpen = $state(false);

  function statusBadge(status: string): string {
      if (status === 'Good') return 'bg-green-500/10 text-green-400 border-green-500/30';
      if (status === 'Missing') return 'bg-red-500/10 text-red-400 border-red-500/30';
      return 'bg-yellow-500/10 text-yellow-400 border-yellow-500/30';
  }
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm h-full">
  <SeoGuide bind:isOpen={isGuideOpen} />
  
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent">{m.seo_basic_meta_data()}</h3>
      <button onclick={() => isGuideOpen = true} class="p-1 hover:bg-cyan-500/10 rounded-full text-accent transition-colors" title={m.secops_guide_title()}><Info class="size-4" /></button>
  </div>
  
  {#if isLoading}
    <div class="space-y-4 animate-pulse">
        <div class="h-10 bg-surface rounded"></div>
        <div class="h-16 bg-surface rounded"></div>
    </div>
  {:else if data}
    <div class="grid grid-cols-1 gap-3">
        <!-- Title -->
        <div class="p-3 bg-sunken rounded-lg border border-base">
            <div class="flex items-center justify-between mb-1">
                <p class="text-xs text-muted font-bold">{m.seo_page_title()}</p>
                <span class="px-1.5 py-0.5 text-xs rounded border {statusBadge(data.title.status)}">{data.title.status} ({data.title.length} chars)</span>
            </div>
            <p class="text-sm text-primary-text font-mono wrap-break-word">{data.title.text}</p>
        </div>
        
        <!-- Meta Description -->
        <div class="p-3 bg-sunken rounded-lg border border-base">
            <div class="flex items-center justify-between mb-1">
                <p class="text-xs text-muted font-bold">{m.seo_meta_desc()}</p>
                <span class="px-1.5 py-0.5 text-xs rounded border {statusBadge(data.meta_description.status)}">{data.meta_description.status} ({data.meta_description.length} chars)</span>
            </div>
            <p class="text-sm text-primary-text wrap-break-word">{data.meta_description.text}</p>
        </div>

        <!-- Keywords -->
        <div class="p-3 bg-sunken rounded-lg border border-base">
            <p class="text-xs text-muted font-bold mb-1">Meta Keywords</p>
            <p class="text-sm text-primary-text font-mono wrap-break-word">{data.meta_keywords}</p>
        </div>

        <!-- Technical Meta -->
        <div class="grid grid-cols-2 gap-2">
            <div class="p-2.5 bg-sunken rounded-lg border border-base">
                <p class="text-xs text-muted font-bold">Canonical</p>
                <p class="text-xs text-primary-text font-mono mt-1 break-all">{data.canonical_url}</p>
            </div>
            <div class="p-2.5 bg-sunken rounded-lg border border-base">
                <p class="text-xs text-muted font-bold">Robots</p>
                <p class="text-xs text-primary-text font-mono mt-1">{data.meta_robots}</p>
            </div>
            <div class="p-2.5 bg-sunken rounded-lg border border-base">
                <p class="text-xs text-muted font-bold">Viewport</p>
                <p class="text-xs text-primary-text font-mono mt-1 break-all">{data.viewport}</p>
            </div>
            <div class="p-2.5 bg-sunken rounded-lg border border-base">
                <p class="text-xs text-muted font-bold">Language</p>
                <p class="text-xs text-primary-text font-mono mt-1">{data.language}</p>
            </div>
        </div>
    </div>
  {:else}
      <div class="text-muted text-sm py-4">{m.seo_no_data()}</div>
  {/if}
</div>
