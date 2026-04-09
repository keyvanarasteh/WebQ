<script lang="ts">
    import type { Snippet } from 'svelte';

    let { 
        children, 
        class: className = "" 
    }: { 
        children: Snippet, 
        class?: string 
    } = $props();
    
    let mouseX = $state(0);
    let mouseY = $state(0);
    let isHovered = $state(false);

    function onMouseMove(e: MouseEvent) {
        const rect = (e.currentTarget as HTMLElement).getBoundingClientRect();
        mouseX = e.clientX - rect.left;
        mouseY = e.clientY - rect.top;
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div 
    class="relative overflow-hidden rounded-xl bg-qix-obsidian/30 border border-[#27272a] transition-all duration-300 hover:border-cyan-500/20 group {className}"
    onmousemove={onMouseMove}
    onmouseenter={() => isHovered = true}
    onmouseleave={() => isHovered = false}
>
    <!-- Background overlay spotlight -->
    <div 
        class="pointer-events-none absolute -inset-px rounded-xl opacity-0 transition-opacity duration-300 group-hover:opacity-100"
        style:background="radial-gradient(600px circle at {mouseX}px {mouseY}px, rgba(34,211,238,0.08), transparent 40%)"
    ></div>
    
    <!-- Border spotlight -->
    <div 
        class="pointer-events-none absolute inset-0 rounded-xl opacity-0 transition-opacity duration-300 group-hover:opacity-100"
        style:background="radial-gradient(400px circle at {mouseX}px {mouseY}px, rgba(34,211,238,0.4), transparent 40%)"
        style:mask="linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0)"
        style:-webkit-mask="linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0)"
        style:mask-composite="exclude"
        style:-webkit-mask-composite="xor"
        style:padding="1px"
    ></div>

    <div class="relative z-10 w-full h-full">
        {@render children()}
    </div>
</div>
