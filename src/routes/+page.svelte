<script lang="ts">
  import { Shield, Activity, Target } from "lucide-svelte";
  // @ts-expect-error
  import { resolve } from "$app/paths";
</script>

<div class="text-center space-y-4 mb-10">
  <h1 class="text-5xl font-black tracking-tight text-primary-text">WebQ <span class="text-accent drop-shadow-[0_0_15px_rgba(34,211,238,0.5)]">Core</span></h1>
  <p class="text-muted text-lg">Central OSINT and SecOps Dashboard.</p>
</div>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto">
    <!-- Engine Health Widget -->
    <a href={resolve('/settings/dependencies')} class="bg-surface/30 border border-cyan-500/30 rounded-xl p-6 hover:bg-surface/50 hover:border-cyan-500/60 transition-all shadow-lg group relative overflow-hidden">
        <div class="absolute -right-10 -top-10 w-32 h-32 bg-linear-to-br from-transparent to-cyan-500/10 rounded-full blur-xl group-hover:to-cyan-400/20 transition-all"></div>
        <div class="flex items-center gap-4 mb-4 relative z-10">
            <div class="p-3 bg-background rounded-lg shadow-inner">
                <Activity class="w-6 h-6 text-cyan-400 group-hover:drop-shadow-[0_0_10px_rgba(34,211,238,0.8)]" />
            </div>
            <h2 class="text-xl font-bold text-primary-text">Engine Health</h2>
        </div>
        <p class="text-secondary-text text-sm mb-4 relative z-10">Monitor OS dependencies (Nmap, Subfinder, Dig, OpenSSL) for full scan coverage.</p>
        <div class="text-xs uppercase tracking-widest text-cyan-500 font-semibold relative z-10">Check Telemetry &rarr;</div>
    </a>

    <!-- New Scan Widget -->
    <a href={resolve('/intelligence/domain-info')} class="bg-surface/30 border border-base rounded-xl p-6 hover:bg-surface/50 transition-all shadow-lg group">
        <div class="flex items-center gap-4 mb-4">
            <div class="p-3 bg-background rounded-lg shadow-inner">
                <Target class="w-6 h-6 text-fuchsia-400 group-hover:drop-shadow-[0_0_10px_rgba(232,121,249,0.8)]" />
            </div>
            <h2 class="text-xl font-bold text-primary-text">New Scan</h2>
        </div>
        <p class="text-secondary-text text-sm mb-4">Launch Domain OSINT, Nmap Zero-Day mapping or automated Subdomain discovery.</p>
        <div class="text-xs uppercase tracking-widest text-fuchsia-400 font-semibold">Start Recon &rarr;</div>
    </a>

    <!-- Scan History Database Widget -->
    <a href={resolve('/history')} class="bg-surface/30 border border-emerald-500/30 rounded-xl p-6 hover:bg-surface/50 hover:border-emerald-500/60 transition-all shadow-lg group relative overflow-hidden">
        <div class="absolute -right-10 -top-10 w-32 h-32 bg-linear-to-br from-transparent to-emerald-500/10 rounded-full blur-xl group-hover:to-emerald-400/20 transition-all"></div>
        <div class="flex items-center gap-4 mb-4 relative z-10">
            <div class="p-3 bg-background rounded-lg shadow-inner">
                <Shield class="w-6 h-6 text-emerald-400 group-hover:drop-shadow-[0_0_10px_rgba(16,185,129,0.8)]" />
            </div>
            <h2 class="text-xl font-bold text-primary-text">Scan History</h2>
        </div>
        <p class="text-secondary-text text-sm mb-4 relative z-10">Review past vulnerability assessments and intelligence reports securely from the SQLite DB.</p>
        <div class="text-xs uppercase tracking-widest text-emerald-400 font-semibold flex items-center gap-2 relative z-10">
            <span class="w-2 h-2 rounded-full bg-emerald-500 shadow-[0_0_8px_rgba(16,185,129,0.8)]"></span> Database Active &rarr;
        </div>
    </a>
</div>
