<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  type TechnicalSeoResult = {
      is_ssl: boolean;
      has_sitemap: boolean;
      has_robots_txt: boolean;
  };
  
  let { data, isLoading } = $props<{ data: TechnicalSeoResult | undefined, isLoading: boolean }>();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4">{m.seo_technical_title()}</h3>
  
  {#if isLoading}
    <div class="space-y-4 animate-pulse">
        <div class="h-10 bg-surface rounded"></div>
        <div class="h-10 bg-surface rounded"></div>
        <div class="h-10 bg-surface rounded"></div>
    </div>
  {:else if data}
    <div class="space-y-3">
        {#each [
            { label: m.seo_ssl_active(), active: data.is_ssl },
            { label: m.seo_sitemap_found(), active: data.has_sitemap },
            { label: m.seo_robots_present(), active: data.has_robots_txt }
        ] as metric, i (i)}
        <div class="flex items-center justify-between p-3 bg-background rounded-lg border border-base">
            <span class="text-sm font-medium text-primary-text">{metric.label}</span>
            {#if metric.active}
                <span class="w-6 h-6 rounded bg-green-500/20 text-green-500 flex items-center justify-center font-bold border border-green-500/20">✓</span>
            {:else}
                <span class="w-6 h-6 rounded bg-red-500/20 text-red-500 flex items-center justify-center font-bold border border-red-500/20">✗</span>
            {/if}
        </div>
        {/each}
    </div>
  {:else}
      <div class="text-muted text-sm py-4">{m.seo_no_data()}</div>
  {/if}
</div>
