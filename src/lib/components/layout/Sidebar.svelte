<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
    // @ts-expect-error SvelteKit generated module not playing nice with TS ambient types sometimes
    import { resolve } from '$app/paths';
    import { Shield, Target, Search, Settings, Network, Activity } from 'lucide-svelte';

    const menuItems = [
        { name: 'Dashboard', path: '/', icon: Shield },
        { name: 'Intelligence', path: '/intelligence/domain-info', icon: Search },
        { name: 'Reconnaissance', path: '/recon', icon: Target },
        { name: 'Security Assessment', path: '/assessment/security-posture', icon: Network },
        { name: 'Engine Health', path: '/settings/dependencies', icon: Activity },
        { name: 'Settings', path: '/settings', icon: Settings },
    ];
</script>

<aside class="w-64 h-full bg-background border-r border-base p-4 flex flex-col transition-all {appState.sidebarOpen ? 'translate-x-0' : '-translate-x-full'}">
    <div class="flex items-center gap-3 mb-8 px-2">
        <div class="h-8 w-8 rounded-md bg-cyan-400 flex items-center justify-center shadow-[0_0_15px_rgba(34,211,238,0.5)]">
            <Shield class="text-on-accent size-5" />
        </div>
        <h2 class="text-xl font-bold tracking-widest uppercase text-primary-text shadow-cyan-400 drop-shadow-md">WebQ</h2>
    </div>

    <nav class="flex-1 space-y-2">
        {#each menuItems as item (item.name)}
            <a href={resolve(item.path)} class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-muted hover:text-primary-text hover:bg-surface transition-all">
                <item.icon class="size-4 text-accent/80" />
                {item.name}
            </a>
        {/each}
    </nav>
</aside>
