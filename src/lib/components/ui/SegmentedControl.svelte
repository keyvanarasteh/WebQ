<script lang="ts">
  let { options = [], value = $bindable(), name = 'segmented' } = $props<{
      options: { label: string; value: string }[],
      value: string,
      name?: string
  }>();
</script>

<div class="flex p-1 space-x-1 bg-background border border-base rounded-xl max-w-fit shadow-inner">
  {#each options as option (option.value)}
    <label class="relative flex-1 cursor-pointer">
      <input type="radio" {name} value={option.value} bind:group={value} class="sr-only peer" />
      <div class="px-6 py-2 text-sm font-semibold text-center rounded-lg text-muted transition-all duration-200 peer-checked:bg-cyan-500/10 peer-checked:text-cyan-400 peer-checked:shadow-[0_0_15px_rgba(34,211,238,0.15)] peer-checked:ring-1 peer-checked:ring-cyan-500/30 hover:text-primary-text hover:bg-surface/50 peer-checked:hover:bg-cyan-500/10">
        {option.label}
      </div>
    </label>
  {/each}
</div>
