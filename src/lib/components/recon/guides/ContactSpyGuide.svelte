<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Users, Briefcase, Mail, Phone, Search, ShieldCheck, AlertTriangle, ArrowRight, ChevronRight, ChevronLeft, ShieldAlert } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. OSINT Harvesting", icon: Search },
        { id: 1, label: "2. Exploitation", icon: AlertTriangle },
        { id: 2, label: "3. Defense", icon: ShieldCheck }
    ];

    function nextTab() {
        if (activeTab < tabs.length - 1) activeTab++;
    }
    function prevTab() {
        if (activeTab > 0) activeTab--;
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
    <div 
        class="fixed inset-0 z-[60] flex items-center justify-center p-4 sm:p-6 bg-black/80 backdrop-blur-md"
        transition:fade={{ duration: 200 }}
        onclick={close}
    >
        <div 
            class="bg-[#0A0C10] border border-indigo-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(99,102,241,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-indigo-950/40 via-purple-900/10 to-transparent p-6 border-b border-indigo-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-indigo-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-zinc-400 hover:text-white bg-white/5 hover:bg-indigo-500/20 rounded-xl transition-all border border-white/5 hover:border-indigo-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-indigo-500/20 to-purple-600/10 flex items-center justify-center border border-indigo-500/30 shadow-inner">
                            <Users size={28} class="text-indigo-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-white">{m.recon_contact_guide_title ? m.recon_contact_guide_title() : 'Contact Intelligence (OSINT)'}</h2>
                            <p class="text-sm text-zinc-400 mt-1 max-w-md">Harvesting employee emails, structures, and social footprints.</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-indigo-500/20 text-indigo-300 border border-indigo-500/20 shadow-sm' : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5 border border-transparent'}"
                            >
                                <tab.icon size={14} />
                                <span class="hidden sm:inline">{tab.label}</span>
                            </button>
                        {/each}
                    </div>
                </div>
            </div>

            <!-- Content Area - Scrollable -->
            <div class="flex-1 overflow-y-auto p-6 relative">
                {#if activeTab === 0}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-indigo-300 mb-4">Open-Source Intelligence (OSINT)</h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <p class="text-zinc-300 text-sm leading-relaxed">
                                    Before attacking a network, adversaries attack the personnel. Using tools like Hunter.io, LinkedIn scrapers, and search engine dorks, attackers map out your entire corporate structure.
                                </p>
                                <div class="bg-indigo-950/20 border border-indigo-500/10 rounded-xl p-4">
                                    <h4 class="text-indigo-400 font-medium mb-3 flex items-center gap-2"><Search size={16}/> Harvesting Methods</h4>
                                    <ul class="space-y-3 text-sm text-zinc-400">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-indigo-500 mt-1 shrink-0"/> <span><strong>Email Permutation:</strong> guessing `first.last@company.com` based on common corporate structures.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-indigo-500 mt-1 shrink-0"/> <span><strong>Public Repositories:</strong> Extracting developer emails left in `.git` commit histories on GitHub/GitLab.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-indigo-500 mt-1 shrink-0"/> <span><strong>PDF Metadata:</strong> Downloading public case studies and extracting the "Author" fields using EXIF tools.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Visual Network UI -->
                            <div class="bg-black/50 border border-white/5 rounded-xl p-6 relative overflow-hidden shadow-inner h-full flex flex-col justify-center">
                                <div class="absolute inset-0 bg-[linear-gradient(to_right,#4f46e510_1px,transparent_1px),linear-gradient(to_bottom,#4f46e510_1px,transparent_1px)] bg-[size:24px_24px]"></div>
                                
                                <div class="relative z-10 space-y-4">
                                    <div class="flex items-center gap-3 bg-zinc-900/80 p-3 rounded-lg border border-white/10 backdrop-blur">
                                        <div class="bg-blue-500/20 p-2 rounded text-blue-400"><Briefcase size={16}/></div>
                                        <div>
                                            <p class="text-sm font-bold text-zinc-200">Sarah Jenkins</p>
                                            <p class="text-xs text-zinc-500">VP of Finance (LinkedIn)</p>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-3 bg-zinc-900/80 p-3 rounded-lg border border-indigo-500/30 shadow-[0_0_15px_rgba(79,70,229,0.15)] backdrop-blur ml-6">
                                        <div class="bg-indigo-500/20 p-2 rounded text-indigo-400"><Mail size={16}/></div>
                                        <div>
                                            <p class="text-sm font-bold text-indigo-300">s.jenkins@target.com</p>
                                            <p class="text-xs text-zinc-500">Pattern match: (FirstInitial.LastName)</p>
                                        </div>
                                    </div>
                                    <div class="flex items-center gap-3 bg-zinc-900/80 p-3 rounded-lg border border-rose-500/30 shadow-[0_0_15px_rgba(244,63,94,0.15)] backdrop-blur ml-12">
                                        <div class="bg-rose-500/20 p-2 rounded text-rose-400"><Phone size={16}/></div>
                                        <div>
                                            <p class="text-sm font-bold text-rose-300">+1 (555) 019-8392</p>
                                            <p class="text-xs text-zinc-500">Found in leaked resume PDF</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-indigo-300">Exploitation Vectors</h3>
                        </div>
                        <p class="text-sm text-zinc-400 mb-6 max-w-3xl">Once the human infrastructure is mapped, attackers weaponize the data. Humans are historically the weakest link in any perimeter.</p>

                        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 mb-8">
                            <div class="bg-indigo-950/20 border border-indigo-500/20 rounded-xl p-5 hover:bg-indigo-900/20 transition-colors">
                                <Mail class="text-indigo-400 mb-3" size={24} />
                                <h4 class="text-sm font-bold text-indigo-300 mb-2">Spear Phishing</h4>
                                <p class="text-xs text-zinc-400 leading-relaxed">Targeting executives or IT admins with highly tailored emails referencing internal projects (found via social media) to inject malware or steal MFA tokens.</p>
                            </div>
                            <div class="bg-purple-950/20 border border-purple-500/20 rounded-xl p-5 hover:bg-purple-900/20 transition-colors">
                                <Users class="text-purple-400 mb-3" size={24} />
                                <h4 class="text-sm font-bold text-purple-300 mb-2">Password Spraying</h4>
                                <p class="text-xs text-zinc-400 leading-relaxed">Using the harvested email list, attackers try common passwords (e.g., `Company2024!`) across all accounts slowly to avoid account lockout policies.</p>
                            </div>
                            <div class="bg-rose-950/20 border border-rose-500/20 rounded-xl p-5 hover:bg-rose-900/20 transition-colors">
                                <Phone class="text-rose-400 mb-3" size={24} />
                                <h4 class="text-sm font-bold text-rose-300 mb-2">Vishing (Voice Phishing)</h4>
                                <p class="text-xs text-zinc-400 leading-relaxed">Calling the IT Helpdesk pretending to be a specific exposed employee who "lost their phone" to get an MFA device reset or password change.</p>
                            </div>
                        </div>

                        <!-- Code Example -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-indigo-500/30 overflow-hidden font-mono text-sm shadow-[0_0_20px_rgba(99,102,241,0.1)]">
                            <div class="absolute top-0 right-0 bg-indigo-500/10 text-indigo-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-indigo-500/30">Password Spray Automation</div>
                            <div class="p-4 overflow-x-auto pt-10 text-zinc-400 custom-scrollbar">
                                <p class="text-emerald-400 flex items-center gap-2 font-semibold text-xs"><span>➜</span> <span class="text-blue-400">hydra</span> -L emails.txt -p 'Winter2024!' owa.target.com https-post-form</p>
                                <p class="text-zinc-500 text-[10px] mt-2">[DATA] 150 emails loaded. Starting attack...</p>
                                <p class="text-emerald-400 font-bold bg-emerald-500/10 px-2 py-1 rounded inline-block mt-2 tracking-widest text-xs">[SUCCESS] login: s.jenkins@target.com pass: Winter2024!</p>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Hardening Human Infrastructure</h3>
                        <p class="text-sm text-zinc-400 mb-6 max-w-3xl">You cannot hide every employee, but you can severely limit the blast radius of OSINT harvesting and social engineering.</p>

                        <!-- Mitigation Checklist -->
                        <div class="space-y-4">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit">
                                    <ShieldAlert size={20} class="text-emerald-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">MFA & FIDO2 Enforcement</h4>
                                    <p class="text-xs text-emerald-200/70 leading-relaxed">Even if a password is sprayed or phished, hardware security keys (YubiKey/FIDO2) physically prevent the attacker from authenticating, neutralizing 99% of phishing attacks.</p>
                                </div>
                            </div>

                            <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                                <div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl">
                                    <h4 class="text-sm font-bold text-gray-200 mb-2 flex items-center gap-2">
                                        <Mail size={16} class="text-blue-400"/> DMARC & SPF
                                    </h4>
                                    <p class="text-xs text-gray-400 leading-relaxed">Configure strict DMARC policies (`p=reject`). This prevents attackers from spoofing your domain when emailing your own employees (internal spear-phishing).</p>
                                </div>
                                <div class="bg-[#121214] border border-[#27272a] p-5 rounded-xl">
                                    <h4 class="text-sm font-bold text-gray-200 mb-2 flex items-center gap-2">
                                        <Users size={16} class="text-orange-400"/> Security Awareness
                                    </h4>
                                    <p class="text-xs text-gray-400 leading-relaxed">Train high-value targets (Executives, Finance) to recognize urgency-based pretexting. Remove public PDF directories that leak metadata.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-indigo-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
                <div class="text-xs text-zinc-500 hidden sm:block">
                    Use <kbd class="px-2 py-1 bg-white/5 rounded mx-1 text-zinc-400 border border-white/10 font-mono">Esc</kbd> or click outside to dismiss
                </div>
                <div class="flex items-center gap-3 w-full sm:w-auto justify-between sm:justify-end">
                    {#if activeTab > 0}
                        <button 
                            onclick={prevTab}
                            class="flex items-center gap-1.5 px-5 py-2.5 bg-white/5 hover:bg-white/10 text-zinc-300 text-sm font-medium rounded-xl border border-white/5 hover:border-white/10 transition-all focus:ring-2 focus:ring-white/10 outline-none"
                        >
                            <ChevronLeft size={16} /> Previous
                        </button>
                    {:else}
                        <div></div>
                    {/if}
                    
                    {#if activeTab < tabs.length - 1}
                        <button 
                            onclick={nextTab}
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(99,102,241,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-indigo-500/50 outline-none"
                        >
                            Continue <ChevronRight size={16} />
                        </button>
                    {:else}
                        <button 
                            onclick={close}
                            class="flex items-center gap-1.5 px-8 py-2.5 bg-emerald-500 hover:bg-emerald-400 text-emerald-950 text-sm font-bold rounded-xl shadow-[0_0_20px_rgba(16,185,129,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-emerald-500/50 outline-none"
                        >
                            Complete
                        </button>
                    {/if}
                </div>
            </div>
        </div>
    </div>
{/if}

<style>
    .custom-scrollbar::-webkit-scrollbar {
        width: 8px;
        height: 8px;
    }
    .custom-scrollbar::-webkit-scrollbar-track {
        background: transparent;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb {
        background-color: rgba(255, 255, 255, 0.1);
        border-radius: 10px;
    }
    .custom-scrollbar::-webkit-scrollbar-thumb:hover {
        background-color: rgba(255, 255, 255, 0.2);
    }
</style>
