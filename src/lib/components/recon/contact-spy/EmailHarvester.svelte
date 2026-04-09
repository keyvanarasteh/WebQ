<script lang="ts">
	import { ShieldAlert, User } from 'lucide-svelte';

	let { emails }: { emails: string[] } = $props();

	const ROLE_PREFIXES = ['admin', 'info', 'support', 'sales', 'contact', 'webmaster', 'billing', 'hr', 'abuse', 'postmaster', 'security', 'hello', 'jobs', 'careers', 'press', 'media', 'help', 'dev'];

	function isRoleBased(email: string) {
		const prefix = email.split('@')[0]?.toLowerCase() || '';
		return ROLE_PREFIXES.includes(prefix);
	}

	let categorizedEmails = $derived.by(() => {
		const result = emails.map(email => ({
			email,
			roleBased: isRoleBased(email)
		}));
		// Sort: role based first, then alphabetically
		return result.sort((a,b) => {
			if (a.roleBased && !b.roleBased) return -1;
			if (!a.roleBased && b.roleBased) return 1;
			return a.email.localeCompare(b.email);
		});
	});
</script>

<ul class="space-y-3">
	{#each categorizedEmails as item, i (item.email)}
		<li class="flex flex-col gap-2 bg-background p-3 rounded-lg border border-base hover:border-slate-600 transition-colors">
			<div class="flex justify-between items-center w-full">
				<div class="flex items-center gap-3 overflow-hidden">
					<span class="text-xs text-muted font-mono shrink-0">{String(i + 1).padStart(2, '0')}</span>
					<span class="text-slate-300 text-sm font-fira truncate tracking-tight">{item.email}</span>
				</div>
				<div class="shrink-0 ml-2 border-l border-base pl-3">
					{#if item.roleBased}
						<span class="inline-flex items-center gap-1.5 bg-rose-500/10 text-rose-400 text-[10px] font-bold px-2 py-1 rounded tracking-widest uppercase items-center shadow-lg shadow-rose-500/10 border border-rose-500/20">
							<ShieldAlert size={12} />
							Role
						</span>
					{:else}
						<span class="inline-flex items-center gap-1.5 bg-blue-500/10 text-blue-400 text-[10px] font-bold px-2 py-1 rounded tracking-widest uppercase items-center shadow-lg shadow-blue-500/10 border border-blue-500/20">
							<User size={12} />
							Personal
						</span>
					{/if}
				</div>
			</div>
		</li>
	{/each}
</ul>
