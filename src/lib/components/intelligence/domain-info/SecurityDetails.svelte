<script lang="ts">
  import type { SecurityInfo } from '$lib/types/intelligence';
  import { ShieldCheck, ShieldX, ArrowRight, HelpCircle } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import SecurityDetailsGuide from './SecurityDetailsGuide.svelte';

  type Props = {
      isLoading: boolean;
      security: SecurityInfo | null;
      score: number | undefined;
  };

  let { isLoading, security, score }: Props = $props();
  let guideOpen = $state(false);

  const HEADER_LABELS: Record<string, string> = {
      'strict-transport-security': 'HSTS',
      'x-frame-options': 'X-Frame-Options',
      'x-content-type-options': 'X-Content-Type',
      'x-xss-protection': 'XSS Protection',
      'content-security-policy': 'CSP',
  };

  const ALL_HEADERS = ['strict-transport-security', 'x-frame-options', 'x-content-type-options', 'x-xss-protection', 'content-security-policy'];
</script>

<SecurityDetailsGuide bind:isOpen={guideOpen} />

<div class="bg-background border border-base rounded-xl p-6 shadow-sm transition-all duration-300">
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-accent flex items-center gap-2">
          <ShieldCheck class="size-5" />
          Security Assessment
      </h3>
      <button onclick={() => guideOpen = true} class="p-1.5 rounded-lg text-muted hover:text-accent hover:bg-cyan-500/10 border border-transparent hover:border-cyan-500/20 transition-all" title={m.guide_security_headers_title()}>
          <HelpCircle class="size-4" />
      </button>
  </div>

  {#if isLoading}
      <div class="space-y-3 animate-pulse">
          <div class="h-12 bg-surface rounded-lg"></div>
          <div class="h-8 bg-surface/50 rounded"></div>
          <div class="h-8 bg-surface/50 rounded"></div>
      </div>
  {:else if security}
      <!-- Score -->
      <div class="mb-5">
          <p class="text-5xl font-black {score != null && score > 70 ? 'text-green-500' : score != null && score > 40 ? 'text-yellow-500' : 'text-red-500'}">{score ?? 0}</p>
          <p class="text-sm text-muted font-medium tracking-wide">TOTAL SECURITY SCORE</p>
      </div>

      <!-- HTTPS Checks -->
      <div class="space-y-2 mb-4">
          <div class="flex items-center justify-between p-2.5 bg-sunken rounded-lg border border-base">
              <span class="text-sm text-primary-text font-medium">HTTPS Available</span>
              {#if security.https_available}
                  <span class="px-2 py-0.5 bg-green-500/10 text-green-400 rounded border border-green-500/30 text-xs font-bold">YES</span>
              {:else}
                  <span class="px-2 py-0.5 bg-red-500/10 text-red-400 rounded border border-red-500/30 text-xs font-bold">NO</span>
              {/if}
          </div>
          <div class="flex items-center justify-between p-2.5 bg-sunken rounded-lg border border-base">
              <span class="text-sm text-primary-text font-medium flex items-center gap-1.5">HTTP <ArrowRight class="size-3 text-muted" /> HTTPS Redirect</span>
              {#if security.https_redirect}
                  <span class="px-2 py-0.5 bg-green-500/10 text-green-400 rounded border border-green-500/30 text-xs font-bold">YES</span>
              {:else}
                  <span class="px-2 py-0.5 bg-red-500/10 text-red-400 rounded border border-red-500/30 text-xs font-bold">NO</span>
              {/if}
          </div>
      </div>

      <!-- Security Headers Checklist -->
      <div class="border-t border-base pt-4">
          <p class="text-xs text-muted uppercase tracking-widest mb-3 font-bold">Security Headers ({security.headers_count}/5)</p>
          <div class="space-y-1.5">
              {#each ALL_HEADERS as header (header)}
                  {@const present = header in security.security_headers}
                  <div class="flex items-center justify-between p-2 rounded-md {present ? 'bg-green-500/5' : 'bg-red-500/5'}">
                      <span class="text-xs font-mono text-primary-text">{HEADER_LABELS[header] ?? header}</span>
                      {#if present}
                          <ShieldCheck class="size-3.5 text-green-400" />
                      {:else}
                          <ShieldX class="size-3.5 text-red-400" />
                      {/if}
                  </div>
              {/each}
          </div>
      </div>
  {:else}
      <div class="border-2 border-dashed border-base rounded-xl p-6 flex flex-col items-center justify-center gap-3 text-center">
          <span class="text-xs font-bold tracking-widest px-3 py-1 bg-surface border border-base rounded-full text-muted">{m.intel_pending_badge()}</span>
          <ShieldCheck class="size-8 text-muted/30" />
          <p class="text-sm text-muted">{m.intel_pending_msg()}</p>
      </div>
  {/if}
</div>
