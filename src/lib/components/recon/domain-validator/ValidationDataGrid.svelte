<script lang="ts">
  import { Info } from 'lucide-svelte';
  import ValidatorGuide from '$lib/components/recon/guides/ValidatorGuide.svelte';
  import * as m from '$lib/paraglide/messages';
  type ValidationResult = {
      domain: string;
      is_valid: boolean;
      dns_resolved: boolean;
      http_ok: boolean;
      https_ok: boolean;
  };
  
  let { results, isLoading } = $props<{ results: ValidationResult[] | undefined, isLoading: boolean }>();
  let isGuideOpen = $state(false);
</script>

<div class="bg-white/5 dark:bg-[#09090b] border border-gray-200 dark:border-[#27272a] rounded-xl overflow-hidden shadow-sm">
   <ValidatorGuide bind:isOpen={isGuideOpen} />
   
   <div class="p-6 border-b border-gray-200 dark:border-[#27272a] bg-gray-50 dark:bg-[#121214] flex justify-between items-center">
       <div class="flex items-center gap-2">
           <h3 class="text-lg font-bold text-gray-900 dark:text-cyan-400">{m.val_ledger_title()}</h3>
           <button onclick={() => isGuideOpen = true} class="p-1 hover:bg-cyan-500/10 rounded-full text-cyan-500 transition-colors" title={m.secops_guide_title()}><Info class="size-4" /></button>
       </div>
       <span class="text-xs text-gray-500 font-mono">{m.val_domains_tracked({ count: results?.length || 0 })}</span>
   </div>

   <div class="overflow-x-auto max-h-[600px] overflow-y-auto">
       <table class="w-full text-left border-collapse">
           <thead class="sticky top-0 z-10">
               <tr class="bg-gray-100 dark:bg-[#18181b] border-b border-gray-200 dark:border-[#27272a] text-xs uppercase tracking-widest text-gray-500 font-black">
                   <th class="p-4">Target Domain</th>
                   <th class="p-4 text-center">DNS Res</th>
                   <th class="p-4 text-center">HTTP:80</th>
                   <th class="p-4 text-center">HTTPS:443</th>
                   <th class="p-4 text-right">Target State</th>
               </tr>
           </thead>
           <tbody class="divide-y divide-gray-200 dark:divide-[#27272a]">
               {#if isLoading && (!results || results.length === 0)}
                   {#each Array(7) as _, i (i)}
                       <tr class="animate-pulse bg-gray-50/50 dark:bg-[#09090b]">
                           <td class="p-4"><div class="h-4 bg-gray-200 dark:bg-[#27272a] rounded w-48"></div></td>
                           <td class="p-4"><div class="h-4 bg-gray-200 dark:bg-[#27272a] rounded w-8 mx-auto"></div></td>
                           <td class="p-4"><div class="h-4 bg-gray-200 dark:bg-[#27272a] rounded w-8 mx-auto"></div></td>
                           <td class="p-4"><div class="h-4 bg-gray-200 dark:bg-[#27272a] rounded w-8 mx-auto"></div></td>
                           <td class="p-4"><div class="h-6 bg-gray-200 dark:bg-[#27272a] rounded w-16 ml-auto"></div></td>
                       </tr>
                   {/each}
               {:else if results && results.length > 0}
                   {#each results as row (row.domain)}
                       <tr class="hover:bg-gray-100 hover:dark:bg-[#121214] transition-colors {row.is_valid ? 'bg-white dark:bg-[#09090b]' : 'bg-gray-50 dark:bg-[#0d0d0f] opacity-80'}">
                           <td class="p-4 font-mono text-sm text-gray-900 dark:text-gray-200 font-medium">{row.domain}</td>
                           <td class="p-4 text-center">
                               {#if row.dns_resolved}
                                   <span class="text-green-500 font-black">✔</span>
                               {:else}
                                   <span class="text-red-500 font-black">✖</span>
                               {/if}
                           </td>
                           <td class="p-4 text-center">
                               {#if row.http_ok}
                                   <span class="text-green-500 font-black">✔</span>
                               {:else}
                                   <span class="text-red-500 font-black">✖</span>
                               {/if}
                           </td>
                           <td class="p-4 text-center">
                               {#if row.https_ok}
                                   <span class="text-green-500 font-black">✔</span>
                               {:else}
                                   <span class="text-red-500 font-black">✖</span>
                               {/if}
                           </td>
                           <td class="p-4 text-right">
                               {#if row.is_valid}
                                   <span class="px-2 py-1 bg-green-500/10 text-green-600 dark:text-green-400 text-[10px] font-black rounded uppercase tracking-widest border border-green-500/30">VALID</span>
                               {:else}
                                   <span class="px-2 py-1 bg-red-500/10 text-red-600 dark:text-red-500 text-[10px] font-black rounded uppercase tracking-widest border border-red-500/30">INVALID</span>
                               {/if}
                           </td>
                       </tr>
                   {/each}
               {:else}
                   <tr>
                       <td colspan="5" class="p-12 text-center text-gray-500 text-sm font-mono tracking-wide">
                           Waiting for targets to be processed.
                       </td>
                   </tr>
               {/if}
           </tbody>
       </table>
   </div>
</div>
