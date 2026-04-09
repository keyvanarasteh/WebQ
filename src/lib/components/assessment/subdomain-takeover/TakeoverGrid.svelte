<script lang="ts">
  import TakeoverRiskBadges from './TakeoverRiskBadges.svelte';
  import * as m from '$lib/paraglide/messages';
  import { ShieldAlert, Server, Copy, Check } from 'lucide-svelte';
  
  let { vulnerableSubdomains }: { vulnerableSubdomains: any[] } = $props();
  
  let copiedText = $state<string | null>(null);
  
  function copyText(text: string) {
    navigator.clipboard.writeText(text);
    copiedText = text;
    setTimeout(() => {
      copiedText = null;
    }, 2000);
  }
</script>

<div class="rounded-xl border border-subtle bg-[#14171C] overflow-hidden">
  <div class="px-6 py-4 border-b border-subtle flex items-center justify-between">
    <div class="flex items-center gap-2 text-primary-text">
      <ShieldAlert size={16} class="text-rose-500"/>
      <h3 class="font-medium text-sm">Identified Vulnerabilities</h3>
    </div>
    <div class="text-xs text-muted bg-glass px-2 py-1 rounded-md">
      {vulnerableSubdomains.length} Found
    </div>
  </div>

  <div class="overflow-x-auto">
    <table class="w-full text-left text-sm border-collapse">
      <thead>
        <tr class="bg-glass-hover border-b border-subtle text-muted">
          <th class="px-6 py-3 font-medium">{m.sec_takeover_col_subdomain()}</th>
          <th class="px-6 py-3 font-medium">{m.sec_takeover_col_service()}</th>
          <th class="px-6 py-3 font-medium">{m.sec_takeover_col_type()}</th>
          <th class="px-6 py-3 font-medium">{m.sec_takeover_col_confidence()}</th>
          <th class="px-6 py-3 font-medium">{m.sec_takeover_col_actions()}</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-subtle">
        {#each vulnerableSubdomains as vuln (vuln.subdomain)}
          <tr class="hover:bg-glass-hover transition-colors">
            <td class="px-6 py-3 font-medium text-primary-text break-all flex flex-col justify-center">
              {vuln.subdomain}
              {#if vuln.cname}
                <div class="text-xs text-muted mt-1 flex items-center gap-1">
                   <span class="text-indigo-400 px-1 py-0.5 rounded bg-indigo-500/10 border border-indigo-500/20">CNAME</span> 
                   {vuln.cname}
                </div>
              {/if}
            </td>
            <td class="px-6 py-3">
              <div class="flex items-center gap-2">
                <Server size={14} class="text-muted" />
                <span class={vuln.service === 'Unknown' ? 'text-muted bg-slate-500/10 px-1.5 py-0.5 rounded' : 'text-emerald-400 bg-emerald-500/10 px-1.5 py-0.5 rounded'}>{vuln.service}</span>
              </div>
            </td>
            <td class="px-6 py-3 text-secondary-text">
              {vuln.vulnerability_type}
            </td>
            <td class="px-6 py-3">
              <TakeoverRiskBadges confidence={vuln.confidence} />
            </td>
            <td class="px-6 py-3">
               <button 
                  class="p-1.5 rounded-lg bg-glass text-muted hover:text-primary-text hover:bg-glass-hover transition-colors"
                  onclick={() => copyText(vuln.subdomain)}
                  title="Copy Subdomain"
               >
                 {#if copiedText === vuln.subdomain}
                   <Check size={14} class="text-emerald-500" />
                 {:else}
                   <Copy size={14} />
                 {/if}
               </button>
            </td>
          </tr>
        {/each}
        {#if vulnerableSubdomains.length === 0}
          <tr>
            <td colspan="5" class="px-6 py-12 text-center text-muted">
              No vulnerable subdomains found. Your assets are secure.
            </td>
          </tr>
        {/if}
      </tbody>
    </table>
  </div>
</div>
