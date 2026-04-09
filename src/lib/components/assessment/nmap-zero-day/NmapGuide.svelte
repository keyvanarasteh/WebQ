<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { ShieldAlert, Fingerprint } from 'lucide-svelte';
</script>

<div class="grid grid-cols-1 md:grid-cols-2 gap-6 mt-8">
    <div class="border border-white/5 bg-black/40 p-6 rounded-xl relative overflow-hidden group">
        <div class="absolute inset-0 bg-gradient-to-br from-red-500/5 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-700"></div>
        <div class="absolute -right-4 -top-4 opacity-5 pointer-events-none">
            <Fingerprint size={120} />
        </div>
        
        <h3 class="text-white font-medium flex items-center gap-2 mb-3 relative z-10">
            <Fingerprint size={16} class="text-red-400" />
            {m.sec_nmap_guide_mechanics()}
        </h3>
        <p class="text-sm text-zinc-400 leading-relaxed relative z-10">
            {m.sec_nmap_guide_desc()}
        </p>
    </div>

    <div class="border border-emerald-500/20 bg-emerald-500/5 p-6 rounded-xl relative overflow-hidden group">
        <div class="absolute inset-0 bg-gradient-to-br from-emerald-500/5 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-700"></div>
        <div class="absolute -right-4 -top-4 opacity-5 pointer-events-none">
            <ShieldAlert size={120} />
        </div>
        
        <h3 class="text-emerald-400 font-medium flex items-center gap-2 mb-3 relative z-10">
            <ShieldAlert size={16} />
            {m.sec_nmap_guide_secops()}
        </h3>
        <p class="text-sm text-zinc-400 leading-relaxed relative z-10">
            {m.sec_nmap_guide_secops_desc()}
        </p>
    </div>
</div>
