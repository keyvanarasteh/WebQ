<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { UserSearch, FileSearch, HelpCircle, ShieldAlert } from 'lucide-svelte';

    let { isOpen = $bindable(false) } = $props();

    function closeGuide() {
        isOpen = false;
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div
        class="absolute inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm"
        onclick={closeGuide}
    >
        <div
            class="relative w-full max-w-lg border rounded-xl bg-gray-950/95 border-gray-800/80 shadow-2xl p-6"
            onclick={(e) => e.stopPropagation()}
        >
            <div class="flex items-center gap-3 mb-4">
                <div class="p-2 border rounded-lg bg-teal-500/10 border-teal-500/20 text-teal-400">
                    <FileSearch class="w-6 h-6" />
                </div>
                <div>
                    <h3 class="text-xl font-medium tracking-tight text-white">
                        {m.recon_scanner_guide_title()}
                    </h3>
                    <p class="text-sm text-gray-400">
                        {m.recon_scanner_subtitle()}
                    </p>
                </div>
            </div>

            <div class="space-y-4 mb-6">
                <p class="text-sm leading-relaxed text-gray-300">
                    {m.recon_scanner_guide_desc()}
                </p>
            </div>

            <div class="p-4 border sm:flex items-start gap-4 rounded-lg bg-indigo-500/5 border-indigo-500/20">
                <ShieldAlert class="w-6 h-6 mt-1 shrink-0 text-indigo-400" />
                <div>
                    <h4 class="mb-1 text-sm font-medium text-indigo-300">
                        {m.secops_context()}
                    </h4>
                    <p class="text-xs leading-relaxed text-indigo-200/80">
                        {m.recon_scanner_secops_tip()}
                    </p>
                </div>
            </div>

            <button
                onclick={closeGuide}
                class="w-full py-2.5 mt-6 text-sm font-medium transition-colors border rounded-lg text-gray-300 hover:text-white bg-gray-900 border-gray-800 hover:bg-gray-800"
            >
                {m.recon_subdomain_scan()}
            </button>
        </div>
    </div>
{/if}
