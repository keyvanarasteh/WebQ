<script lang="ts">
  import * as m from '$lib/paraglide/messages';
  import type { TechnicalSeoResult } from '$lib/types/intelligence';
  import { Check, X } from 'lucide-svelte';

  type Props = {
      data: TechnicalSeoResult | undefined;
      isLoading: boolean;
  };
  
  let { data, isLoading }: Props = $props();
</script>

<div class="bg-background border border-base rounded-xl p-6 shadow-sm">
  <h3 class="text-lg font-bold text-accent mb-4">{m.seo_technical_title()}</h3>
  
  {#if isLoading}
    <div class="space-y-4 animate-pulse">
        <div class="h-10 bg-surface rounded"></div>
        <div class="h-10 bg-surface rounded"></div>
    </div>
  {:else if data}
    <div class="space-y-3">
        <div class="grid grid-cols-2 gap-2">
            <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
                <p class="text-2xl font-black text-accent">{data.http_status}</p>
                <p class="text-xs text-muted">HTTP Status</p>
            </div>
            <div class="p-2.5 bg-sunken rounded-lg border border-base text-center">
                <p class="text-2xl font-black text-primary-text">{(data.page_size_bytes / 1024).toFixed(1)}<span class="text-sm text-muted ml-1">KB</span></p>
                <p class="text-xs text-muted">Page Size</p>
            </div>
        </div>

        <div class="grid grid-cols-3 gap-2">
            <div class="p-2 bg-sunken rounded-lg border border-base text-center">
                <p class="text-lg font-bold text-primary-text">{data.redirects}</p>
                <p class="text-xs text-muted">Redirects</p>
            </div>
            <div class="p-2 bg-sunken rounded-lg border border-base text-center">
                <p class="text-lg font-bold text-cyan-400">{data.internal_links}</p>
                <p class="text-xs text-muted">Internal</p>
            </div>
            <div class="p-2 bg-sunken rounded-lg border border-base text-center">
                <p class="text-lg font-bold text-yellow-400">{data.external_links}</p>
                <p class="text-xs text-muted">External</p>
            </div>
        </div>

        <div class="space-y-1.5">
            <div class="flex items-center justify-between p-2.5 bg-sunken rounded-lg border border-base">
                <span class="text-sm font-medium text-primary-text">Structured Data</span>
                <span class="text-sm font-mono text-accent">{data.structured_data_count} items</span>
            </div>
            <div class="flex items-center justify-between p-2.5 bg-sunken rounded-lg border border-base">
                <span class="text-sm font-medium text-primary-text">Breadcrumbs</span>
                {#if data.has_breadcrumbs}
                    <Check class="size-4 text-green-400" />
                {:else}
                    <X class="size-4 text-red-400" />
                {/if}
            </div>
        </div>
    </div>
  {:else}
      <div class="text-muted text-sm py-4">{m.seo_no_data()}</div>
  {/if}
</div>
