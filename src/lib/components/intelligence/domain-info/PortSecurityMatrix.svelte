<script lang="ts">
  import { HelpCircle, Server } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import PortSecurityGuide from './PortSecurityGuide.svelte';

  type PortSecurityProps = {
      isLoading: boolean;
      ports: string[] | undefined;
      score: number | undefined;
  };

  let { isLoading, ports, score }: PortSecurityProps = $props();
  let guideOpen = $state(false);
</script>

<PortSecurityGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm transition-all duration-300">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent">Port Security</h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_ports_title()}>
          <HelpCircle class="size-4" />
      </button>
  </div>

  {#if isLoading}
      <div class="space-y-4">
          <div class="h-10 w-full bg-surface/50 rounded-lg animate-pulse"></div>
          <div class="flex gap-2">
              <div class="h-6 w-12 bg-surface rounded animate-pulse"></div>
              <div class="h-6 w-12 bg-surface rounded animate-pulse"></div>
          </div>
      </div>
  {:else if score != null}
      <!-- Security Score -->
      <div class="mb-6">
          <p class="text-5xl font-black {score > 70 ? 'text-green-500' : 'text-orange-500'}">{score}</p>
          <p class="text-sm text-muted font-medium tracking-wide">TOTAL SECURITY SCORE</p>
      </div>

      <!-- Open ports grid -->
      <div class="border-t border-base pt-4">
          <p class="text-xs text-muted uppercase tracking-widest mb-3">Open Ports</p>
          <div class="flex flex-wrap gap-2">
              {#if ports && ports.length > 0}
                  {#each ports as port (port)}
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
