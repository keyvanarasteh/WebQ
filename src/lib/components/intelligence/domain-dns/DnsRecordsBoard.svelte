<script lang="ts">
  import { HelpCircle } from 'lucide-svelte';
  import DnsRecordsBoardGuide from './DnsRecordsBoardGuide.svelte';
  import * as m from '$lib/paraglide/messages';
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
  
  let isGuideOpen = $state(false);
  let activeTab = $state('A');
  const tabs = ['A', 'AAAA', 'MX', 'NS', 'SOA', 'TXT', 'CNAME'];
  let activeRecords = $derived(records ? (records[activeTab.toLowerCase() as keyof DnsRecordsMap] || []) : []);
</script>

<DnsRecordsBoardGuide bind:isOpen={isGuideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm transition-all h-full">
  <div class="flex items-center justify-between mb-4 border-b border-base pb-2">
      <h3 class="text-lg font-bold text-accent">{m.dns_records_board_title()}</h3>
      <button onclick={() => isGuideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_dns_records_title()}><HelpCircle class="size-4" /></button>
  </div>

  {#if isLoading}
      <div class="flex items-center justify-center h-48">
          <div class="w-12 h-12 border-4 border-base border-t-cyan-500 rounded-full animate-spin"></div>
      </div>
  {:else if records}
      <!-- Tabs Header -->
      <div class="flex flex-wrap gap-2 mb-4">
          {#each tabs as tab (tab)}
              {@const count = records[tab.toLowerCase() as keyof DnsRecordsMap]?.length || 0}
              <button 
                  class="px-4 py-1.5 text-sm font-medium rounded-md transition-all {activeTab === tab ? 'bg-cyan-500/10 text-accent border border-cyan-500/30' : 'text-muted hover:text-primary-text hover:bg-surface/50'}"
                  onclick={() => activeTab = tab}
              >
                  {tab}
                  <!-- Badge for count -->
                  {#if count > 0}
                      <span class="ml-2 text-xs bg-cyan-500/20 text-cyan-300 px-1.5 rounded-full">{count}</span>
                  {/if}
              </button>
          {/each}
      </div>

      <!-- Tab Content Area -->
      <div class="bg-sunken border border-base rounded-lg p-4 min-h-[300px] overflow-y-auto max-h-[500px]">
          {#if activeRecords.length > 0}
              <div class="flex flex-col gap-2">
                  {#each activeRecords as record, i (i)}
                      <div class="p-3 bg-glass border border-base rounded-md font-mono text-sm text-primary-text break-all select-all flex items-start gap-3">
                          <span class="text-cyan-600 select-none">{i + 1}.</span>
                          <span class="text-cyan-100">{record}</span>
                      </div>
                  {/each}
              </div>
          {:else}
              <div class="flex items-center justify-center h-full text-muted mt-10">
                  <p>{m.dns_records_empty({ tab: activeTab })}</p>
              </div>
          {/if}
      </div>
  {:else}
      <div class="flex items-center justify-center h-48 text-muted">
          <p>{m.dns_records_pending()}</p>
      </div>
  {/if}
</div>
