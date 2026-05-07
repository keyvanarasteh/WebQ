<script lang="ts">
    import { Shield, Activity, Target, AlertTriangle, UserX, Ban, PieChart } from "lucide-svelte";
    import { onMount, onDestroy } from "svelte";

    // Mock data for the dashboard
    let kpis = $state({
        totalEvents: 428,
        uniqueAttackers: 35,
        blockedPayloads: 412
    });

    let liveEvents = $state([
        { id: 1, time: new Date().toISOString(), category: "SQL Injection", severity: "Critical", attacker_ip: "192.168.1.104", payload: "admin' OR 1=1 --" },
        { id: 2, time: new Date(Date.now() - 5000).toISOString(), category: "XSS", severity: "High", attacker_ip: "10.0.0.5", payload: "<scr" + "ipt>alert(1)</scr" + "ipt>" },
        { id: 3, time: new Date(Date.now() - 15000).toISOString(), category: "Path Traversal", severity: "High", attacker_ip: "172.16.0.2", payload: "../../../../etc/passwd" },
    ]);

    let vectorDistribution = [
        { name: "SQLi", value: 35, color: "#ef4444" },
        { name: "XSS", value: 25, color: "#f97316" },
        { name: "RCE", value: 15, color: "#eab308" },
        { name: "LFI", value: 10, color: "#3b82f6" },
        { name: "Other", value: 15, color: "#a855f7" }
    ];

    // Simple SVG pie chart math
    let cumulativePercent = 0;
    const slices = vectorDistribution.map(slice => {
        const start = cumulativePercent;
        cumulativePercent += slice.value;
        const end = cumulativePercent;
        return {
            ...slice,
            start,
            end
        };
    });

    function getCoordinatesForPercent(percent: number) {
        const x = Math.cos(2 * Math.PI * percent);
        const y = Math.sin(2 * Math.PI * percent);
        return [x, y];
    }
</script>

<div class="space-y-6 max-w-7xl mx-auto p-6">
    <div class="text-center space-y-4 mb-10">
        <h1 class="text-5xl font-black tracking-tight text-primary-text">React2Shell <span class="text-yellow-500 drop-shadow-[0_0_15px_rgba(234,179,8,0.5)]">Telemetry</span></h1>
        <p class="text-muted text-lg">Dashboard Overview of Honeypot Intelligence</p>
    </div>

    <!-- KPI Counters -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg flex items-center gap-4">
            <div class="p-4 bg-blue-500/10 rounded-lg text-blue-500">
                <Activity class="w-8 h-8" />
            </div>
            <div>
                <div class="text-sm font-bold text-muted uppercase tracking-wider">Total Events</div>
                <div class="text-3xl font-black text-primary-text">{kpis.totalEvents}</div>
            </div>
        </div>
        <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg flex items-center gap-4">
            <div class="p-4 bg-purple-500/10 rounded-lg text-purple-500">
                <UserX class="w-8 h-8" />
            </div>
            <div>
                <div class="text-sm font-bold text-muted uppercase tracking-wider">Unique Attackers</div>
                <div class="text-3xl font-black text-primary-text">{kpis.uniqueAttackers}</div>
            </div>
        </div>
        <div class="bg-surface/30 border border-border/50 rounded-xl p-6 shadow-lg flex items-center gap-4">
            <div class="p-4 bg-green-500/10 rounded-lg text-green-500">
                <Ban class="w-8 h-8" />
            </div>
            <div>
                <div class="text-sm font-bold text-muted uppercase tracking-wider">Blocked Payloads</div>
                <div class="text-3xl font-black text-primary-text">{kpis.blockedPayloads}</div>
            </div>
        </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
        <!-- Live Threat Feed -->
        <div class="lg:col-span-2 bg-surface/30 border border-border/50 rounded-xl flex flex-col h-[500px] shadow-lg relative overflow-hidden">
            <div class="p-4 border-b border-border/50 bg-surface/50 flex items-center gap-3">
                <Shield class="w-5 h-5 text-red-500" />
                <h2 class="text-lg font-bold text-primary-text">Live Threat Feed</h2>
            </div>
            <div class="flex-1 overflow-y-auto p-4 custom-scrollbar space-y-3">
                {#each liveEvents as event}
                    <div class="bg-background border border-border/50 rounded-lg p-3 hover:border-red-500/30 transition-colors">
                        <div class="flex justify-between items-start mb-2">
                            <div class="flex items-center gap-2">
                                <span class="w-2 h-2 rounded-full {event.severity === 'Critical' ? 'bg-red-500 animate-pulse' : 'bg-orange-500'}"></span>
                                <span class="text-xs font-bold {event.severity === 'Critical' ? 'text-red-500' : 'text-orange-500'} uppercase tracking-wider">{event.severity}</span>
                                <span class="text-xs text-muted">|</span>
                                <span class="text-xs font-mono text-primary-text">{new Date(event.time).toLocaleTimeString()}</span>
                            </div>
                            <span class="text-xs font-mono bg-surface px-2 py-0.5 rounded border border-border">{event.attacker_ip}</span>
                        </div>
                        <div class="flex flex-wrap items-center gap-2 mb-2">
                            <span class="px-2 py-1 bg-red-500/10 text-red-400 text-xs font-bold rounded border border-red-500/20">{event.category}</span>
                        </div>
                        <div class="bg-surface/50 border border-border/50 rounded p-2 mt-2">
                            <div class="text-xs text-muted mb-1">Matched Payload</div>
                            <div class="font-mono text-xs text-red-400 break-all bg-background p-1.5 rounded border border-red-500/10">{event.payload}</div>
                        </div>
                    </div>
                {/each}
            </div>
        </div>

        <!-- Attack Vector Distribution Chart -->
        <div class="bg-surface/30 border border-border/50 rounded-xl flex flex-col h-[500px] shadow-lg relative overflow-hidden">
            <div class="p-4 border-b border-border/50 bg-surface/50 flex items-center gap-3">
                <PieChart class="w-5 h-5 text-blue-400" />
                <h2 class="text-lg font-bold text-primary-text">Vector Distribution</h2>
            </div>
            <div class="flex-1 flex flex-col items-center justify-center p-6">
                <!-- Simple CSS/SVG Pie Chart -->
                <div class="relative w-48 h-48 mb-8">
                    <svg viewBox="-1 -1 2 2" class="transform -rotate-90 w-full h-full">
                        {#each slices as slice}
                            {@const start = getCoordinatesForPercent(slice.start / 100)}
                            {@const end = getCoordinatesForPercent(slice.end / 100)}
                            {@const largeArcFlag = slice.value > 50 ? 1 : 0}
                            <path
                                d={`M ${start[0]} ${start[1]} A 1 1 0 ${largeArcFlag} 1 ${end[0]} ${end[1]} L 0 0`}
                                fill={slice.color}
                                class="hover:opacity-80 transition-opacity cursor-pointer"
                            />
                        {/each}
                    </svg>
                    <!-- Inner Circle for Donut Effect -->
                    <div class="absolute inset-0 m-auto w-24 h-24 bg-surface/50 backdrop-blur-sm rounded-full flex items-center justify-center shadow-inner">
                        <div class="text-center">
                            <div class="text-2xl font-black text-primary-text">100%</div>
                        </div>
                    </div>
                </div>
                
                <div class="w-full space-y-2">
                    {#each vectorDistribution as slice}
                        <div class="flex items-center justify-between text-sm">
                            <div class="flex items-center gap-2">
                                <div class="w-3 h-3 rounded-full" style="background-color: {slice.color}"></div>
                                <span class="text-primary-text">{slice.name}</span>
                            </div>
                            <span class="font-bold text-muted">{slice.value}%</span>
                        </div>
                    {/each}
                </div>
            </div>
        </div>
    </div>
</div>
