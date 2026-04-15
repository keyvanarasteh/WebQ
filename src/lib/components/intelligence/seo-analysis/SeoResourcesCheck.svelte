<script lang="ts">
  import { FileSearch, Check, X, HelpCircle } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import SeoResourcesGuide from './SeoResourcesGuide.svelte';

  type Props = {
      data: Record<string, string> | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);
</script>

<SeoResourcesGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><FileSearch class="size-5" /> {m.seo_resources_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_seo_resources_title()}><HelpCircle class="size-4" /></button>
  </div>

  {#if isLoading}
    <div class="space-y-2 animate-pulse">
        {#each Array(4) as _, i (i)}<div class="h-10 bg-surface rounded"></div>{/each}
    </div>
  {:else if data}
    <div class="space-y-1.5">
        {#each Object.entries(data) as [file, status] (file)}
            <div class="flex items-center justify-between p-2.5 bg-sunken rounded-lg border border-base">
                <span class="text-sm font-mono text-primary-text">{file}</span>
                {#if status === 'Found'}
                    <Check class="size-4 text-green-400" />
                {:else}
                    <X class="size-4 text-red-400" />
                {/if}
            </div>
        {/each}
    </div>
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <FileSearch class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
