<script lang="ts">
  import { HelpCircle, Server, RefreshCw, Play } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import * as m from '$lib/paraglide/messages';
  import PortSecurityGuide from './PortSecurityGuide.svelte';

  type PortSecurityProps = {
      isLoading: boolean;
      ports: string[] | undefined;
      score: number | undefined;
      domain: string;
  };

  let { isLoading, ports, score, domain }: PortSecurityProps = $props();
  let guideOpen = $state(false);
  let isRescanning = $state(false);
  let localPorts = $state<string[] | undefined>(undefined);

  $effect(() => {
      if (ports !== undefined) {
          localPorts = ports;
      }
  });

  async function handleRescan() {
      if (!domain) return;
      isRescanning = true;
      try {
          const ipInfo = await invoke<{ ipv4: string | null }>('scan_ip_resolution', { domain });
          if (ipInfo.ipv4) {
              localPorts = await invoke<string[]>('scan_ports', { ip: ipInfo.ipv4 });
          } else {
              console.error("Could not resolve IPv4 for Port Scan");
              localPorts = [];
          }
      } catch(e) {
          console.error("Failed to rescan Ports:", e);
      } finally {
          isRescanning = false;
      }
  }

  let finalLoading = $derived(isLoading || isRescanning);
  let isValidDomain = $derived(/^[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/.test(domain));
</script>

<PortSecurityGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm transition-all duration-300">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent">Port Security</h3>
      <div class="flex items-center gap-1">
          <button onclick={handleRescan} disabled={isRescanning || isLoading || !isValidDomain} class="p-1.5 rounded-lg {isRescanning ? 'text-cyan-400' : 'text-muted'} hover:text-cyan-400 hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all disabled:opacity-50 disabled:cursor-not-allowed" title={localPorts != null ? "Refresh Ports Scan" : "Run Ports Scan"}>
              {#if localPorts != null}
                  <RefreshCw class="size-4 {isRescanning ? 'animate-spin' : ''}" />
              {:else}
                  <Play class="size-4" />
              {/if}
          </button>
          <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_ports_title()}>
              <HelpCircle class="size-4" />
          </button>
      </div>
  </div>

  {#if finalLoading}
      <div class="space-y-4">
          <div class="h-10 w-full bg-surface/50 rounded-lg animate-pulse"></div>
          <div class="flex gap-2">
              <div class="h-6 w-12 bg-surface rounded animate-pulse"></div>
              <div class="h-6 w-12 bg-surface rounded animate-pulse"></div>
          </div>
      </div>
  {:else if localPorts !== undefined}
      <!-- Security Score -->
      {#if score != null}
          <div class="mb-6">
              <p class="text-5xl font-black {score > 70 ? 'text-green-500' : 'text-orange-500'}">{score}</p>
              <p class="text-sm text-muted font-medium tracking-wide">TOTAL SECURITY SCORE</p>
          </div>
      {/if}

      <!-- Open ports grid -->
      <div class="{score != null ? 'border-t border-base pt-4' : 'pt-2'}">
          <p class="text-xs text-muted uppercase tracking-widest mb-3">Open Ports</p>
          <div class="flex flex-wrap gap-2">
              {#if localPorts && localPorts.length > 0}
                  {#each localPorts as port, i (i)}
                      {@const isDanger = port.startsWith('21') || port.startsWith('22') || port.startsWith('3306')}
                      <span class="px-2 py-1 text-xs font-mono rounded-md border
                          {isDanger ? 'bg-red-500/10 border-red-500/50 text-red-400' : 'bg-cyan-500/10 border-cyan-500/30 text-accent'}">
                          {port}
                      </span>
                  {/each}
              {:else}
                  <span class="text-sm text-muted">No open ports detected.</span>
              {/if}
          </div>
      </div>
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <Server class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
