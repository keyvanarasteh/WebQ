<script lang="ts">
	import { Share2, Link2, ExternalLink } from 'lucide-svelte';

	interface SocialProfile {
		platform: string;
		username: string;
		url: string;
		found_on?: string;
	}

	let { profiles }: { profiles: Record<string, SocialProfile[]> } = $props();

	function getPlatformColor(platform: string) {
		const p = platform.toLowerCase();
		if (p.includes('linkedin')) return 'text-blue-500 border-blue-500/30 bg-blue-500/10 shadow-blue-500/20';
		if (p.includes('twitter') || p.includes('x')) return 'text-sky-400 border-sky-400/30 bg-sky-400/10 shadow-sky-400/20';
		if (p.includes('github')) return 'text-indigo-200 border-slate-500/30 bg-slate-500/10 shadow-slate-500/20';
		if (p.includes('instagram')) return 'text-pink-500 border-pink-500/30 bg-pink-500/10 shadow-pink-500/20';
		if (p.includes('facebook')) return 'text-blue-600 border-blue-600/30 bg-blue-600/10 shadow-blue-600/20';
		if (p.includes('youtube')) return 'text-red-500 border-red-500/30 bg-red-500/10 shadow-red-500/20';
		return 'text-indigo-400 border-indigo-400/30 bg-indigo-400/10 shadow-indigo-400/20';
	}
</script>

<div class="space-y-6">
	{#each Object.entries(profiles) as [platform, profileList] (platform)}
		{@const theme = getPlatformColor(platform)}
		<div>
			<h4 class="text-xs font-black text-slate-400 tracking-widest uppercase mb-3 px-1 border-b border-[#27272a] inline-block pb-1">
				{platform} <span class="bg-[#27272a] text-xs px-2 py-0.5 rounded-full ml-2 text-white">{profileList.length}</span>
			</h4>
			<div class="grid gap-3 select-none">
				{#each profileList as profile, i (i + profile.url)}
					<a 
						href={profile.url} 
						target="_blank" 
						rel="noopener noreferrer"
						class="flex flex-col bg-[#09090b] p-4 rounded-lg border focus:outline-none hover:scale-[1.02] transition-transform duration-200 cursor-pointer shadow-xl {theme}"
					>
						<div class="flex justify-between items-start gap-4">
							<div class="flex flex-col overflow-hidden">
								<span class="text-sm font-bold truncate tracking-wide">{profile.username}</span>
								{#if profile.found_on}
									<span class="text-[10px] text-slate-500 truncate mt-1.5 break-all flex items-center gap-1.5 group-hover:text-slate-400 transition-colors">
										<Link2 size={12} class="shrink-0" /> {profile.found_on}
									</span>
								{/if}
							</div>
							<ExternalLink size={16} class="opacity-50 mt-0.5" />
						</div>
					</a>
				{/each}
			</div>
		</div>
	{/each}
</div>
