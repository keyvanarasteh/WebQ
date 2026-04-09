<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Shield, Network } from 'lucide-svelte';
</script>

<div class="mt-8 grid grid-cols-1 md:grid-cols-2 gap-6">
    <div class="bg-black/40 border border-white/5 rounded-xl p-6 backdrop-blur-xl shadow-2xl relative overflow-hidden group">
        <div class="absolute -right-12 -top-12 w-48 h-48 bg-teal-500/10 blur-3xl rounded-full group-hover:bg-teal-500/20 transition-colors duration-700"></div>
        <div class="flex items-start gap-4 relative">
            <div class="mt-1 w-10 h-10 rounded-lg bg-teal-400/10 flex items-center justify-center text-teal-400 border border-teal-400/20 shrink-0">
                <Network size={20} />
            </div>
            <div>
                <h3 class="text-white font-medium mb-2 flex items-center gap-2">
                    {m.sec_cfbypass_guide_mechanics()}
                </h3>
                <p class="text-sm text-zinc-400 leading-relaxed">
                    {m.sec_cfbypass_guide_desc()}
                </p>
            </div>
        </div>
    </div>

    <div class="bg-black/40 border border-white/5 rounded-xl p-6 backdrop-blur-xl shadow-2xl relative overflow-hidden group">
        <div class="absolute -right-12 -top-12 w-48 h-48 bg-emerald-500/10 blur-3xl rounded-full group-hover:bg-emerald-500/20 transition-colors duration-700"></div>
        <div class="flex items-start gap-4 relative">
            <div class="mt-1 w-10 h-10 rounded-lg bg-emerald-400/10 flex items-center justify-center text-emerald-400 border border-emerald-400/20 shrink-0">
                <Shield size={20} />
            </div>
            <div>
                <h3 class="text-white font-medium mb-2 flex items-center gap-2">
                    {m.sec_cfbypass_guide_secops()}
                </h3>
                <p class="text-sm text-zinc-400 leading-relaxed">
                    {m.sec_cfbypass_guide_secops_desc()}
                </p>
            </div>
        </div>
    </div>
</div>
