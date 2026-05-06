<script lang="ts">
    import { appState } from '$lib/stores/AppState.svelte';
    // @ts-expect-error SvelteKit generated module not playing nice with TS ambient types sometimes
    import { resolve } from '$app/paths';
    // @ts-expect-error Same for stores
    import { page } from '$app/stores';
    import { Shield, Target, Search, Settings, Network, Info, Database, ShieldAlert } from 'lucide-svelte';

    const menuItems = [
        { name: 'Dashboard', path: '/', icon: Shield },
        { name: 'Intelligence', path: '/intelligence/domain-info', icon: Search },
        { name: 'Reconnaissance', path: '/recon', icon: Target },
        { name: 'Security Assessment', path: '/assessment/security-posture', icon: Network },
        { name: 'React2Shell', path: '/react', icon: ShieldAlert },
        { name: 'React Honeypot', path: '/react/honeypot', icon: Shield },
        { name: 'Wiki', path: '/roadmap', icon: Database },
        { name: 'Settings', path: '/settings', icon: Settings },
        { name: 'About & Feedback', path: '/about', icon: Info },
    ];
</script>

<aside class="h-full bg-background transition-all duration-300 overflow-hidden {appState.sidebarOpen ? 'w-64 border-r border-base' : 'w-0 border-none'}">
    <div class="w-64 h-full p-4 flex flex-col">
        <div class="flex items-center gap-3 mb-8 px-2">
            <div class="h-8 w-8 rounded-md bg-cyan-400 flex items-center justify-center shadow-[0_0_15px_rgba(34,211,238,0.5)]">
                <Shield class="text-on-accent size-5" />
            </div>
            <h2 class="text-xl font-bold tracking-widest uppercase text-primary-text shadow-cyan-400 drop-shadow-md">WebQ</h2>
        </div>

        <nav class="flex-1 space-y-2">
            {#each menuItems as item (item.name)}
                {@const active = $page.url.pathname === resolve(item.path) || ($page.url.pathname.startsWith(resolve(item.path) + '/') && item.path !== '/')}
                <a href={resolve(item.path)} class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm transition-all {active ? 'text-primary-text bg-surface shadow-[0_0_15px_rgba(34,211,238,0.15)] ring-1 ring-cyan-500/20' : 'text-muted hover:text-primary-text hover:bg-surface'}">
                    <item.icon class="size-4 {active ? 'text-cyan-400' : 'text-accent/80'}" />
                    {item.name}
                </a>
            {/each}
        </nav>
    </div>
</aside>
