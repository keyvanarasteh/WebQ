<script lang="ts">
  import type { PageSpeedResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Zap } from 'lucide-svelte';

  type Props = {
      data: PageSpeedResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <Zap class="size-5" /> {m.seo_speed_title()}
  </h3>

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
  {/if}
</div>
