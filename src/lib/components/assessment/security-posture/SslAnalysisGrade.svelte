<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Lock, LockOpen, Server, ShieldCheck, ShieldAlert } from 'lucide-svelte';

    let { ssl }: {
        ssl: {
            ssl_available: boolean,
            protocol_version?: string,
            cipher_suite?: string,
            cipher_strength: string,
            overall_grade: string,
            subject?: string,
            issuer?: string
        }
    } = $props();

    function getGradeColor(grade: string) {
        if (grade.startsWith('A')) return 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20';
        if (grade.startsWith('B')) return 'text-teal-400 bg-teal-500/10 border-teal-500/20';
        if (grade.startsWith('C')) return 'text-yellow-400 bg-yellow-500/10 border-yellow-500/20';
        return 'text-red-400 bg-red-500/10 border-red-500/20';
    }
</script>

<div class="border border-subtle bg-glass backdrop-blur-md rounded-xl p-6 relative overflow-hidden group">
    <div class="absolute -right-10 -top-10 opacity-[0.03] group-hover:opacity-10 transition-opacity pointer-events-none">
        <Lock size={150} />
    </div>

    <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
            <div class={ssl.ssl_available ? "text-emerald-400" : "text-red-400"}>
                {#if ssl.ssl_available}
                    <Lock class="w-5 h-5" />
                {:else}
                    <LockOpen class="w-5 h-5" />
                {/if}
            </div>
            <h3 class="text-lg font-medium text-primary-text">SSL / TLS Configuration</h3>
        </div>
        {#if ssl.ssl_available}
            <div class={`px-3 py-1 rounded-full border font-bold text-sm ${getGradeColor(ssl.overall_grade)}`}>
                Grade {ssl.overall_grade}
            </div>
        {/if}
    </div>

    {#if ssl.ssl_available}
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div class="bg-surface/50 border border-subtle rounded-lg p-3">
                <span class="text-xs text-muted uppercase tracking-wider block mb-1">Protocol Version</span>
                <span class="text-sm font-mono text-cyan-400 uppercase">{ssl.protocol_version || 'Unknown'}</span>
            </div>
            <div class="bg-surface/50 border border-subtle rounded-lg p-3">
                <span class="text-xs text-muted uppercase tracking-wider block mb-1">Cipher Strength</span>
                <span class="text-sm font-mono text-cyan-400 capitalize">{ssl.cipher_strength}</span>
            </div>
            <div class="bg-surface/50 border border-subtle rounded-lg p-3 md:col-span-2">
                <span class="text-xs text-muted uppercase tracking-wider block mb-1">Cipher Suite</span>
                <span class="text-xs font-mono text-secondary-text truncate block" title={ssl.cipher_suite}>
                    {ssl.cipher_suite || 'Unknown'}
                </span>
            </div>
            {#if ssl.issuer || ssl.subject}
                <div class="bg-surface/50 border border-subtle rounded-lg p-3 md:col-span-2 flex flex-col gap-2 mt-2">
                    {#if ssl.subject}
                        <div class="flex items-center gap-2">
                            <Server size={14} class="text-muted shrink-0"/>
                            <span class="text-xs text-muted shrink-0">Subject:</span>
                            <span class="text-xs font-mono text-primary-text truncate" title={ssl.subject}>{ssl.subject}</span>
                        </div>
                    {/if}
                    {#if ssl.issuer}
                        <div class="flex items-center gap-2">
                            <ShieldCheck size={14} class="text-emerald-500 shrink-0"/>
                            <span class="text-xs text-muted shrink-0">Issuer:</span>
                            <span class="text-xs font-mono text-primary-text truncate" title={ssl.issuer}>{ssl.issuer}</span>
                        </div>
                    {/if}
                </div>
            {/if}
        </div>
    {:else}
        <div class="p-4 bg-red-500/10 border border-red-500/20 rounded-lg flex items-start gap-3">
            <ShieldAlert class="w-5 h-5 text-red-400 shrink-0 mt-0.5" />
            <p class="text-sm text-red-200">No verifiable SSL/TLS certificate was found on this endpoint. The connection is vulnerable to interception.</p>
        </div>
    {/if}
</div>
