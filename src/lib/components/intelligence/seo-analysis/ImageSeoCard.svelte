<script lang="ts">
  import type { ImageSeoResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Image } from 'lucide-svelte';

  type Props = {
      data: ImageSeoResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <Image class="size-5" /> {m.seo_image_title()}
  </h3>

  {#if isLoading}
    <div class="h-16 bg-surface rounded animate-pulse"></div>
  {:else if data}
    <div class="space-y-2">
        <div class="flex justify-between p-2.5 bg-sunken rounded-lg border border-base">
            <span class="text-sm text-primary-text">Total Images</span>
            <span class="text-sm font-mono text-accent">{data.total_images}</span>
        </div>
        <div class="flex justify-between p-2.5 bg-sunken rounded-lg border border-base">
            <span class="text-sm text-primary-text">Lazy Loaded</span>
            <span class="text-sm font-mono text-primary-text">{data.lazy_loaded}</span>
        </div>
        <div class="flex justify-between p-2.5 bg-sunken rounded-lg border border-base">
            <span class="text-sm text-primary-text">With Alt Text</span>
            <span class="text-sm font-mono {data.with_alt_text === data.total_images ? 'text-green-400' : 'text-yellow-400'}">{data.with_alt_text}</span>
        </div>
        <div class="flex justify-between p-2.5 bg-sunken rounded-lg border border-base">
            <span class="text-sm text-primary-text">With Title</span>
            <span class="text-sm font-mono text-primary-text">{data.with_title}</span>
        </div>
        <div class="flex justify-between p-2.5 bg-sunken rounded-lg border border-base">
            <span class="text-sm text-primary-text">Score</span>
            <span class="text-sm font-bold text-accent">{data.optimization_score}</span>
        </div>
    </div>
  {/if}
</div>
