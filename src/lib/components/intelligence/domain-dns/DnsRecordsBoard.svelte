<script lang="ts">
  type DnsRecordsMap = {
      a: string[];
      aaaa: string[];
      mx: string[];
      ns: string[];
      soa: string[];
      txt: string[];
      cname: string[];
  };

  type Props = {
      records: DnsRecordsMap | undefined;
      isLoading: boolean;
  };

  let { records, isLoading }: Props = $props();
  
  let activeTab = $state('A');
  const tabs = ['A', 'AAAA', 'MX', 'NS', 'SOA', 'TXT', 'CNAME'];
</script>

<div class="bg-white/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm dark:shadow-md transition-all h-full">
  <div class="flex items-center justify-between mb-4 border-b border-[#27272a] pb-2">
      <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400">DNS Records</h3>
  </div>

  {#if isLoading}
      <div class="flex items-center justify-center h-48">
          <div class="w-12 h-12 border-4 border-gray-200 dark:border-gray-800 border-t-cyan-500 rounded-full animate-spin"></div>
      </div>
  {:else if records}
      <!-- Tabs Header -->
      <div class="flex flex-wrap gap-2 mb-4">
          {#each tabs as tab (tab)}
              <button 
                  class="px-4 py-1.5 text-sm font-medium rounded-md transition-all {activeTab === tab ? 'bg-cyan-500/10 text-cyan-400 border border-cyan-500/30' : 'text-gray-500 hover:text-gray-300 hover:bg-[#27272a]/50'}"
                  onclick={() => activeTab = tab}
              >
                  {tab}
                  <!-- Badge for count -->
                  {@const count = records[tab.toLowerCase() as keyof DnsRecordsMap]?.length || 0}
                  {#if count > 0}
                      <span class="ml-2 text-xs bg-cyan-500/20 text-cyan-300 px-1.5 rounded-full">{count}</span>
                  {/if}
              </button>
          {/each}
      </div>

      <!-- Tab Content Area -->
      <div class="bg-[#121214] border border-[#27272a] rounded-lg p-4 min-h-[300px] overflow-y-auto max-h-[500px]">
          {@const activeRecords = records[activeTab.toLowerCase() as keyof DnsRecordsMap] || []}
          
          {#if activeRecords.length > 0}
              <div class="flex flex-col gap-2">
                  {#each activeRecords as record, i (i)}
                      <div class="p-3 bg-white/5 dark:bg-[#18181b] border border-[#27272a] rounded-md font-mono text-sm text-gray-300 break-all select-all flex items-start gap-3">
                          <span class="text-cyan-600 select-none">{i + 1}.</span>
                          <span class="text-cyan-100">{record}</span>
                      </div>
                  {/each}
              </div>
          {:else}
              <div class="flex items-center justify-center h-full text-gray-600 mt-10">
                  <p>No {activeTab} records found.</p>
              </div>
          {/if}
      </div>
  {:else}
      <div class="flex items-center justify-center h-48 text-gray-600">
          <p>Execute scan to fetch DNS records.</p>
      </div>
  {/if}
</div>
