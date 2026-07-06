<script lang="ts">
    import { ChevronDown, ChevronUp, Code, FileCode, ShieldAlert, FileSearch, Terminal } from "lucide-svelte";
    
    import { reactSignatureGroups } from "$lib/data/reactSignatures";

    let expandedSections = $state<Record<string, boolean>>({});

    function toggleSection(id: string) {
        expandedSections[id] = !expandedSections[id];
    }
</script>

<div class="space-y-6 max-w-7xl mx-auto p-6">
    <div class="text-center space-y-4 mb-10">
        <h1 class="text-5xl font-black tracking-tight text-primary-text">Signature <span class="text-blue-500 drop-shadow-[0_0_15px_rgba(59,130,246,0.5)]">Matrix</span></h1>
        <p class="text-muted text-lg">Comprehensive documentation of React2Shell Honeypot detection vectors.</p>
    </div>

    {#each reactSignatureGroups as group}
        <div class="mb-8">
            <h2 class="text-2xl font-bold mb-4 flex items-center gap-2 text-{group.color}-500">
                <ShieldAlert class="w-6 h-6" /> {group.severity} Severity
            </h2>
            <div class="space-y-4">
                {#each group.items as item, index}
                    {@const accordionId = `${group.severity}-${index}`}
                    <div class="bg-surface/30 border border-border/50 rounded-xl overflow-hidden transition-all duration-300 shadow-lg {expandedSections[accordionId] ? 'border-' + group.color + '-500/50 shadow-[0_0_15px_rgba(var(--color-' + group.color + '-500),0.1)]' : ''}">
                        <button 
                            class="w-full p-4 flex items-center justify-between bg-surface/50 hover:bg-surface transition-colors text-left focus:outline-none"
                            onclick={() => toggleSection(accordionId)}
                        >
                            <div class="flex items-center gap-4">
                                <div class="p-2 bg-{group.color}-500/10 rounded-lg text-{group.color}-500">
                                    <FileSearch class="w-5 h-5" />
                                </div>
                                <div>
                                    <h3 class="font-bold text-primary-text text-lg">{item.category}</h3>
                                    <p class="text-xs text-muted font-mono">{item.ids}</p>
                                </div>
                            </div>
                            <div class="text-muted">
                                {#if expandedSections[accordionId]}
                                    <ChevronUp class="w-5 h-5" />
                                {:else}
                                    <ChevronDown class="w-5 h-5" />
                                {/if}
                            </div>
                        </button>
                        
                        {#if expandedSections[accordionId]}
                            <div class="p-6 border-t border-border/50 space-y-4 bg-background/50">
                                <div>
                                    <h4 class="text-sm font-bold text-primary-text mb-1 flex items-center gap-2">
                                        <ShieldAlert class="w-4 h-4 text-muted" /> Description / Access Granted
                                    </h4>
                                    <p class="text-sm text-secondary-text">{item.description}</p>
                                </div>
                                
                                <div class="bg-surface/80 rounded-lg p-4 border border-border/50">
                                    <h4 class="text-sm font-bold text-primary-text mb-2 flex items-center gap-2">
                                        <Terminal class="w-4 h-4 text-green-400" /> cURL Trigger Example
                                    </h4>
                                    <code class="block font-mono text-xs text-green-400 break-all">{item.curl}</code>
                                </div>
                                
                                <div class="bg-surface/80 rounded-lg p-4 border border-border/50">
                                    <h4 class="text-sm font-bold text-primary-text mb-2 flex items-center gap-2">
                                        <FileCode class="w-4 h-4 text-blue-400" /> Fix / Remediation
                                    </h4>
                                    <p class="text-sm text-blue-300 font-mono">{item.remediation}</p>
                                </div>
                            </div>
                        {/if}
                    </div>
                {/each}
            </div>
        </div>
    {/each}
</div>
