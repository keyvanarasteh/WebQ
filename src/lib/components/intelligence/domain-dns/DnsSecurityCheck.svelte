<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import { HelpCircle } from 'lucide-svelte';
  import DnsSecurityCheckGuide from './DnsSecurityCheckGuide.svelte';
  type DnsRecordsMap = {
      txt: string[];
  };

  type Props = {
      records: DnsRecordsMap | undefined;
      isLoading: boolean;
  };

  let { records, isLoading }: Props = $props();
  let guideOpen = $state(false);

  let txtRecords = $derived(records?.txt || []);
  let hasSpf = $derived(txtRecords.some((r: string) => r.toLowerCase().includes('v=spf1')));
  let hasDmarc = $derived(txtRecords.some((r: string) => r.toLowerCase().includes('v=dmarc1')));
</script>

<DnsSecurityCheckGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm transition-all">
  <div class="flex items-center justify-between mb-6">
      <h3 class="text-lg font-bold text-accent">{m.dns_security_policies_title()}</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_dns_security_title()}>
          <HelpCircle class="size-4" />
      </button>
  </div>

  {#if isLoading}
      <div class="space-y-4">
          <div class="h-10 w-full bg-surface/50 rounded-lg animate-pulse"></div>
          <div class="h-10 w-full bg-surface/50 rounded-lg animate-pulse"></div>
      </div>
  {:else if records}
      <div class="space-y-4">
          <!-- SPF Check -->
          <div class="flex items-center justify-between p-3 bg-sunken rounded-lg border border-base">
              <div>
                  <p class="text-sm font-bold text-primary-text">{m.dns_spf_record()}</p>
                  <p class="text-xs text-muted mt-1">{m.dns_spf_desc()}</p>
              </div>
              {#if hasSpf}
                  <span class="px-2 py-1 bg-green-500/10 text-green-400 rounded-md border border-green-500/30 text-xs font-bold tracking-widest">{m.dns_detected()}</span>
              {:else}
                  <span class="px-2 py-1 bg-red-500/10 text-red-500 rounded-md border border-red-500/30 text-xs font-bold tracking-widest animate-pulse">{m.dns_missing()}</span>
              {/if}
          </div>

          <!-- DMARC Check -->
          <div class="flex items-center justify-between p-3 bg-sunken rounded-lg border border-base">
              <div>
                  <p class="text-sm font-bold text-primary-text">{m.dns_dmarc_record()}</p>
                  <p class="text-xs text-muted mt-1">{m.dns_dmarc_desc()}</p>
              </div>
              {#if hasDmarc}
                  <span class="px-2 py-1 bg-green-500/10 text-green-400 rounded-md border border-green-500/30 text-xs font-bold tracking-widest">{m.dns_detected()}</span>
              {:else}
                  <span class="px-2 py-1 bg-red-500/10 text-red-500 rounded-md border border-red-500/30 text-xs font-bold tracking-widest">{m.dns_missing()}</span>
              {/if}
          </div>
      </div>
  {:else}
      <div class="text-sm text-muted text-center py-6">
          {m.dns_awaiting_data()}
      </div>
  {/if}
</div>
