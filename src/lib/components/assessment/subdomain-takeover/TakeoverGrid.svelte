<script lang="ts">
  import TakeoverRiskBadges from './TakeoverRiskBadges.svelte';
  import * as m from '$lib/paraglide/messages';
  import { ShieldAlert, Server, Copy, Check, ChevronDown, ChevronRight, Info, AlertTriangle, Network } from 'lucide-svelte';
  
  let { vulnerableSubdomains }: { vulnerableSubdomains: any[] } = $props();
  
  let copiedText = $state<string | null>(null);
  let expandedRows = $state<Set<string>>(new Set());

  function toggleExpand(subdomain: string) {
    if (expandedRows.has(subdomain)) {
      expandedRows.delete(subdomain);
    } else {
      expandedRows.add(subdomain);
    }
    // trigger Svelte 5 reactivity on sets by reassignment if needed. 
    // Usually Set methods aren't deeply reactive unless we do:
    expandedRows = new Set(expandedRows);
  }
  
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
          <tr class="hover:bg-glass-hover transition-colors cursor-pointer" onclick={() => toggleExpand(vuln.subdomain)}>
            <td class="px-6 py-3 font-medium text-primary-text break-all flex flex-col justify-center">
              <div class="flex items-center gap-2">
                {#if expandedRows.has(vuln.subdomain)}
                   <ChevronDown size={14} class="text-muted shrink-0" />
                {:else}
                   <ChevronRight size={14} class="text-muted shrink-0" />
                {/if}
                {vuln.subdomain}
              </div>
              {#if vuln.cname}
                <div class="text-xs text-muted mt-1 flex items-center gap-1 pl-6">
                   <span class="text-indigo-400 px-1 py-0.5 rounded bg-indigo-500/10 border border-indigo-500/20">CNAME</span> 
                   {vuln.cname}
                </div>
              {/if}
              {#if vuln.http_status}
                 <div class="text-xs text-muted mt-1 flex items-center gap-1 pl-6">
                   <span class="text-amber-400 px-1 py-0.5 rounded bg-amber-500/10 border border-amber-500/20">HTTP</span> 
                   Status: {vuln.http_status}
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
                  onclick={(e) => { e.stopPropagation(); copyText(vuln.subdomain); }}
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
          
          {#if expandedRows.has(vuln.subdomain)}
            <tr class="bg-surface/50 border-b border-subtle">
              <td colspan="5" class="px-6 py-4">
                  <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
                      <!-- Left details -->
                      <div class="flex flex-col gap-3">
                          <div class="flex items-start gap-2 text-sm text-secondary-text">
                              <Info size={16} class="text-indigo-400 shrink-0 mt-0.5" />
                              <div>
                                  <span class="text-muted block text-xs tracking-wider uppercase mb-1">Description</span>
                                  {vuln.description}
                              </div>
                          </div>
                          <div class="flex items-start gap-2 text-sm text-secondary-text">
                              <AlertTriangle size={16} class="text-orange-400 shrink-0 mt-0.5" />
                              <div>
                                  <span class="text-muted block text-xs tracking-wider uppercase mb-1">Mitigation</span>
                                  {vuln.mitigation}
                              </div>
                          </div>
                          <div>
                              <span class="text-muted block text-xs tracking-wider uppercase mb-1">Exploitation Difficulty</span>
                              <span class="px-2 py-0.5 text-xs text-orange-400 bg-orange-500/10 border border-orange-500/20 rounded font-semibold">
                                  {vuln.exploitation_difficulty}
                              </span>
                          </div>
                      </div>

                      <!-- Right details: DNS -->
                      <div class="bg-glass border border-subtle rounded-lg p-3">
                          <div class="flex items-center gap-2 mb-3">
                              <Network size={14} class="text-teal-400" />
                              <span class="text-xs font-semibold text-muted uppercase tracking-wider">DNS Records Exposed</span>
                          </div>
                          
                          <div class="space-y-1">
                              {#if vuln.dns_info.a_records.length > 0}
                                  <div class="grid grid-cols-12 gap-2 text-xs">
                                      <span class="col-span-2 font-mono font-bold text-muted text-right">A</span>
                                      <span class="col-span-10 font-mono text-teal-300 break-words">{vuln.dns_info.a_records.join(', ')}</span>
                                  </div>
                              {/if}
                              {#if vuln.dns_info.cname_records.length > 0}
                                  <div class="grid grid-cols-12 gap-2 text-xs">
                                      <span class="col-span-2 font-mono font-bold text-muted text-right">CNAME</span>
                                      <span class="col-span-10 font-mono text-purple-300 break-words">{vuln.dns_info.cname_records.join(', ')}</span>
                                  </div>
                              {/if}
                              {#if vuln.dns_info.txt_records.length > 0}
                                  <div class="grid grid-cols-12 gap-2 text-xs">
                                      <span class="col-span-2 font-mono font-bold text-muted text-right">TXT</span>
                                      <span class="col-span-10 font-mono text-amber-300 break-words line-clamp-2" title={vuln.dns_info.txt_records.join(' | ')}>{vuln.dns_info.txt_records.join(' | ')}</span>
                                  </div>
                              {/if}
                              {#if vuln.dns_info.ns_records.length > 0}
                                  <div class="grid grid-cols-12 gap-2 text-xs">
                                      <span class="col-span-2 font-mono font-bold text-muted text-right">NS</span>
                                      <span class="col-span-10 font-mono text-blue-300 break-words">{vuln.dns_info.ns_records.join(', ')}</span>
                                  </div>
                              {/if}
                              {#if !vuln.dns_info.has_valid_dns}
                                  <div class="text-xs text-rose-400 font-medium py-1">Missing Valid DNS Resolution (NXDomain possible)</div>
                              {/if}
                          </div>
                      </div>
                  </div>
              </td>
            </tr>
          {/if}
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
