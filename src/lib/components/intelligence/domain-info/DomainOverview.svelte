<script lang="ts">
  import type { DomainInfoResult } from '$lib/types/intelligence';
  import { Globe, Server, Shield, Clock, Copy, Check, HelpCircle } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import DomainOverviewGuide from './DomainOverviewGuide.svelte';

  type Props = {
      isLoading: boolean;
      result: DomainInfoResult | null;
  };

  let { isLoading, result }: Props = $props();
  let copiedField = $state<string | null>(null);
  let guideOpen = $state(false);

  function copyToClipboard(value: string, fieldId: string) {
      navigator.clipboard.writeText(value);
      copiedField = fieldId;
      setTimeout(() => copiedField = null, 1500);
  }
</script>

<DomainOverviewGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm h-full relative overflow-hidden transition-all duration-300">
  {#if isLoading}
      <div class="animate-pulse space-y-6">
          <div class="h-6 w-1/3 bg-surface rounded"></div>
          <div class="grid grid-cols-2 gap-4 mt-4">
              {#each Array(8) as _, i (i)}
                  <div class="h-16 w-full bg-surface/50 rounded-lg"></div>
              {/each}
          </div>
      </div>
  {:else if result}
      <div class="flex items-center justify-between mb-5">
          <h3 class="text-lg font-bold text-accent flex items-center gap-2">
              <Globe class="size-5" />
              Infrastructure Overview
          </h3>
          <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_domain_overview_title()}>
              <HelpCircle class="size-4" />
          </button>
      </div>

      <!-- Network Section -->
      <div class="mb-5">
          <p class="text-xs text-muted uppercase tracking-widest font-bold mb-3 flex items-center gap-1.5">
              <Server class="size-3.5" /> Network
          </p>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
              <!-- IPv4 -->
              {#if result.ipv4}
                  <button class="p-3 rounded-lg bg-surface border border-base text-left hover:border-cyan-500/50 transition-colors group" onclick={() => result?.ipv4 && copyToClipboard(result.ipv4, 'ipv4')}>
                      <p class="text-xs text-muted uppercase font-semibold">IPv4</p>
                      <p class="text-sm text-primary-text mt-1 font-mono flex items-center gap-1.5">
                          {result.ipv4}
                          {#if copiedField === 'ipv4'}<Check class="size-3 text-green-400" />{:else}<Copy class="size-3 text-muted opacity-0 group-hover:opacity-100 transition-opacity" />{/if}
                      </p>
                  </button>
              {/if}

              <!-- All IPv4 -->
              {#if result.all_ipv4.length > 1}
                  <div class="p-3 rounded-lg bg-surface border border-base">
                      <p class="text-xs text-muted uppercase font-semibold">All IPv4</p>
                      <div class="flex flex-wrap gap-1 mt-1">
                          {#each result.all_ipv4 as ip (ip)}
                              <span class="text-xs font-mono text-accent bg-cyan-500/10 px-1.5 py-0.5 rounded">{ip}</span>
                          {/each}
                      </div>
                  </div>
              {/if}

              <!-- IPv6 -->
              {#if result.ipv6.length > 0}
                  <div class="p-3 rounded-lg bg-surface border border-base col-span-1 md:col-span-2">
                      <p class="text-xs text-muted uppercase font-semibold">IPv6</p>
                      <div class="flex flex-col gap-1 mt-1">
                          {#each result.ipv6 as ip (ip)}
                              <span class="text-xs font-mono text-primary-text break-all">{ip}</span>
                          {/each}
                      </div>
                  </div>
              {/if}

              <!-- Reverse DNS -->
              {#if result.reverse_dns}
                  <button class="p-3 rounded-lg bg-surface border border-base text-left hover:border-cyan-500/50 transition-colors group" onclick={() => result?.reverse_dns && copyToClipboard(result.reverse_dns, 'rdns')}>
                      <p class="text-xs text-muted uppercase font-semibold">Reverse DNS</p>
                      <p class="text-sm text-primary-text mt-1 font-mono flex items-center gap-1.5">
                          {result.reverse_dns}
                          {#if copiedField === 'rdns'}<Check class="size-3 text-green-400" />{:else}<Copy class="size-3 text-muted opacity-0 group-hover:opacity-100 transition-opacity" />{/if}
                      </p>
                  </button>
              {/if}
          </div>
      </div>

      <!-- Server Section -->
      <div class="mb-5">
          <p class="text-xs text-muted uppercase tracking-widest font-bold mb-3 flex items-center gap-1.5">
              <Server class="size-3.5" /> Server
          </p>
          <div class="grid grid-cols-1 md:grid-cols-3 gap-3">
              {#if result.web_server}
                  <div class="p-3 rounded-lg bg-surface border border-base">
                      <p class="text-xs text-muted uppercase font-semibold">Web Server</p>
                      <p class="text-sm text-primary-text mt-1 font-mono">{result.web_server}</p>
                  </div>
              {/if}
              {#if result.http_status}
                  <div class="p-3 rounded-lg bg-surface border border-base">
                      <p class="text-xs text-muted uppercase font-semibold">HTTP Status</p>
                      <p class="text-sm text-primary-text mt-1 font-mono">{result.http_status}</p>
                  </div>
              {/if}
              {#if result.response_time_ms != null}
                  <div class="p-3 rounded-lg bg-surface border border-base">
                      <p class="text-xs text-muted uppercase font-semibold flex items-center gap-1"><Clock class="size-3" /> Response Time</p>
                      <p class="text-sm mt-1 font-mono {result.response_time_ms < 500 ? 'text-green-400' : result.response_time_ms < 1500 ? 'text-yellow-400' : 'text-red-400'}">{result.response_time_ms.toFixed(0)} ms</p>
                  </div>
              {/if}
          </div>
      </div>

      <!-- WHOIS Section -->
      <div class="mb-5">
          <p class="text-xs text-muted uppercase tracking-widest font-bold mb-3 flex items-center gap-1.5">
              <Shield class="size-3.5" /> WHOIS
          </p>
          <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
              <div class="p-3 rounded-lg bg-surface border border-base">
                  <p class="text-xs text-muted uppercase font-semibold">Registrar</p>
                  <p class="text-sm text-primary-text mt-1 font-mono">{result.whois.registrar}</p>
              </div>
              <div class="p-3 rounded-lg bg-surface border border-base">
                  <p class="text-xs text-muted uppercase font-semibold">Registrant</p>
                  <p class="text-sm text-primary-text mt-1 font-mono">{result.whois.registrant}</p>
              </div>
              <div class="p-3 rounded-lg bg-surface border border-base">
                  <p class="text-xs text-muted uppercase font-semibold">Created</p>
                  <p class="text-sm text-primary-text mt-1 font-mono">{result.whois.creation_date}</p>
              </div>
              <div class="p-3 rounded-lg bg-surface border border-base">
                  <p class="text-xs text-muted uppercase font-semibold">Expires</p>
                  <p class="text-sm text-primary-text mt-1 font-mono">{result.whois.expiry_date}</p>
              </div>
              <div class="p-3 rounded-lg bg-surface border border-base">
                  <p class="text-xs text-muted uppercase font-semibold">Last Updated</p>
                  <p class="text-sm text-primary-text mt-1 font-mono">{result.whois.last_updated}</p>
              </div>
              <div class="p-3 rounded-lg bg-surface border border-base">
                  <p class="text-xs text-muted uppercase font-semibold">Privacy</p>
                  <p class="text-sm mt-1 font-mono {result.whois.privacy_protection === 'Active' ? 'text-green-400' : 'text-yellow-400'}">{result.whois.privacy_protection}</p>
              </div>
              <!-- Domain Status -->
              <div class="p-3 rounded-lg bg-surface border border-base col-span-1 md:col-span-2">
                  <p class="text-xs text-muted uppercase font-semibold">Domain Status</p>
                  <div class="flex flex-wrap gap-1.5 mt-1.5">
                      {#each result.whois.domain_status as status (status)}
                          <span class="text-xs font-mono px-2 py-0.5 rounded bg-cyan-500/10 text-accent border border-cyan-500/20">{status}</span>
                      {/each}
                  </div>
              </div>
          </div>

          <!-- Name Servers -->
          {#if result.whois.name_servers.length > 0}
              <div class="mt-3 p-3 rounded-lg bg-surface border border-base">
                  <p class="text-xs text-muted uppercase font-semibold mb-2">Name Servers</p>
                  <div class="flex flex-wrap gap-2">
                      {#each result.whois.name_servers as ns (ns)}
                          <span class="text-xs font-mono text-primary-text bg-surface border border-base px-2 py-1 rounded">{ns}</span>
                      {/each}
                  </div>
              </div>
          {/if}
      </div>
  {/if}
</div>
