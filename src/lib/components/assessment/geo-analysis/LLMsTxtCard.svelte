<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { FileText, CheckCircle2, ShieldAlert, Cpu } from 'lucide-svelte';

    let { llmsTxt }: {
        llmsTxt: {
            found: boolean,
            files: string[]
        }
    } = $props();
</script>

<div class="border border-subtle bg-glass backdrop-blur-md rounded-xl p-6 relative overflow-hidden group">
    <div class="absolute -right-6 -bottom-6 opacity-[0.03] group-hover:opacity-[0.06] transition-opacity pointer-events-none">
        <Cpu size={140} />
    </div>

    <div class="flex items-center justify-between mb-5">
        <div class="flex items-center gap-3">
            <div class={llmsTxt.found ? "text-cyan-400" : "text-muted"}>
                <FileText class="w-5 h-5" />
            </div>
            <h3 class="text-lg font-medium text-primary-text tracking-wide">LLMs.txt Standards</h3>
        </div>
        {#if llmsTxt.found}
            <div class="px-2.5 py-1 text-[10px] uppercase font-bold tracking-widest rounded-full bg-cyan-500/10 border border-cyan-500/20 text-cyan-400">
                AI Ready
            </div>
        {:else}
            <div class="px-2.5 py-1 text-[10px] uppercase font-bold tracking-widest rounded-full bg-surface border border-subtle text-muted">
                Not Found
            </div>
        {/if}
    </div>

    {#if llmsTxt.found}
        <div class="space-y-3">
            <p class="text-xs text-muted mb-4 border-b border-subtle pb-3">
                This domain structurally provides documentation explicitly tailored for LLMs across the following paths:
            </p>
            {#each llmsTxt.files as file}
                <div class="flex items-center gap-3 p-3 bg-cyan-500/5 border border-cyan-500/10 rounded-lg hover:border-cyan-500/30 transition-colors">
                    <CheckCircle2 size={16} class="text-cyan-400 shrink-0" />
                    <span class="text-sm font-mono text-cyan-300 truncate">{file}</span>
                </div>
            {/each}
        </div>
    {:else}
        <div class="p-4 bg-glass border border-subtle rounded-lg flex items-start gap-3 mt-4">
            <ShieldAlert class="w-5 h-5 text-yellow-400 shrink-0 mt-0.5" />
            <div class="flex flex-col gap-1">
                <span class="text-sm font-medium text-yellow-300">No Machine-Readable Context</span>
                <p class="text-xs text-muted leading-relaxed">
                    Servers lacking `llms.txt` or `llms-full.txt` may suffer SEO penalties and hallucinated context from AI agents scraping their raw HTML rather than structured markdown.
                </p>
            </div>
        </div>
    {/if}
</div>
