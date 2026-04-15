<script lang="ts">
  import type { SslInfo } from '$lib/types/intelligence';
  import { ShieldCheck, ShieldAlert, ShieldX, Lock, HelpCircle, RefreshCw } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import * as m from '$lib/paraglide/messages';
  import SslStatusGuide from './SslStatusGuide.svelte';

  type Props = {
      isLoading: boolean;
      ssl: SslInfo | null;
      domain: string;
  };

  let { isLoading, ssl, domain }: Props = $props();
  let guideOpen = $state(false);
  let isRescanning = $state(false);
  let localSsl = $state<SslInfo | null>(null);

  $effect(() => {
      if (ssl) {
          localSsl = ssl;
      }
  });

  async function handleRescan() {
      if (!domain) return;
      isRescanning = true;
      try {
          localSsl = await invoke<SslInfo>('scan_ssl', { domain });
      } catch (e) {
          console.error("Failed to rescan SSL:", e);
      } finally {
          isRescanning = false;
      }
  }

  let finalLoading = $derived(isLoading || isRescanning);

  let statusColor = $derived(
      localSsl?.status === 'Valid' ? 'text-green-400 border-green-500' :
      localSsl?.status === 'Error' ? 'text-red-400 border-red-500' :
      'text-yellow-400 border-yellow-500'
  );

  let gaugeColor = $derived(
      localSsl?.days_until_expiry != null && localSsl.days_until_expiry > 30 ? 'border-green-500' :
      localSsl?.days_until_expiry != null && localSsl.days_until_expiry > 7 ? 'border-yellow-500' :
      'border-red-500'
  );
</script>

<SslStatusGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm transition-all duration-300">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2">
          <Lock class="size-5" />
          SSL & Cryptography
      </h3>
      <div class="flex items-center gap-1">
          {#if !isLoading && domain}
              <button onclick={handleRescan} disabled={isRescanning} class="p-1.5 rounded-lg text-muted hover:text-cyan-400 hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all disabled:opacity-50" title="Independent SSL Rescan">
                  <RefreshCw class="size-4 {isRescanning ? 'animate-spin' : ''}" />
              </button>
          {/if}
          <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_ssl_title()}>
              <HelpCircle class="size-4" />
          </button>
      </div>
  </div>

  {#if finalLoading}
      <div class="space-y-4 animate-pulse">
          <div class="flex justify-center"><div class="w-20 h-20 bg-surface rounded-full"></div></div>
          <div class="h-4 bg-surface rounded w-2/3 mx-auto"></div>
          <div class="h-4 bg-surface rounded w-1/2 mx-auto"></div>
      </div>
  {:else if localSsl}
      <!-- Status Badge + Days Gauge -->
      <div class="flex items-center gap-4 mb-5">
          <div class="relative w-20 h-20 flex items-center justify-center rounded-full border-4 {gaugeColor} shrink-0">
              <span class="text-xl font-bold text-primary-text">{localSsl.days_until_expiry ?? '?'}</span>
          </div>
          <div>
              <div class="flex items-center gap-2 mb-1">
                  {#if localSsl.status === 'Valid'}
                      <ShieldCheck class="size-4 text-green-400" />
                  {:else if localSsl.status === 'Error'}
                      <ShieldX class="size-4 text-red-400" />
                  {:else}
                      <ShieldAlert class="size-4 text-yellow-400" />
                  {/if}
                  <span class="text-sm font-bold {statusColor}">{localSsl.status}</span>
              </div>
              <p class="text-xs text-muted">Days Until Expiry</p>
          </div>
      </div>

      <!-- SSL Detail Fields -->
      <div class="space-y-2.5">
          {#if localSsl.issued_to}
              <div class="flex justify-between items-start p-2.5 bg-sunken rounded-lg border border-base">
                  <span class="text-xs text-muted font-semibold uppercase">Issued To</span>
                  <span class="text-sm text-primary-text font-mono text-right max-w-[60%] break-all">{localSsl.issued_to}</span>
              </div>
          {/if}
          {#if localSsl.issuer}
              <div class="flex justify-between items-start p-2.5 bg-sunken rounded-lg border border-base">
                  <span class="text-xs text-muted font-semibold uppercase">Issuer (CA)</span>
                  <span class="text-sm text-primary-text font-mono text-right max-w-[60%] break-all">{localSsl.issuer}</span>
              </div>
          {/if}
          {#if localSsl.protocol_version}
              <div class="flex justify-between items-center p-2.5 bg-sunken rounded-lg border border-base">
                  <span class="text-xs text-muted font-semibold uppercase">Protocol</span>
                  <span class="text-sm font-mono {localSsl.protocol_version.includes('TLSv1.3') ? 'text-green-400' : localSsl.protocol_version.includes('TLSv1.2') ? 'text-accent' : 'text-yellow-400'}">{localSsl.protocol_version}</span>
              </div>
          {/if}
          {#if localSsl.expiry_date}
              <div class="flex justify-between items-center p-2.5 bg-sunken rounded-lg border border-base">
                  <span class="text-xs text-muted font-semibold uppercase">Expiry Date</span>
                  <span class="text-sm text-primary-text font-mono">{localSsl.expiry_date}</span>
              </div>
          {/if}
      </div>

      <!-- SANs -->
      {#if localSsl.alternative_names.length > 0}
          <div class="mt-4 p-3 bg-sunken rounded-lg border border-base">
              <p class="text-xs text-muted font-semibold uppercase mb-2">Subject Alt Names ({localSsl.alternative_names.length})</p>
              <div class="flex flex-wrap gap-1.5">
                  {#each localSsl.alternative_names as san, i (i)}
                      <span class="text-xs font-mono px-2 py-0.5 rounded bg-cyan-500/10 text-accent border border-cyan-500/20">{san}</span>
                  {/each}
              </div>
          </div>
      {/if}
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <Lock class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
