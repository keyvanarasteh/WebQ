<script lang="ts">
  import { Activity, HelpCircle } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import AnalyticsGuide from './AnalyticsGuide.svelte';

  type Props = {
      data: Record<string, string> | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);
  let detected = $derived(data ? Object.entries(data).filter(([, v]) => v === 'Found') : []);
  let notDetected = $derived(data ? Object.entries(data).filter(([, v]) => v !== 'Found') : []);
</script>

<AnalyticsGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><Activity class="size-5" /> {m.seo_analytics_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_analytics_title()}><HelpCircle class="size-4" /></button>
  </div>

  {#if isLoading}
    <div class="h-24 bg-surface rounded animate-pulse"></div>
  {:else if data}
    {#if detected.length > 0}
        <p class="text-xs text-muted font-bold uppercase mb-2">Detected ({detected.length})</p>
        <div class="flex flex-wrap gap-2 mb-4">
            {#each detected as [name] (name)}
                <span class="text-xs font-mono px-2 py-1 rounded bg-green-500/10 text-green-400 border border-green-500/20">{name}</span>
            {/each}
        </div>
    {/if}
    {#if notDetected.length > 0}
        <p class="text-xs text-muted font-bold uppercase mb-2">Not Detected ({notDetected.length})</p>
        <div class="flex flex-wrap gap-1.5">
            {#each notDetected as [name] (name)}
                <span class="text-xs font-mono px-2 py-1 rounded bg-surface text-muted border border-base">{name}</span>
            {/each}
        </div>
    {/if}
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <Activity class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
