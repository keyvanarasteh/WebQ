<script lang="ts">
  type WpUser = { id: number, name: string, slug: string };
  type WordPressAnalysis = {
      is_wordpress: boolean;
      version: string | null;
      themes: string[];
      plugins: string[];
      users: WpUser[];
  };
  
  let { data, isLoading } = $props<{ data: WordPressAnalysis | undefined, isLoading: boolean }>();
</script>

<div class="bg-gray-50/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400">CMS Fingerprinting</h3>
      {#if data?.is_wordpress}
          <span class="px-2 py-1 bg-blue-500/10 text-blue-500 dark:text-blue-400 border border-blue-500/30 rounded-md text-[10px] font-black tracking-widest animate-pulse">WP DETECTED</span>
      {/if}
  </div>
  
  {#if isLoading}
    <div class="h-32 bg-gray-200 dark:bg-[#27272a] rounded animate-pulse"></div>
  {:else if data}
      {#if data.is_wordpress}
          <div class="space-y-4">
              <div class="p-4 bg-gray-50 dark:bg-[#121214] border border-gray-200 dark:border-[#27272a] rounded-lg border-l-4 border-l-blue-500">
                  <p class="text-xs text-gray-500 font-bold mb-1">Version Detection</p>
                  <p class="text-sm font-mono text-gray-900 dark:text-gray-300">{data.version || 'Hardened / Version hidden'}</p>
              </div>

              {#if data.users.length > 0}
                  <div class="p-4 bg-gray-50 dark:bg-[#121214] border border-gray-200 dark:border-[#27272a] rounded-lg">
                      <p class="text-xs text-red-500 font-bold mb-2 uppercase tracking-wide">Exposed Users (REST API VULN)</p>
                      <ul class="space-y-1">
                          {#each data.users as user (user.id)}
                              <li class="text-xs md:text-sm text-gray-900 dark:text-gray-300 border-b border-gray-200 dark:border-[#27272a] pb-1 font-mono last:border-0 last:pb-0">
                                  ID: {user.id} <span class="mx-2 text-gray-400">|</span> <span class="text-cyan-600 dark:text-cyan-500">{user.name}</span> <span class="text-gray-500">(@{user.slug})</span>
                              </li>
                          {/each}
                      </ul>
                  </div>
              {/if}
          </div>
      {:else}
          <div class="text-gray-500 text-sm py-4 flex items-center justify-center border border-gray-200 dark:border-[#27272a] rounded-lg bg-gray-50 dark:bg-[#121214] min-h-[140px]">
              No WordPress CMS identified on target host.
          </div>
      {/if}
  {:else}
      <div class="text-gray-500 text-sm py-4">Awaiting scan data...</div>
  {/if}
</div>
