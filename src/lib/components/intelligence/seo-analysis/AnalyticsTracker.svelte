<script lang="ts">
  import { Activity } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';

  type Props = {
      data: Record<string, string> | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let detected = $derived(data ? Object.entries(data).filter(([, v]) => v === 'Found') : []);
  let notDetected = $derived(data ? Object.entries(data).filter(([, v]) => v !== 'Found') : []);
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <Activity class="size-5" /> {m.seo_analytics_title()}
  </h3>

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
  {/if}
</div>
