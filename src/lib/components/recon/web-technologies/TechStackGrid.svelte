<script lang="ts">
  type WebTechResult = {
      server: string | null;
      powered_by: string | null;
      framework: string | null;
      language: string | null;
  };
  
  let { data, isLoading } = $props<{ data: WebTechResult | undefined, isLoading: boolean }>();
</script>

<div class="bg-gray-50/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400 mb-4">Core Technology Stack</h3>
  
  {#if isLoading}
    <div class="grid grid-cols-2 gap-4 animate-pulse">
        <div class="h-16 bg-gray-200 dark:bg-[#27272a] rounded-lg"></div>
        <div class="h-16 bg-gray-200 dark:bg-[#27272a] rounded-lg"></div>
    </div>
  {:else if data}
    <div class="grid grid-cols-2 gap-4">
        {#each [
            { name: 'Web Server', value: data.server },
            { name: 'Powered By', value: data.powered_by },
            { name: 'Framework', value: data.framework },
            { name: 'Language', value: data.language }
        ] as tech, i (i)}
            {#if tech.value}
                <div class="p-3 bg-gray-50 dark:bg-[#121214] border border-gray-200 dark:border-[#27272a] rounded-lg flex flex-col justify-center">
                    <p class="text-[10px] text-gray-500 uppercase tracking-widest font-bold mb-1">{tech.name}</p>
                    <p class="text-sm text-gray-900 dark:text-cyan-50 font-mono tracking-tight">{tech.value}</p>
                </div>
            {/if}
        {/each}
        {#if !data.server && !data.powered_by && !data.framework && !data.language}
            <div class="col-span-2 text-center text-gray-500 py-4 h-full flex items-center justify-center">No core technologies fingerprinted</div>
        {/if}
    </div>
  {:else}
      <div class="text-gray-500 text-sm py-4">Awaiting scan data...</div>
  {/if}
</div>
