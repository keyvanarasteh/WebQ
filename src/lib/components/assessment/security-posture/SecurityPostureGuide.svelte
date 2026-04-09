<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, ShieldAlert, GraduationCap } from 'lucide-svelte';
    import { fade, slide } from 'svelte/transition';

    let { isOpen = $bindable(false) } = $props();
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div
        class="fixed inset-0 z-50 flex py-16 justify-center bg-black/60 backdrop-blur-sm overflow-y-auto"
        in:fade={{ duration: 200 }}
        out:fade={{ duration: 150 }}
        onclick={() => (isOpen = false)}
    >
        <div
            class="relative w-full max-w-2xl p-6 md:p-8 border border-white/10 bg-slate-950/90 shadow-2xl rounded-2xl h-fit m-auto"
            in:slide={{ duration: 300, axis: 'y' }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Close Button -->
            <button
                class="absolute top-4 right-4 p-2 text-slate-400 hover:text-white transition-colors rounded-lg hover:bg-white/5"
                onclick={() => (isOpen = false)}
                aria-label="Close Guide"
            >
                <X class="w-5 h-5" />
            </button>

            <!-- Guide Header -->
            <div class="flex items-center gap-4 mb-6 pb-6 border-b border-white/10">
                <div class="flex items-center justify-center w-12 h-12 rounded-xl bg-orange-500/20 text-orange-400">
                    <GraduationCap class="w-6 h-6" />
                </div>
                <div>
                    <h2 class="text-xl font-medium text-white mb-1">
                        {m.sec_posture_title()}
                    </h2>
                    <p class="text-sm text-slate-400">
                        Attack Surface Mapping & Misconfiguration Discovery
                    </p>
                </div>
            </div>

            <!-- Content Area -->
            <div class="space-y-6 text-sm text-slate-300">
                <div class="p-4 border sm:flex items-start gap-4 rounded-lg bg-orange-500/5 border-orange-500/20">
                    <ShieldAlert class="w-6 h-6 mt-1 shrink-0 text-orange-400" />
                    <div>
                        <h4 class="mb-1 font-medium text-orange-300">
                            {m.sec_posture_secops_context()}
                        </h4>
                        <p class="leading-relaxed text-orange-200/80">
                            {m.sec_posture_secops_tip()}
                        </p>
                    </div>
                </div>

                <div class="space-y-4">
                    <h3 class="text-base font-medium text-white">Why it matters?</h3>
                    <ul class="space-y-3 list-disc pl-5 marker:text-orange-500/50">
                        <li>
                            <strong class="text-slate-200">WAF Bypass:</strong> Weak or incorrectly configured Web Application Firewalls (like Cloudflare) can be bypassed if the origin IP is discovered, neutralizing entire security boundaries.
                        </li>
                        <li>
                            <strong class="text-slate-200">Missing Headers:</strong> Lack of HSTS, CSP, or X-Frame-Options leaves your application directly vulnerable to Man-in-the-Middle (MitM), Cross-Site Scripting (XSS), and Clickjacking attacks.
                        </li>
                        <li>
                            <strong class="text-slate-200">CORS Misconfigurations:</strong> A wildcard (`*`) <code class="px-1.5 py-0.5 rounded bg-white/10 text-xs font-mono">Access-Control-Allow-Origin</code> combined with credentials allows any malicious website to hijack authenticated user data.
                        </li>
                    </ul>
                </div>

                <div class="p-4 rounded-lg bg-white/5 border border-white/5 space-y-2 mt-4">
                    <h4 class="font-medium text-slate-200">Red Team Perspective</h4>
                    <p class="text-slate-400 italic">
                        "The overall grade helps rapidly identify 'soft targets' during reconnaissance. A 'D' or 'F' grade usually indicates a massive lack of DevSecOps pipelines, making it highly probable to find deeply rooted vulnerabilities beyond just headers."
                    </p>
                </div>
            </div>
        </div>
    </div>
{/if}
