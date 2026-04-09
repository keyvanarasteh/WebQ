<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
    // @ts-expect-error SvelteKit generated module not playing nice with TS ambient types sometimes
    import { resolve } from '$app/paths';
    import { Shield, Target, Search, Settings, Network } from 'lucide-svelte';

    const menuItems = [
        { name: 'Dashboard', path: '/', icon: Shield },
        { name: 'Intelligence', path: '/intelligence/domain-info', icon: Search },
        { name: 'Reconnaissance', path: '/recon', icon: Target },
        { name: 'Security Assessment', path: '/assessment/security-posture', icon: Network },
        { name: 'Settings', path: '/settings', icon: Settings },
    ];
</script>

<aside class="w-64 h-full bg-qix-obsidian border-r border-[#27272a] p-4 flex flex-col transition-all {appState.sidebarOpen ? 'translate-x-0' : '-translate-x-full'}">
    <div class="flex items-center gap-3 mb-8 px-2">
        <div class="h-8 w-8 rounded-md bg-cyan-400 flex items-center justify-center shadow-[0_0_15px_rgba(34,211,238,0.5)]">
            <Shield class="text-black size-5" />
        </div>
        <h2 class="text-xl font-bold tracking-widest uppercase text-white shadow-cyan-400 drop-shadow-md">WebQ</h2>
    </div>

    <nav class="flex-1 space-y-2">
        {#each menuItems as item (item.name)}
            <a href={resolve(item.path)} class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm text-gray-400 hover:text-white hover:bg-[#27272a] transition-all">
                <item.icon class="size-4 text-cyan-500/80" />
                {item.name}
            </a>
        {/each}
    </nav>
</aside>
