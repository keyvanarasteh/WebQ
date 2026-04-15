<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import type { SocialMediaResult } from '$lib/types/intelligence';
  import { HelpCircle, Share2 } from 'lucide-svelte';
  import SocialMediaGuide from './SocialMediaGuide.svelte';

  type Props = {
      data: SocialMediaResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);
</script>

<SocialMediaGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><Share2 class="size-5" /> {m.seo_social_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_social_media_title()}><HelpCircle class="size-4" /></button>
  </div>
  
  {#if isLoading}
    <div class="h-32 bg-surface rounded animate-pulse"></div>
  {:else if data}
    <div class="grid grid-cols-1 gap-4">
        <!-- OpenGraph -->
        <div class="p-4 bg-sunken border border-base rounded-lg">
            <h4 class="text-xs font-bold text-muted mb-3 uppercase tracking-wider">{m.seo_og_tags()}</h4>
            <div class="space-y-2">
                {#each Object.entries(data.open_graph) as [key, value] (key)}
                    <div class="flex items-start gap-2">
                        <span class="text-xs text-accent font-mono shrink-0">{key}:</span>
                        <span class="text-sm text-primary-text wrap-break-word {value === 'Not Found' ? 'text-red-400 italic' : ''}">{value}</span>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Twitter Cards -->
        <div class="p-4 bg-sunken border border-base rounded-lg">
            <h4 class="text-xs font-bold text-muted mb-3 uppercase tracking-wider">{m.seo_twitter_arch()}</h4>
            <div class="space-y-2">
                {#each Object.entries(data.twitter_cards) as [key, value] (key)}
                    <div class="flex items-start gap-2">
                        <span class="text-xs text-blue-400 font-mono shrink-0">{key}:</span>
                        <span class="text-sm text-primary-text wrap-break-word {value === 'Not Found' ? 'text-red-400 italic' : ''}">{value}</span>
                    </div>
                {/each}
            </div>
        </div>
    </div>
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <Share2 class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
