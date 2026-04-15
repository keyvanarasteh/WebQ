<script lang="ts">
  import type { LinkAnalysisResult } from '$lib/types/intelligence';
  import * as m from '$lib/paraglide/messages';
  import { Link } from 'lucide-svelte';

  type Props = {
      data: LinkAnalysisResult | undefined;
      isLoading: boolean;
  };

  let { data, isLoading }: Props = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <Link class="size-5" /> {m.seo_links_title()}
  </h3>

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
  {/if}
</div>
