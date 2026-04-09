<script lang="ts">
  import { Info } from 'lucide-svelte';
  import SecurityHeadersGuide from '$lib/components/recon/guides/SecurityHeadersGuide.svelte';
  import * as m from '$lib/paraglide/messages';
  type SecurityHeaderInfo = {
      strict_transport_security: boolean;
      x_frame_options: boolean;
      x_content_type_options: boolean;
      content_security_policy: boolean;
  };
  
  let { data, isLoading } = $props<{ data: SecurityHeaderInfo | undefined, isLoading: boolean }>();
  let isGuideOpen = $state(false);
</script>

<div class="col-span-full bg-background/5 bg-background border border-base border-base rounded-xl p-6 shadow-sm mt-6">
  <SecurityHeadersGuide bind:isOpen={isGuideOpen} />
  
  <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-bold text-inverse text-accent">{m.sec_headers_assessment()}</h3>
      <button onclick={() => isGuideOpen = true} class="p-1 hover:bg-cyan-500/10 rounded-full text-accent transition-colors" title={m.secops_guide_title()}><Info class="size-4" /></button>
  </div>
  
  {#if isLoading}
    <div class="h-24 bg-surface bg-surface rounded animate-pulse"></div>
  {:else if data}
    <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-4">
        {#each [
            { name: m.sec_header_hsts(), val: data.strict_transport_security, desc: m.sec_header_hsts_desc() },
            { name: m.sec_header_xfo(), val: data.x_frame_options, desc: m.sec_header_xfo_desc() },
            { name: m.sec_header_xcto(), val: data.x_content_type_options, desc: m.sec_header_xcto_desc() },
            { name: m.sec_header_csp(), val: data.content_security_policy, desc: m.sec_header_csp_desc() },
        ] as header (header.name)}
            <div class="p-4 bg-background dark:bg-[#121214] border border-base border-base hover:border-gray-300 dark:hover:border-[#3f3f46] transition-colors rounded-lg flex flex-col justify-between h-full">
                <div class="mb-4">
                    <h4 class="text-sm font-bold text-inverse text-primary-text mb-1">{header.name}</h4>
                    <p class="text-xs text-muted leading-relaxed">{header.desc}</p>
                </div>
                <div>
                    {#if header.val}
                        <span class="inline-flex items-center px-2 py-1 bg-green-500/10 text-green-500 dark:text-green-400 text-xs font-black rounded border border-green-500/20 uppercase tracking-widest">{m.sec_header_configured()}</span>
                    {:else}
                        <span class="inline-flex items-center px-2 py-1 bg-red-500/10 text-red-500 text-xs font-black rounded border border-red-500/20 uppercase tracking-widest">{m.sec_header_missing()}</span>
                    {/if}
                </div>
            </div>
        {/each}
    </div>
  {:else}
      <div class="text-muted text-sm py-4">{m.tech_awaiting()}</div>
  {/if}
</div>
