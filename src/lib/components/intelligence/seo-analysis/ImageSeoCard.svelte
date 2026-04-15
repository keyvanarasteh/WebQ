<script lang="ts">
  import type { ImageSeoResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Image, HelpCircle } from 'lucide-svelte';
  import ImageSeoGuide from './ImageSeoGuide.svelte';

  type Props = {
      data: ImageSeoResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);
</script>

<ImageSeoGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><Image class="size-5" /> {m.seo_image_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_images_title()}><HelpCircle class="size-4" /></button>
  </div>

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
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <Image class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
