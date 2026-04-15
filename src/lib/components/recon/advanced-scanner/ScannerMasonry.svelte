<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { KeyRound, ShieldAlert, Code, Link2, HelpCircle } from 'lucide-svelte';
    import AdvancedScannerGuide from '$lib/components/recon/advanced-scanner/AdvancedScannerGuide.svelte';

    export interface SecretFinding {
        pattern_name: string;
        matched_content: string;
        entropy: number;
        url: string;
    }

    export interface JsVulnerability {
        vuln_type: string;
        description: string;
        snippet: string;
        url: string;
    }

    export interface SsrfFinding {
        parameter: string;
        vector: string;
        url: string;
        description: string;
    }

    export interface ScannerResult {
        domain: string;
        secrets: SecretFinding[];
        js_vulnerabilities: JsVulnerability[];
        ssrf_vulnerabilities: SsrfFinding[];
        api_endpoints_discovered: string[];
        summary: { scans_completed: number; total_findings: number };
    }

    interface Props {
        result: ScannerResult | null;
        isLoading?: boolean;
    }

    let { result, isLoading = false }: Props = $props();
    let isGuideOpen = $state(false);

    function formatText(t: string) {
        if (!t) return "";
        if (t.length > 50) return t.substring(0, 50) + "...";
        return t;
    }

    let totalEmpty = $derived(
        !result ||
        (result.secrets.length === 0 &&
            result.js_vulnerabilities.length === 0 &&
            result.ssrf_vulnerabilities.length === 0 &&
            result.api_endpoints_discovered.length === 0)
    );
</script>

<AdvancedScannerGuide bind:isOpen={isGuideOpen} />

<div class="grid grid-cols-1 gap-6 mt-8 md:grid-cols-2">
    <!-- 1. Secrets Column -->
    <div class="space-y-4">
        <div class="flex items-center justify-between mb-4 pb-2 border-b border-fuchsia-500/10">
            <div class="flex items-center gap-2">
                <KeyRound class="w-5 h-5 text-fuchsia-400" />
                <h3 class="font-medium text-primary-text">{m.recon_scanner_badge_secrets()} ({result?.secrets.length || 0})</h3>
            </div>
            <button onclick={() => isGuideOpen = true} class="p-1 hover:bg-fuchsia-500/10 rounded-full text-fuchsia-400 transition-colors" title="Guide">
                <HelpCircle class="size-4" />
            </button>
        </div>
        
        {#if !result}
            <div class="flex flex-col items-center justify-center py-8 text-muted border border-dashed border-base rounded-lg bg-background/50">
                <p class="text-sm font-fira tracking-widest uppercase font-bold text-base-text mb-1">Secrets Waiting</p>
                <p class="text-xs text-center max-w-[200px]">Initiate advanced scan</p>
            </div>
        {:else if result.secrets.length === 0}
            <p class="text-sm italic text-muted">No secrets found.</p>
        {:else}
            <div class="flex flex-col gap-3">
                {#each result.secrets as sec (sec.pattern_name + sec.matched_content.substring(0, 10))}
                    <div class="p-4 border border-fuchsia-500/20 bg-fuchsia-500/5 rounded-xl">
                        <div class="flex justify-between items-start mb-2">
                            <span class="px-2 py-0.5 text-xs font-semibold rounded bg-fuchsia-500/20 text-fuchsia-300 border border-fuchsia-500/20">{sec.pattern_name}</span>
                            <span class="text-xs text-muted font-mono" title="Shannon Entropy">H: {sec.entropy.toFixed(2)}</span>
                        </div>
                        <p class="text-sm font-mono text-primary-text break-all">{sec.matched_content}</p>
                        <p class="mt-2 text-xs truncate text-muted hover:text-muted" title={sec.url}>{sec.url}</p>
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <!-- 2. JS Vulnerabilities -->
    <div class="space-y-4">
        <div class="flex items-center justify-between mb-4 pb-2 border-b border-amber-500/10">
            <div class="flex items-center gap-2">
                <Code class="w-5 h-5 text-amber-400" />
                <h3 class="font-medium text-primary-text">{m.recon_scanner_badge_js()} ({result?.js_vulnerabilities.length || 0})</h3>
            </div>
            <button onclick={() => isGuideOpen = true} class="p-1 hover:bg-amber-500/10 rounded-full text-amber-400 transition-colors" title="Guide">
                <HelpCircle class="size-4" />
            </button>
        </div>
        
        {#if !result}
            <div class="flex flex-col items-center justify-center py-8 text-muted border border-dashed border-base rounded-lg bg-background/50">
                <p class="text-sm font-fira tracking-widest uppercase font-bold text-base-text mb-1">JS Vectors Waiting</p>
                <p class="text-xs text-center max-w-[200px]">Initiate advanced scan</p>
            </div>
        {:else if result.js_vulnerabilities.length === 0}
            <p class="text-sm italic text-muted">No JS vulnerabilities found.</p>
        {:else}
            <div class="flex flex-col gap-3">
                {#each result.js_vulnerabilities as vuln (vuln.vuln_type + vuln.snippet.substring(0, 10))}
                    <div class="p-4 border border-amber-500/20 bg-amber-500/5 rounded-xl">
                        <span class="inline-block px-2 py-0.5 mb-2 text-xs font-semibold rounded bg-amber-500/20 text-amber-300 border border-amber-500/20">{vuln.vuln_type}</span>
                        <p class="text-sm text-primary-text">{vuln.description}</p>
                        {#if vuln.snippet}
                            <div class="p-2 mt-2 font-mono text-xs border rounded bg-glass border-base text-muted break-all">
                                {formatText(vuln.snippet)}
                            </div>
                        {/if}
                        <p class="mt-2 text-xs truncate text-muted hover:text-muted" title={vuln.url}>{vuln.url}</p>
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <!-- 3. SSRF Findings -->
    <div class="space-y-4">
        <div class="flex items-center justify-between mb-4 pb-2 border-b border-rose-500/10">
            <div class="flex items-center gap-2">
                <ShieldAlert class="w-5 h-5 text-rose-400" />
                <h3 class="font-medium text-primary-text">{m.recon_scanner_badge_ssrf()} ({result?.ssrf_vulnerabilities.length || 0})</h3>
            </div>
            <button onclick={() => isGuideOpen = true} class="p-1 hover:bg-rose-500/10 rounded-full text-rose-400 transition-colors" title="Guide">
                <HelpCircle class="size-4" />
            </button>
        </div>
        
        {#if !result}
            <div class="flex flex-col items-center justify-center py-8 text-muted border border-dashed border-base rounded-lg bg-background/50">
                <p class="text-sm font-fira tracking-widest uppercase font-bold text-base-text mb-1">SSRF Waiting</p>
                <p class="text-xs text-center max-w-[200px]">Initiate advanced scan</p>
            </div>
        {:else if result.ssrf_vulnerabilities.length === 0}
            <p class="text-sm italic text-muted">No SSRF vectors found.</p>
        {:else}
            <div class="flex flex-col gap-3">
                {#each result.ssrf_vulnerabilities as ssrf (ssrf.parameter + ssrf.url)}
                    <div class="p-4 border border-rose-500/20 bg-rose-500/5 rounded-xl">
                        <div class="flex items-center gap-2 mb-2">
                            <span class="px-2 py-0.5 text-xs font-semibold rounded bg-rose-500/20 text-rose-300 border border-rose-500/20">param: {ssrf.parameter}</span>
                        </div>
                        <p class="text-sm text-primary-text">{ssrf.description}</p>
                        <div class="p-2 mt-2 font-mono text-xs border rounded bg-glass border-base text-muted break-all">
                            {formatText(ssrf.vector)}
                        </div>
                        <p class="mt-2 text-xs truncate text-muted hover:text-muted" title={ssrf.url}>{ssrf.url}</p>
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <!-- 4. API Endpoints -->
    <div class="space-y-4">
        <div class="flex items-center justify-between mb-4 pb-2 border-b border-sky-500/10">
            <div class="flex items-center gap-2">
                <Link2 class="w-5 h-5 text-sky-400" />
                <h3 class="font-medium text-primary-text">{m.recon_scanner_badge_apis()} ({result?.api_endpoints_discovered.length || 0})</h3>
            </div>
            <button onclick={() => isGuideOpen = true} class="p-1 hover:bg-sky-500/10 rounded-full text-sky-400 transition-colors" title="Guide">
                <HelpCircle class="size-4" />
            </button>
        </div>
        
        {#if !result}
            <div class="flex flex-col items-center justify-center py-8 text-muted border border-dashed border-base rounded-lg bg-background/50">
                <p class="text-sm font-fira tracking-widest uppercase font-bold text-base-text mb-1">APIs Waiting</p>
                <p class="text-xs text-center max-w-[200px]">Initiate advanced scan</p>
            </div>
        {:else if result.api_endpoints_discovered.length === 0}
            <p class="text-sm italic text-muted">No API endpoints discovered.</p>
        {:else}
            <div class="flex flex-col gap-3">
                <div class="p-4 border bg-surface/50 border-base rounded-xl">
                    <ul class="space-y-2">
                        {#each result.api_endpoints_discovered as endpoint, i (i)}
                            <li class="flex items-start gap-2">
                                <span class="text-sky-500 mt-1">↳</span>
                                <span class="font-mono text-sm text-primary-text break-all">{endpoint}</span>
                            </li>
                        {/each}
                    </ul>
                </div>
            </div>
        {/if}
    </div>
</div>
