<script lang="ts">
  import type { SeoScoreResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Award } from 'lucide-svelte';

  type Props = {
      data: SeoScoreResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();

  let gradeColor = $derived(
      data?.grade === 'A' ? 'text-green-400 border-green-500' :
      data?.grade === 'B' ? 'text-cyan-400 border-cyan-500' :
      data?.grade === 'C' ? 'text-yellow-400 border-yellow-500' :
      data?.grade === 'D' ? 'text-orange-400 border-orange-500' :
      'text-red-400 border-red-500'
  );
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <Award class="size-5" /> {m.seo_score_title()}
  </h3>

  {#if isLoading}
    <div class="flex justify-center animate-pulse"><div class="w-24 h-24 bg-surface rounded-full"></div></div>
  {:else if data}
    <div class="flex flex-col items-center">
        <div class="relative w-28 h-28 flex items-center justify-center rounded-full border-4 {gradeColor} mb-3">
            <div class="text-center">
                <span class="text-3xl font-black {gradeColor}">{data.grade}</span>
            </div>
        </div>
        <p class="text-2xl font-bold text-primary-text">{data.score}<span class="text-sm text-muted">/{data.max_score}</span></p>
        <p class="text-sm text-accent font-mono">{data.percentage}</p>
    </div>
  {/if}
</div>
