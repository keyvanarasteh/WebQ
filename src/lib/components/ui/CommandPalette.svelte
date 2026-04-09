<script lang="ts">
    import { Search, Shield, Target, Network, Settings, X, Terminal } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';
    import { goto } from '$app/navigation';
    import { resolve } from '$app/paths';

    let isOpen = $state(false);
    let searchQuery = $state('');

    const commands = [
        { id: 'dashboard', label: 'Go to Dashboard', path: '/', icon: Shield, category: 'Navigation' },
        { id: 'intel', label: 'Domain Intelligence', path: '/intelligence/domain-info', icon: Search, category: 'Navigation' },
        { id: 'recon', label: 'Reconnaissance', path: '/recon/subdomain-discovery', icon: Target, category: 'Navigation' },
        { id: 'assess', label: 'Security Assessment', path: '/assessment/security-posture', icon: Network, category: 'Navigation' },
        { id: 'settings', label: 'Settings', path: '/settings', icon: Settings, category: 'System' },
    ];

    let filteredCommands = $derived(commands.filter(c => 
        c.label.toLowerCase().includes(searchQuery.toLowerCase()) || 
        c.category.toLowerCase().includes(searchQuery.toLowerCase())
    ));

    $effect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
                e.preventDefault();
                isOpen = !isOpen;
                if (isOpen) searchQuery = '';
            }
            if (e.key === 'Escape' && isOpen) {
                isOpen = false;
            }
        };
        window.addEventListener('keydown', handleKeyDown);
        return () => window.removeEventListener('keydown', handleKeyDown);
    });

    function executeCommand(path: string) {
        goto(resolve(path));
        isOpen = false;
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div class="fixed inset-0 z-50 flex items-start justify-center pt-32 bg-black/60 backdrop-blur-sm" transition:fade={{ duration: 150 }} onclick={() => isOpen = false}>
        <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions, a11y_no_noninteractive_element_interactions -->
        <div class="w-full max-w-2xl bg-background border border-cyan-500/30 rounded-xl shadow-[0_0_50px_-12px_rgba(34,211,238,0.2)] overflow-hidden flex flex-col" transition:fly={{ y: -20, duration: 250, easing: backOut }} onclick={(e) => e.stopPropagation()}>
            <div class="flex items-center px-4 py-3 border-b border-base gap-3">
                <Terminal class="text-accent size-5" />
                <!-- svelte-ignore a11y_autofocus -->
                <input 
                    type="text" 
                    bind:value={searchQuery}
                    placeholder="Type a command or search..."
                    class="w-full bg-transparent border-none text-primary-text focus:outline-none focus:ring-0 text-sm font-mono"
                    autofocus
                />
                <button onclick={() => isOpen = false} class="text-muted hover:text-primary-text transition-colors bg-white/5 hover:bg-white/10 rounded-md p-1 border border-transparent hover:border-white/10">
                    <X size={16} />
                </button>
            </div>
            
            <div class="max-h-96 overflow-y-auto p-2 scrollbar-thin">
                {#if filteredCommands.length === 0}
                    <div class="px-4 py-8 text-center text-muted font-mono text-sm">No commands found matching "{searchQuery}"</div>
                {:else}
                    {#each filteredCommands as cmd (cmd.id)}
                        <button 
                            onclick={() => executeCommand(cmd.path)}
                            class="w-full flex items-center gap-3 px-4 py-3 rounded-lg text-left text-primary-text hover:bg-cyan-500/10 hover:text-accent transition-colors border border-transparent hover:border-cyan-500/30 group"
                        >
                            <cmd.icon class="size-4 opacity-50 group-hover:opacity-100 transition-opacity" />
                            <div class="flex flex-col">
                                <span class="text-sm font-medium">{cmd.label}</span>
                                <span class="text-[10px] text-muted font-mono uppercase tracking-widest">{cmd.category}</span>
                            </div>
                        </button>
                    {/each}
                {/if}
            </div>
            
            <div class="bg-black/40 px-4 py-2 border-t border-base text-xs text-muted flex justify-between items-center font-mono">
                <span><kbd class="px-1.5 py-0.5 rounded bg-white/5 border border-white/10 mr-1">↑↓</kbd> to navigate</span>
                <span><kbd class="px-1.5 py-0.5 rounded bg-white/5 border border-white/10 mr-1">Enter</kbd> to select</span>
                <span><kbd class="px-1.5 py-0.5 rounded bg-white/5 border border-white/10 mr-1">Esc</kbd> to close</span>
            </div>
        </div>
    </div>
{/if}
