<script lang="ts">
    import * as m from '$lib/paraglide/messages';

    let { vulnerabilities = [] }: { vulnerabilities: Array<{vuln_type: string, severity: string}> } = $props();

    // Using Svelte 5 derived state to map categories.
    let categories = $derived.by(() => {
        let sql = 0, xss = 0, ssrf = 0, lfi = 0, auth = 0;
        let c_sql = 0, c_xss = 0, c_ssrf = 0, c_lfi = 0, c_auth = 0;
        
        for (let v of vulnerabilities) {
            let t = v.vuln_type.toUpperCase();
            if (t.includes('SQL')) { sql++; if (v.severity === 'CRITICAL') c_sql++; }
            else if (t.includes('XSS')) { xss++; if (v.severity === 'CRITICAL') c_xss++; }
            else if (t.includes('SSRF')) { ssrf++; if (v.severity === 'CRITICAL') c_ssrf++; }
            else if (t.includes('LFI') || t.includes('PATH') || t.includes('XXE')) { lfi++; if (v.severity === 'CRITICAL') c_lfi++; }
            else { auth++; if (v.severity === 'CRITICAL') c_auth++; }
        }

        return [
            { name: "SQL Injection", count: sql, critical: c_sql },
            { name: "Cross-Site Scripting", count: xss, critical: c_xss },
            { name: "SSRF / SSRF", count: ssrf, critical: c_ssrf },
            { name: "LFI / Traversal", count: lfi, critical: c_lfi },
            { name: "Auth Bypass / Config", count: auth, critical: c_auth },
        ];
    });

    let totalVulns = $derived(vulnerabilities.length);
</script>

<div class="border border-subtle bg-glass p-6 rounded-xl backdrop-blur-xl h-full flex flex-col justify-center">
    <h3 class="text-primary-text font-medium mb-6 flex items-center gap-2">
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="text-rose-400"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
        Vulnerability Distribution
    </h3>
    
    <div class="space-y-5">
        {#each categories as c}
            <div>
                <div class="flex justify-between items-center mb-2">
                    <span class="text-sm font-medium {c.count > 0 ? (c.critical > 0 ? 'text-red-400' : 'text-rose-400') : 'text-muted'} transition-colors">{c.name}</span>
                    <span class="text-xs font-mono {c.count > 0 ? 'text-primary-text' : 'text-muted'}">{c.count} Found</span>
                </div>
                <div class="h-2.5 w-full bg-glass rounded-full overflow-hidden shadow-inner">
                    <div 
                        class="h-full rounded-full transition-all duration-1000 ease-out {c.count > 0 ? (c.critical > 0 ? 'bg-gradient-to-r from-red-600 to-red-500 shadow-[0_0_10px_rgba(239,68,68,0.5)]' : 'bg-gradient-to-r from-rose-600 to-rose-400 shadow-[0_0_10px_rgba(244,63,94,0.3)]') : 'bg-transparent'}"
                        style="width: {totalVulns > 0 ? Math.max((c.count / totalVulns) * 100, c.count > 0 ? 5 : 0) : 0}%"
                    ></div>
                </div>
            </div>
        {/each}
        {#if totalVulns === 0}
            <div class="pt-4 text-center text-muted text-sm">
                Awaiting active scan sequence.
            </div>
        {/if}
    </div>
</div>
