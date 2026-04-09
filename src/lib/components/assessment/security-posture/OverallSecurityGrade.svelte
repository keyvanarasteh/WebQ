<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { Shield, ShieldAlert, ShieldCheck } from 'lucide-svelte';

    let { score, grade, riskLevel, recommendations }: {
        score: number,
        grade: string,
        riskLevel: string,
        recommendations: string[]
    } = $props();

    let isHighRisk = $derived(score < 50);

    let gradeColor = $derived(
        grade.startsWith('A') ? 'text-emerald-400' :
        grade.startsWith('B') ? 'text-blue-400' :
        grade.startsWith('C') ? 'text-yellow-400' :
        'text-red-400'
    );
</script>

<div class="relative overflow-hidden border border-subtle bg-glass backdrop-blur-md rounded-xl p-6 sm:p-8 {isHighRisk ? 'ring-1 ring-red-500/30' : ''}">
    <!-- Background Glare -->
    <div class="absolute -top-32 -right-32 w-64 h-64 rounded-full blur-[100px] pointer-events-none {isHighRisk ? 'bg-red-500/10' : 'bg-emerald-500/10'}"></div>

    <div class="flex items-center justify-between mb-8">
        <h3 class="text-xl font-medium tracking-tight text-primary-text flex items-center gap-2">
            {#if isHighRisk}
                <ShieldAlert class="w-6 h-6 text-red-400" />
            {:else}
                <ShieldCheck class="w-6 h-6 text-emerald-400" />
            {/if}
            {m.sec_posture_grade()}
        </h3>
        <span class="px-3 py-1 text-xs font-semibold uppercase tracking-wider rounded-full {isHighRisk ? 'bg-red-500/20 text-red-300' : 'bg-emerald-500/20 text-emerald-300'}">
            {m.sec_posture_risk({ risk: riskLevel })}
        </span>
    </div>

    <div class="flex flex-col items-center justify-center py-6 mb-8 border-b border-glass">
        <div class="text-7xl font-bold tracking-tight mb-2 flex items-baseline gap-2 {gradeColor}">
            {grade}
            <span class="text-2xl font-light text-muted">/ {score}</span>
        </div>
        <p class="text-sm font-medium text-muted uppercase tracking-widest">{m.sec_posture_score()}</p>
    </div>

    <div>
        <h4 class="text-sm font-medium text-secondary-text mb-4">{m.sec_posture_recommendations()}</h4>
        <div class="space-y-3">
            {#each recommendations as rec}
                <div class="flex items-start gap-3 p-3 rounded-lg bg-glass border border-subtle">
                    <div class="w-1.5 h-1.5 rounded-full bg-indigo-400 mt-2 shrink-0"></div>
                    <p class="text-sm text-secondary-text leading-relaxed">{rec}</p>
                </div>
            {/each}
        </div>
    </div>
</div>
