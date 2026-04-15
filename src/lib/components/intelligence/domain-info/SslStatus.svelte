<script lang="ts">
  import type { SslInfo } from '$lib/types/intelligence';
  import { ShieldCheck, ShieldAlert, ShieldX, Lock } from 'lucide-svelte';

  type Props = {
      isLoading: boolean;
      ssl: SslInfo | null;
  };

  let { isLoading, ssl }: Props = $props();

  let statusColor = $derived(
      ssl?.status === 'Valid' ? 'text-green-400 border-green-500' :
      ssl?.status === 'Error' ? 'text-red-400 border-red-500' :
      'text-yellow-400 border-yellow-500'
  );

  let gaugeColor = $derived(
      ssl?.days_until_expiry != null && ssl.days_until_expiry > 30 ? 'border-green-500' :
      ssl?.days_until_expiry != null && ssl.days_until_expiry > 7 ? 'border-yellow-500' :
      'border-red-500'
  );
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm transition-all duration-300">
  <h3 class="text-lg font-bold text-accent mb-4 flex items-center gap-2">
      <Lock class="size-5" />
      SSL & Cryptography
  </h3>

  {#if isLoading}
      <div class="space-y-4 animate-pulse">
          <div class="flex justify-center"><div class="w-20 h-20 bg-surface rounded-full"></div></div>
          <div class="h-4 bg-surface rounded w-2/3 mx-auto"></div>
          <div class="h-4 bg-surface rounded w-1/2 mx-auto"></div>
      </div>
  {:else if ssl}
      <!-- Status Badge + Days Gauge -->
      <div class="flex items-center gap-4 mb-5">
          <div class="relative w-20 h-20 flex items-center justify-center rounded-full border-4 {gaugeColor} shrink-0">
              <span class="text-xl font-bold text-primary-text">{ssl.days_until_expiry ?? '?'}</span>
          </div>
          <div>
              <div class="flex items-center gap-2 mb-1">
                  {#if ssl.status === 'Valid'}
                      <ShieldCheck class="size-4 text-green-400" />
                  {:else if ssl.status === 'Error'}
                      <ShieldX class="size-4 text-red-400" />
                  {:else}
                      <ShieldAlert class="size-4 text-yellow-400" />
                  {/if}
                  <span class="text-sm font-bold {statusColor}">{ssl.status}</span>
              </div>
              <p class="text-xs text-muted">Days Until Expiry</p>
          </div>
      </div>

      <!-- SSL Detail Fields -->
      <div class="space-y-2.5">
          {#if ssl.issued_to}
              <div class="flex justify-between items-start p-2.5 bg-sunken rounded-lg border border-base">
                  <span class="text-xs text-muted font-semibold uppercase">Issued To</span>
                  <span class="text-sm text-primary-text font-mono text-right max-w-[60%] break-all">{ssl.issued_to}</span>
              </div>
          {/if}
          {#if ssl.issuer}
              <div class="flex justify-between items-start p-2.5 bg-sunken rounded-lg border border-base">
                  <span class="text-xs text-muted font-semibold uppercase">Issuer (CA)</span>
                  <span class="text-sm text-primary-text font-mono text-right max-w-[60%] break-all">{ssl.issuer}</span>
              </div>
          {/if}
          {#if ssl.protocol_version}
              <div class="flex justify-between items-center p-2.5 bg-sunken rounded-lg border border-base">
                  <span class="text-xs text-muted font-semibold uppercase">Protocol</span>
                  <span class="text-sm font-mono {ssl.protocol_version.includes('TLSv1.3') ? 'text-green-400' : ssl.protocol_version.includes('TLSv1.2') ? 'text-accent' : 'text-yellow-400'}">{ssl.protocol_version}</span>
              </div>
          {/if}
          {#if ssl.expiry_date}
              <div class="flex justify-between items-center p-2.5 bg-sunken rounded-lg border border-base">
                  <span class="text-xs text-muted font-semibold uppercase">Expiry Date</span>
                  <span class="text-sm text-primary-text font-mono">{ssl.expiry_date}</span>
              </div>
          {/if}
      </div>

      <!-- SANs -->
      {#if ssl.alternative_names.length > 0}
          <div class="mt-4 p-3 bg-sunken rounded-lg border border-base">
              <p class="text-xs text-muted font-semibold uppercase mb-2">Subject Alt Names ({ssl.alternative_names.length})</p>
              <div class="flex flex-wrap gap-1.5">
                  {#each ssl.alternative_names as san (san)}
                      <span class="text-xs font-mono px-2 py-0.5 rounded bg-cyan-500/10 text-accent border border-cyan-500/20">{san}</span>
                  {/each}
              </div>
          </div>
      {/if}
  {/if}
</div>
