<script lang="ts">
  import type { SchemaMarkupResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Code, HelpCircle } from 'lucide-svelte';
  import SchemaMarkupGuide from './SchemaMarkupGuide.svelte';

  type Props = {
      data: SchemaMarkupResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
  let guideOpen = $state(false);
</script>

<SchemaMarkupGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2"><Code class="size-5" /> {m.seo_schema_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_schema_title()}><HelpCircle class="size-4" /></button>
  </div>

  {#if isLoading}
    <div class="h-20 bg-surface rounded animate-pulse"></div>
  {:else if data}
    <div class="grid grid-cols-3 gap-2 mb-4">
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-accent">{data.json_ld_count}</p>
            <p class="text-xs text-muted">JSON-LD</p>
        </div>
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-primary-text">{data.microdata_items}</p>
            <p class="text-xs text-muted">Microdata</p>
        </div>
        <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
            <p class="text-xl font-bold text-green-400">{data.total_structured_data}</p>
            <p class="text-xs text-muted">Total</p>
        </div>
    </div>

    {#if data.json_ld_types.length > 0}
        <p class="text-xs text-muted font-bold uppercase mb-2">Schema Types</p>
        <div class="flex flex-wrap gap-1.5">
            {#each data.json_ld_types as schemaType (schemaType)}
                <span class="text-xs font-mono px-2 py-0.5 rounded bg-cyan-500/10 text-accent border border-cyan-500/20">{schemaType}</span>
            {/each}
        </div>
    {/if}
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <Code class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
