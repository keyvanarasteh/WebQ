<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Cookie, Network, AlertTriangle, Shield } from 'lucide-svelte';

    let { corsPolicy, cookieSecurity }: {
        corsPolicy: { configured: boolean, headers: Record<string, string>, issues: string[], security_level: string },
        cookieSecurity: { cookies_present: boolean, security_issues: string[], security_score: number }
    } = $props();
</script>

<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <!-- CORS Analysis -->
    <div class="border border-white/5 bg-white/5 backdrop-blur-md rounded-xl p-6">
        <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-3">
                <Network class="w-5 h-5 text-fuchsia-400" />
                <h3 class="text-lg font-medium text-white">{m.sec_posture_cors()}</h3>
            </div>
            <span class="px-2.5 py-1 text-[10px] uppercase tracking-wider font-bold rounded-full {corsPolicy.security_level === 'High' ? 'bg-emerald-500/20 text-emerald-300' : 'bg-yellow-500/20 text-yellow-300'}">
                {corsPolicy.security_level}
            </span>
        </div>

        {#if corsPolicy.issues.length > 0}
            <div class="space-y-3 mb-4">
                {#each corsPolicy.issues as issue}
                    <div class="flex items-start gap-2 p-2.5 rounded text-sm bg-red-500/10 border border-red-500/20 text-red-300">
                        <AlertTriangle class="w-4 h-4 shrink-0 mt-0.5" />
                        <span class="leading-snug">{issue}</span>
                    </div>
                {/each}
            </div>
        {/if}

        <div class="space-y-1">
            {#each Object.entries(corsPolicy.headers) as [key, value] (key)}
                <div class="flex justify-between items-center py-2 border-b border-white/5 last:border-0">
                    <span class="text-xs font-mono text-slate-400 capitalize">{key.replace(/-/g, ' ')}</span>
                    <span class="text-xs font-mono {value === 'Not Set' ? 'text-slate-600' : 'text-fuchsia-300 truncate max-w-[200px]'}">
                        {value}
                    </span>
                </div>
            {/each}
        </div>
    </div>

    <!-- Cookie Security -->
    <div class="border border-white/5 bg-white/5 backdrop-blur-md rounded-xl p-6">
        <div class="flex items-center justify-between mb-4">
            <div class="flex items-center gap-3">
                <Cookie class="w-5 h-5 text-amber-400" />
                <h3 class="text-lg font-medium text-white">{m.sec_posture_cookies()}</h3>
            </div>
            {#if cookieSecurity.cookies_present}
                <span class="px-2.5 py-1 text-[10px] uppercase font-bold rounded-full bg-white/10 text-slate-300">
                    Score: {cookieSecurity.security_score}
                </span>
            {/if}
        </div>

        {#if cookieSecurity.cookies_present}
            {#if cookieSecurity.security_issues.length > 0}
                <div class="space-y-3">
                    {#each cookieSecurity.security_issues as issue}
                        <div class="flex items-center gap-2 p-2.5 rounded text-sm bg-amber-500/10 border border-amber-500/20 text-amber-300">
                            <AlertTriangle class="w-4 h-4 shrink-0" />
                            <span>{issue}</span>
                        </div>
                    {/each}
                </div>
            {:else}
                <div class="p-4 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-emerald-300 text-sm flex items-center gap-2">
                    <Shield class="w-4 h-4" />
                    All target cookies are secured (HttpOnly, Secure, SameSite).
                </div>
            {/if}
        {:else}
            <div class="py-8 text-center text-sm text-slate-500 italic">
                No Set-Cookie headers detected on target.
            </div>
        {/if}
    </div>
</div>
