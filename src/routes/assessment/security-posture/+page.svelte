<script lang="ts">
    import { invoke } from '@tauri-apps/api/core';
    import { Activity, Globe, Info, ShieldAlert } from 'lucide-svelte';
    import * as m from '$lib/paraglide/messages';
    
    // Components
    import OverallSecurityGrade from '$lib/components/assessment/security-posture/OverallSecurityGrade.svelte';
    import WafBypassStatus from '$lib/components/assessment/security-posture/WafBypassStatus.svelte';
    import CorsCookieAnalysis from '$lib/components/assessment/security-posture/CorsCookieAnalysis.svelte';
    import HeadersAnalysis from '$lib/components/assessment/security-posture/HeadersAnalysis.svelte';
    import SecurityPostureGuide from '$lib/components/assessment/security-posture/SecurityPostureGuide.svelte';
    import SslAnalysisGrade from '$lib/components/assessment/security-posture/SslAnalysisGrade.svelte';
    import HttpMethodsInfo from '$lib/components/assessment/security-posture/HttpMethodsInfo.svelte';
    import ServerInfoGrid from '$lib/components/assessment/security-posture/ServerInfoGrid.svelte';
    import VulnScanLog from '$lib/components/assessment/security-posture/VulnScanLog.svelte';
    import { fade } from 'svelte/transition';
    import { listen, type UnlistenFn } from '@tauri-apps/api/event';
    import ScanTerminal from '$lib/components/ui/ScanTerminal.svelte';
    import type { ScanProgressEvent } from '$lib/types/intelligence';

    type SecurityAnalysisResult = {
        domain: string;
        https_available: boolean;
        https_redirect: boolean;
        waf_detection: any;
        security_headers: any;
        ssl_analysis: any;
        cors_policy: any;
        cookie_security: any;
        http_methods: any;
        server_information: any;
        vulnerability_scan: any;
        security_score: any;
        recommendations: string[];
    };

    let targetDomain = $state('');
    let status = $state<'idle' | 'loading' | 'success' | 'error'>('idle');
    let errorMessage = $state('');
    let results = $state<SecurityAnalysisResult | null>(null);

    // Guide Modal
    let showGuide = $state(false);

    let logs = $state<ScanProgressEvent[]>([]);
    let progressPercent = $state(0);

    $effect(() => {
        let unlistenP: UnlistenFn | null = null;
        listen<ScanProgressEvent>('scan-progress', (event) => {
            logs.push(event.payload);
            progressPercent = event.payload.percentage;
        }).then(u => unlistenP = u);
        return () => { if (unlistenP) unlistenP(); };
    });

    async function scanTarget() {
        if (!targetDomain) return;
        
        status = 'loading';
        errorMessage = '';
        results = null;
        logs = [];
        progressPercent = 0;
        logs = [];
        progressPercent = 0;

        try {
            results = await invoke<SecurityAnalysisResult>('scan_security_posture', { domain: targetDomain });
            status = 'success';
        } catch (err: unknown) {
            status = 'error';
            errorMessage = err instanceof Error ? err.message : String(err);
        }
    }
</script>

<svelte:head>
    <title>{m.sec_posture_title()} | WebQ</title>
</svelte:head>

<div class="h-full flex flex-col p-4 sm:p-6 lg:p-8 max-w-7xl mx-auto w-full">
    <!-- Header -->
    <header class="mb-8 flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4">
        <div>
            <h1 class="text-2xl font-semibold tracking-tight text-primary-text flex items-center gap-2">
                <Activity class="w-6 h-6 text-orange-400" />
                {m.sec_posture_title()}
            </h1>
            <p class="text-sm text-muted mt-1">Audit target HTTP security structures, TLS layers, and WAF presence.</p>
        </div>
        <button
            onclick={() => (showGuide = true)}
            class="flex items-center gap-2 px-3 py-1.5 text-sm font-medium text-secondary-text transition-colors border rounded-lg bg-surface hover:bg-surface-hover border-glass"
        >
            <Info class="w-4 h-4" />
            Understanding Posture
        </button>
    </header>

    <!-- Scan Input -->
    <div class="flex gap-3 mb-8">
        <div class="relative flex-1">
            <Globe class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted" />
            <input 
                type="text" 
                bind:value={targetDomain}
                placeholder="example.com"
                onkeydown={(e) => e.key === 'Enter' && scanTarget()}
                class="w-full pl-10 pr-4 py-2.5 bg-glass border border-glass rounded-lg text-primary-text placeholder-muted focus:outline-none focus:ring-2 focus:ring-orange-500/50 transition-shadow"
                disabled={status === 'loading'}
            />
        </div>
        <button 
            onclick={scanTarget}
            disabled={status === 'loading' || !targetDomain}
            class="px-6 py-2.5 bg-orange-500 hover:bg-orange-600 disabled:opacity-50 disabled:cursor-not-allowed text-primary-text font-medium rounded-lg transition-colors flex items-center gap-2"
        >
            {#if status === 'loading'}
                <div class="w-4 h-4 border-2 border-white/20 border-t-white rounded-full animate-spin"></div>
                Analyzing...
            {:else}
                Scan Target
            {/if}
        </button>
    </div>

    <!-- Error State -->
    {#if status === 'error'}
        <div class="p-4 mb-8 border rounded-lg bg-red-500/10 border-red-500/20 text-red-400 flex items-start gap-3">
            <ShieldAlert class="w-5 h-5 shrink-0" />
            <p>{errorMessage}</p>
        </div>
    {/if}

    <!-- Empty State -->
    {#if status === 'idle'}
        <div class="flex flex-col items-center justify-center flex-1 min-h-[300px] border border-dashed rounded-xl border-glass bg-glass">
            <Activity class="w-12 h-12 mb-4 text-tertiary-text" />
            <h3 class="text-lg font-medium text-secondary-text mb-1">{m.val_waiting()}</h3>
            <p class="text-sm text-muted">Enter a target domain above to perform a full security grading.</p>
        </div>
    {/if}

    <!-- Results State -->
    {#if status === 'success' && results}
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- Left Column: Master Grade & WAF -->
            <div class="space-y-6 lg:col-span-1">
                <OverallSecurityGrade 
                    score={results.security_score.overall_score}
                    grade={results.security_score.grade}
                    riskLevel={results.security_score.risk_level}
                    recommendations={results.recommendations}
                    httpsAvailable={results.https_available}
                    httpsRedirect={results.https_redirect}
                />
                <WafBypassStatus wafResult={results.waf_detection} />
            </div>

            <!-- Right Column: Micro Analysis -->
            <div class="space-y-6 lg:col-span-2">
                <SslAnalysisGrade ssl={results.ssl_analysis} />
                <ServerInfoGrid serverInfo={results.server_information} />
                <CorsCookieAnalysis 
                    corsPolicy={results.cors_policy}
                    cookieSecurity={results.cookie_security}
                />
                <HttpMethodsInfo methods={results.http_methods} />
                <HeadersAnalysis 
                    headersAnalysis={results.security_headers}
                />
                <VulnScanLog vulnScan={results.vulnerability_scan} />
            </div>
        </div>
    {/if}

    <SecurityPostureGuide bind:isOpen={showGuide} />
</div>
