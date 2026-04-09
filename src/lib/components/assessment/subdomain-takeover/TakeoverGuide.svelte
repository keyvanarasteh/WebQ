<script lang="ts">
  import { Info, X, ShieldAlert, Globe, ServerCrash } from 'lucide-svelte';
  import * as m from '$lib/paraglide/messages';
  import { fade, fly } from 'svelte/transition';
  
  let { open = $bindable(false) }: { open: boolean } = $props();
</script>

{#if open}
  <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
  <div 
    class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
    transition:fade={{ duration: 150 }}
    onclick={() => open = false}
  >
    <div 
      class="w-full max-w-2xl bg-[#0F1115] border border-white/10 rounded-xl shadow-2xl overflow-hidden flex flex-col max-h-[85vh]"
      transition:fly={{ y: 20, duration: 200 }}
      onclick={(e) => e.stopPropagation()}
    >
      <!-- Header -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-white/5 bg-[#14171C]">
        <div class="flex items-center gap-3">
          <div class="p-2 rounded-lg bg-indigo-500/10 text-indigo-400">
            <Info size={20} />
          </div>
          <div>
            <h2 class="text-white font-medium">{m.sec_takeover_guide_title()}</h2>
            <p class="text-xs text-slate-400">{m.sec_posture_secops_context()}</p>
          </div>
        </div>
        <button
          class="p-2 text-slate-400 hover:text-white hover:bg-white/5 rounded-lg transition-colors"
          onclick={() => { open = false; }}
          aria-label="Close"
        >
          <X size={20} />
        </button>
      </div>
      
      <!-- Content -->
      <div class="flex-1 overflow-y-auto px-6 py-6 space-y-6">
        <div class="p-4 rounded-xl bg-indigo-500/5 border border-indigo-500/10">
          <p class="text-sm text-slate-300 leading-relaxed">
            {m.sec_takeover_guide_desc()}
          </p>
        </div>

        <div class="space-y-4">
          <h3 class="text-xs font-semibold uppercase tracking-wider text-slate-400 flex items-center gap-2">
            <Globe size={14} /> Mechanics
          </h3>
          <ul class="space-y-3">
            <li class="flex items-start gap-3">
              <div class="mt-0.5 p-1 rounded bg-rose-500/10 text-rose-400 border border-rose-500/20">
                <ServerCrash size={14} />
              </div>
              <div class="text-sm text-slate-300">
                <span class="text-white font-medium">Dangling CNAMEs:</span> A CNAME record points to an external service (e.g. `myapp.herokuapp.com`). If you delete the app on Heroku but leave the CNAME, an attacker can register `myapp` on Heroku and serve content on your domain.
              </div>
            </li>
            <li class="flex items-start gap-3">
              <div class="mt-0.5 p-1 rounded bg-amber-500/10 text-amber-400 border border-amber-500/20">
                <ShieldAlert size={14} />
              </div>
              <div class="text-sm text-slate-300">
                <span class="text-white font-medium">Broken Nameservers:</span> Stale NS records delegating poorly managed subdomains to expired services like Route53 or DigitalOcean.
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
{/if}
