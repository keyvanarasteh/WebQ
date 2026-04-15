<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import type { SocialMediaResult } from '$lib/types/intelligence';

  type Props = {
      data: SocialMediaResult | undefined;
      isLoading: boolean;
  };
  
  let { data, isLoading }: Props = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4">{m.seo_social_title()}</h3>
  
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
      <div class="text-muted text-sm py-4">{m.seo_no_data()}</div>
  {/if}
</div>
