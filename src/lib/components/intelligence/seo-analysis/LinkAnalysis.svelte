<script lang="ts">
  import type { LinkAnalysisResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Link, HelpCircle } from 'lucide-svelte';
  import LinkAnalysisGuide from './LinkAnalysisGuide.svelte';

  type Props = {
      data: LinkAnalysisResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);
</script>

<LinkAnalysisGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><Link class="size-5" /> {m.seo_links_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_links_title()}><HelpCircle class="size-4" /></button>
  </div>

  {#if isLoading}
    <div class="h-16 bg-surface rounded animate-pulse"></div>
  {:else if data}
    <div class="grid grid-cols-2 gap-2">
        <div class="p-3 bg-sunken rounded-lg border border-base text-center">
            <p class="text-2xl font-bold text-primary-text">{data.total_links}</p>
            <p class="text-xs text-muted">Total Links</p>
        </div>
        <div class="p-3 bg-sunken rounded-lg border border-base text-center">
            <p class="text-2xl font-bold text-cyan-400">{data.internal_links}</p>
            <p class="text-xs text-muted">Internal</p>
        </div>
        <div class="p-3 bg-sunken rounded-lg border border-base text-center">
            <p class="text-2xl font-bold text-yellow-400">{data.external_links}</p>
            <p class="text-xs text-muted">External</p>
        </div>
        <div class="p-3 bg-sunken rounded-lg border border-base text-center">
            <p class="text-2xl font-bold {data.nofollow_links > 0 ? 'text-red-400' : 'text-muted'}">{data.nofollow_links}</p>
            <p class="text-xs text-muted">Nofollow</p>
        </div>
    </div>
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <Link class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
