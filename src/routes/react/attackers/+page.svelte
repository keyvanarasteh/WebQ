<script lang="ts">
    import { ShieldAlert, Cpu, Globe, Activity, Terminal } from "lucide-svelte";

    // Mock AttackerProfile data
    let topAttackers = $state([
        {
            ip: "192.168.1.104",
            risk_score: 95.5,
            total_requests: 1420,
            is_automated: true,
            browser_fingerprint: {
                os: "Linux x86_64",
                browser: "Chrome 114 (Headless)",
                is_headless: true
            },
            geographic_origin: "Russia",
            primary_vector: "SQL Injection",
            last_seen: new Date(Date.now() - 10000).toISOString()
        },
        {
            ip: "10.0.0.5",
            risk_score: 82.0,
            total_requests: 850,
            is_automated: false,
            browser_fingerprint: {
                os: "Windows 10",
                browser: "Firefox 112",
                is_headless: false
            },
            geographic_origin: "United States",
            primary_vector: "XSS",
            last_seen: new Date(Date.now() - 45000).toISOString()
        },
        {
            ip: "172.16.0.2",
            risk_score: 65.3,
            total_requests: 310,
            is_automated: true,
            browser_fingerprint: {
                os: "macOS 13.4",
                browser: "Safari 16.5",
                is_headless: false
            },
            geographic_origin: "China",
            primary_vector: "Path Traversal",
            last_seen: new Date(Date.now() - 120000).toISOString()
        }
    ]);

    function formatTime(iso: string) {
        return new Date(iso).toLocaleTimeString();
    }
</script>

<div class="space-y-6 max-w-7xl mx-auto p-6">
    <div class="text-center space-y-4 mb-10">
        <h1 class="text-5xl font-black tracking-tight text-primary-text">Attacker <span class="text-purple-500 drop-shadow-[0_0_15px_rgba(168,85,247,0.5)]">Profiling</span></h1>
        <p class="text-muted text-lg">Detailed analysis of threat actors and automated scanners.</p>
    </div>

    <div class="bg-surface/30 border border-border/50 rounded-xl shadow-lg relative overflow-hidden">
        <div class="p-4 border-b border-border/50 bg-surface/50 flex items-center justify-between">
            <div class="flex items-center gap-3">
                <ShieldAlert class="w-5 h-5 text-purple-500" />
                <h2 class="text-lg font-bold text-primary-text">Threat Actor Database</h2>
            </div>
            <span class="px-2 py-0.5 rounded-full bg-purple-500/10 text-purple-500 text-xs font-bold border border-purple-500/20">
                {topAttackers.length} Profiles
            </span>
        </div>
        
        <div class="overflow-x-auto">
            <table class="w-full text-left border-collapse">
                <thead>
                    <tr class="border-b border-border/50 bg-background/50">
                        <th class="p-4 text-xs font-bold text-muted uppercase tracking-wider">Attacker IP</th>
                        <th class="p-4 text-xs font-bold text-muted uppercase tracking-wider">Risk Score</th>
                        <th class="p-4 text-xs font-bold text-muted uppercase tracking-wider">Origin</th>
                        <th class="p-4 text-xs font-bold text-muted uppercase tracking-wider">Activity</th>
                        <th class="p-4 text-xs font-bold text-muted uppercase tracking-wider">Fingerprint</th>
                        <th class="p-4 text-xs font-bold text-muted uppercase tracking-wider">Last Seen</th>
                    </tr>
                </thead>
                <tbody class="divide-y divide-border/50">
                    {#each topAttackers as profile}
                        <tr class="hover:bg-surface/30 transition-colors">
                            <td class="p-4">
                                <div class="font-mono font-bold text-primary-text flex items-center gap-2">
                                    <Terminal class="w-4 h-4 text-muted" /> {profile.ip}
                                </div>
                            </td>
                            <td class="p-4">
                                <div class="flex items-center gap-2">
                                    <span class="font-black text-lg {profile.risk_score > 90 ? 'text-red-500' : profile.risk_score > 70 ? 'text-orange-500' : 'text-yellow-500'}">
                                        {profile.risk_score.toFixed(1)}
                                    </span>
                                </div>
                            </td>
                            <td class="p-4">
                                <div class="flex items-center gap-2 text-primary-text text-sm">
                                    <Globe class="w-4 h-4 text-blue-400" /> {profile.geographic_origin}
                                </div>
                            </td>
                            <td class="p-4">
                                <div class="flex flex-col gap-1 text-xs">
                                    <span class="text-muted"><span class="font-bold text-primary-text">{profile.total_requests}</span> requests</span>
                                    <span class="px-2 py-0.5 rounded w-fit {profile.is_automated ? 'bg-yellow-500/10 text-yellow-500 border border-yellow-500/20' : 'bg-green-500/10 text-green-500 border border-green-500/20'}">
                                        {profile.is_automated ? 'Automated Scanner' : 'Manual Interactor'}
                                    </span>
                                </div>
                            </td>
                            <td class="p-4">
                                <div class="flex flex-col gap-1 text-xs font-mono text-muted">
                                    <div class="flex items-center gap-1">
                                        <Cpu class="w-3 h-3 text-blue-400" /> {profile.browser_fingerprint.os}
                                    </div>
                                    <div class="flex items-center gap-1">
                                        <Globe class="w-3 h-3 text-purple-400" /> {profile.browser_fingerprint.browser}
                                        {#if profile.browser_fingerprint.is_headless}
                                            <span class="ml-1 px-1 py-0.5 bg-red-500/10 text-red-400 rounded border border-red-500/20 text-[10px]">Headless</span>
                                        {/if}
                                    </div>
                                </div>
                            </td>
                            <td class="p-4 text-sm text-primary-text font-mono">
                                {formatTime(profile.last_seen)}
                            </td>
                        </tr>
                    {/each}
                    {#if topAttackers.length === 0}
                        <tr>
                            <td colspan="6" class="p-8 text-center text-muted">
                                <Activity class="w-8 h-8 mx-auto mb-2 opacity-50" />
                                <p>No attacker profiles generated yet.</p>
                            </td>
                        </tr>
                    {/if}
                </tbody>
            </table>
        </div>
    </div>
</div>
