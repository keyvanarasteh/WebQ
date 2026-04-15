<script lang="ts">
  import type { PageSpeedResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Zap, HelpCircle } from 'lucide-svelte';
  import PageSpeedGuide from './PageSpeedGuide.svelte';

  type Props = {
      data: PageSpeedResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);
</script>

<PageSpeedGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><Zap class="size-5" /> {m.seo_speed_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_page_speed_title()}><HelpCircle class="size-4" /></button>
  </div>

  {#if isLoading}
    <div class="h-20 bg-surface rounded animate-pulse"></div>
  {:else if data}
    <div class="grid grid-cols-2 gap-2 mb-3">
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-primary-text">{data.css_files}</p>
            <p class="text-xs text-muted">CSS Files</p>
        </div>
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-primary-text">{data.js_files}</p>
            <p class="text-xs text-muted">JS Files</p>
        </div>
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-yellow-400">{data.inline_styles}</p>
            <p class="text-xs text-muted">Inline Styles</p>
        </div>
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-yellow-400">{data.inline_scripts}</p>
            <p class="text-xs text-muted">Inline Scripts</p>
        </div>
    </div>
    <div class="flex justify-between p-2.5 bg-sunken rounded-lg border border-base">
        <span class="text-sm text-primary-text">Compression</span>
        <span class="text-sm font-mono {data.compression !== 'None' ? 'text-green-400' : 'text-red-400'}">{data.compression}</span>
    </div>
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <Zap class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
