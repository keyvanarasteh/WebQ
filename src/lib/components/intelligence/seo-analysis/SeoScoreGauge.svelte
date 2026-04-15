<script lang="ts">
  import type { SeoScoreResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Award, HelpCircle } from 'lucide-svelte';
  import SeoScoreGuide from './SeoScoreGuide.svelte';

  type Props = {
      data: SeoScoreResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);

  let gradeColor = $derived(
      data?.grade === 'A' ? 'text-green-400 border-green-500' :
      data?.grade === 'B' ? 'text-cyan-400 border-cyan-500' :
      data?.grade === 'C' ? 'text-yellow-400 border-yellow-500' :
      data?.grade === 'D' ? 'text-orange-400 border-orange-500' :
      'text-red-400 border-red-500'
  );
</script>

<SeoScoreGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><Award class="size-5" /> {m.seo_score_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_seo_score_title()}><HelpCircle class="size-4" /></button>
  </div>

  {#if isLoading}
    <div class="flex justify-center animate-pulse"><div class="w-24 h-24 bg-surface rounded-full"></div></div>
  {:else if data}
    <div class="flex flex-col items-center">
        <div class="relative w-28 h-28 flex items-center justify-center rounded-full border-4 {gradeColor} mb-3">
            <span class="text-3xl font-black {gradeColor}">{data.grade}</span>
        </div>
        <p class="text-2xl font-bold text-primary-text">{data.score}<span class="text-sm text-muted">/{data.max_score}</span></p>
        <p class="text-sm text-accent font-mono">{data.percentage}</p>
    </div>
  {:else}
    <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
        <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
        <Award class="size-8 text-muted/30" />
        <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
    </div>
  {/if}
</div>
