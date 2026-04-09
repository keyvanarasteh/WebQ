<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  type SocialMediaResult = {
      og_title: string;
      og_description: string;
      og_image: string;
      twitter_card: string;
  };
  
  let { data, isLoading } = $props<{ data: SocialMediaResult | undefined, isLoading: boolean }>();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4">{m.seo_social_title()}</h3>
  
  {#if isLoading}
    <div class="h-32 bg-surface rounded animate-pulse"></div>
  {:else if data}
    <div class="grid grid-cols-1 gap-4">
        <!-- OpenGraph Module -->
        <div class="p-4 bg-background border border-base rounded-lg hover:border-cyan-500/50 transition-colors">
            <h4 class="text-xs font-bold text-muted mb-3 uppercase tracking-wider">{m.seo_og_tags()}</h4>
            <div class="space-y-2">
                <p class="text-sm text-primary-text break-words"><span class="text-cyan-600 text-accent font-mono mr-1">og:title:</span> {data.og_title || 'N/A'}</p>
                <p class="text-sm text-primary-text break-words"><span class="text-cyan-600 text-accent font-mono mr-1">og:description:</span> {data.og_description || 'N/A'}</p>
                <p class="text-sm text-primary-text break-all"><span class="text-cyan-600 text-accent font-mono mr-1">og:image:</span> {data.og_image || 'N/A'}</p>
            </div>
        </div>

        <!-- Twitter Card -->
        <div class="p-4 bg-background border border-base rounded-lg border-l-4 {data.twitter_card ? 'border-l-blue-500' : 'border-l-red-500'}">
            <h4 class="text-xs font-bold text-muted mb-2 uppercase tracking-wider">{m.seo_twitter_arch()}</h4>
            <p class="text-sm text-inverse text-primary-text font-mono">{data.twitter_card || m.seo_missing_twitter_card()}</p>
        </div>
    </div>
  {:else}
      <div class="text-muted text-sm py-4">{m.seo_no_data()}</div>
  {/if}
</div>
