<script lang="ts">
  type BasicSeoResult = {
      title: string;
      description: string;
      keywords: string;
      h1_count: number;
      heading_structure: boolean;
  };

  let { data, isLoading } = $props<{ data: BasicSeoResult | undefined, isLoading: boolean }>();
</script>

<div class="bg-white/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm h-full">
  <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400 mb-4">Basic Meta Data</h3>
  
  {#if isLoading}
    <div class="space-y-4 animate-pulse">
        <div class="h-10 bg-gray-200 dark:bg-[#27272a] rounded"></div>
        <div class="h-16 bg-gray-200 dark:bg-[#27272a] rounded"></div>
    </div>
  {:else if data}
    <div class="grid grid-cols-1 gap-4">
        <!-- Title -->
        <div class="p-3 bg-gray-50 dark:bg-[#121214] rounded-lg border border-gray-200 dark:border-[#27272a]">
            <p class="text-xs text-gray-500 font-bold mb-1">Page Title</p>
            <p class="text-sm {data.title ? 'text-gray-900 dark:text-white' : 'text-red-500'}">{data.title || 'Missing title tag'}</p>
        </div>
        
        <!-- Description -->
        <div class="p-3 bg-gray-50 dark:bg-[#121214] rounded-lg border border-gray-200 dark:border-[#27272a]">
            <p class="text-xs text-gray-500 font-bold mb-1">Meta Description</p>
            <p class="text-sm {data.description ? 'text-gray-900 dark:text-white' : 'text-red-500'}">{data.description || 'Missing meta description'}</p>
        </div>

        <!-- H1 Tags -->
        <div class="p-3 bg-gray-50 dark:bg-[#121214] rounded-lg border border-gray-200 dark:border-[#27272a] flex justify-between items-center">
            <div>
                <p class="text-xs text-gray-500 font-bold mb-1">H1 Tags</p>
                <p class="text-sm text-gray-900 dark:text-white">Count: {data.h1_count}</p>
            </div>
            {#if data.h1_count === 1}
                <span class="px-2 py-1 bg-green-500/10 text-green-500 dark:text-green-400 text-xs rounded border border-green-500/30">OPTIMAL</span>
            {:else if data.h1_count > 1}
                <span class="px-2 py-1 bg-yellow-500/10 text-yellow-600 dark:text-yellow-500 text-xs rounded border border-yellow-500/30">MULTIPLE</span>
            {:else}
                <span class="px-2 py-1 bg-red-500/10 text-red-500 text-xs rounded border border-red-500/30">MISSING</span>
            {/if}
        </div>
    </div>
  {:else}
      <div class="text-gray-500 text-sm py-4">No data available</div>
  {/if}
</div>
