<script lang="ts">
  type TechnicalSeoResult = {
      is_ssl: boolean;
      has_sitemap: boolean;
      has_robots_txt: boolean;
  };
  
  let { data, isLoading } = $props<{ data: TechnicalSeoResult | undefined, isLoading: boolean }>();
</script>

<div class="bg-white/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400 mb-4">Technical SEO</h3>
  
  {#if isLoading}
    <div class="space-y-4 animate-pulse">
        <div class="h-10 bg-gray-200 dark:bg-[#27272a] rounded"></div>
        <div class="h-10 bg-gray-200 dark:bg-[#27272a] rounded"></div>
        <div class="h-10 bg-gray-200 dark:bg-[#27272a] rounded"></div>
    </div>
  {:else if data}
    <div class="space-y-3">
        {#each [
            { label: 'SSL Active / HTTPS', active: data.is_ssl },
            { label: 'Sitemap.xml Found', active: data.has_sitemap },
            { label: 'Robots.txt Present', active: data.has_robots_txt }
        ] as metric, i (i)}
        <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-[#121214] rounded-lg border border-gray-200 dark:border-[#27272a]">
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{metric.label}</span>
            {#if metric.active}
                <span class="w-6 h-6 rounded bg-green-500/20 text-green-500 dark:text-green-400 flex items-center justify-center font-bold border border-green-500/20">✓</span>
            {:else}
                <span class="w-6 h-6 rounded bg-red-500/20 text-red-500 flex items-center justify-center font-bold border border-red-500/20">✗</span>
            {/if}
        </div>
        {/each}
    </div>
  {:else}
      <div class="text-gray-500 text-sm py-4">No data available</div>
  {/if}
</div>
