<script lang="ts">
  type DnsRecordsMap = {
      txt: string[];
  };

  type Props = {
      records: DnsRecordsMap | undefined;
      isLoading: boolean;
  };

  let { records, isLoading }: Props = $props();

  let txtRecords = $derived(records?.txt || []);
  let hasSpf = $derived(txtRecords.some((r: string) => r.toLowerCase().includes('v=spf1')));
  let hasDmarc = $derived(txtRecords.some((r: string) => r.toLowerCase().includes('v=dmarc1')));
</script>

<div class="bg-white/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl p-6 shadow-sm dark:shadow-md transition-all">
  <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400 mb-6">Security Policies</h3>

  {#if isLoading}
      <div class="space-y-4">
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-800/50 rounded-lg animate-pulse"></div>
          <div class="h-10 w-full bg-gray-200 dark:bg-gray-800/50 rounded-lg animate-pulse"></div>
      </div>
  {:else if records}
      <div class="space-y-4">
          <!-- SPF Check -->
          <div class="flex items-center justify-between p-3 bg-[#121214] rounded-lg border border-[#27272a]">
              <div>
                  <p class="text-sm font-bold text-white">SPF Record</p>
                  <p class="text-xs text-gray-500 mt-1">Sender Policy Framework</p>
              </div>
              {#if hasSpf}
                  <span class="px-2 py-1 bg-green-500/10 text-green-400 rounded-md border border-green-500/30 text-xs font-bold tracking-widest">DETECTED</span>
              {:else}
                  <span class="px-2 py-1 bg-red-500/10 text-red-500 rounded-md border border-red-500/30 text-xs font-bold tracking-widest animate-pulse">MISSING</span>
              {/if}
          </div>

          <!-- DMARC Check -->
          <div class="flex items-center justify-between p-3 bg-[#121214] rounded-lg border border-[#27272a]">
              <div>
                  <p class="text-sm font-bold text-white">DMARC Record</p>
                  <p class="text-xs text-gray-500 mt-1">Domain Message Auth</p>
              </div>
              {#if hasDmarc}
                  <span class="px-2 py-1 bg-green-500/10 text-green-400 rounded-md border border-green-500/30 text-xs font-bold tracking-widest">DETECTED</span>
              {:else}
                  <span class="px-2 py-1 bg-red-500/10 text-red-500 rounded-md border border-red-500/30 text-xs font-bold tracking-widest">MISSING</span>
              {/if}
          </div>
      </div>
  {:else}
      <div class="text-sm text-gray-500 text-center py-6">
          Awaiting input data.
      </div>
  {/if}
</div>
