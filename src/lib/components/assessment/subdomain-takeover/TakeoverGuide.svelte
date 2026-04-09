<script lang="ts">
    import * as m from '$lib/paraglide/messages';
    import { X, Search, Link, ShieldCheck, AlertTriangle, CloudCrash, Code2, Server, ArrowRight, ChevronRight, ChevronLeft, Globe } from 'lucide-svelte';
    import { fade, fly } from 'svelte/transition';
    import { backOut } from 'svelte/easing';

    let { isOpen = $bindable(false) } = $props<{ isOpen: boolean }>();

    function close() {
        isOpen = false;
    }

    let activeTab = $state(0);
    const tabs = [
        { id: 0, label: "1. Fundamentals", icon: Globe },
        { id: 1, label: "2. Exploit Chain", icon: AlertTriangle },
        { id: 2, label: "3. Remediation", icon: ShieldCheck }
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
            class="bg-[#0A0C10] border border-amber-500/20 rounded-2xl w-full max-w-4xl overflow-hidden shadow-[0_0_50px_-12px_rgba(245,158,11,0.15)] relative flex flex-col max-h-[90vh]"
            transition:fly={{ y: 20, duration: 400, easing: backOut }}
            onclick={(e) => e.stopPropagation()}
        >
            <!-- Header Area -->
            <div class="relative bg-gradient-to-r from-amber-950/40 via-orange-900/10 to-transparent p-6 border-b border-amber-500/10 shrink-0">
                <div class="absolute -right-20 -top-20 w-64 h-64 bg-amber-500/10 blur-[64px] rounded-full pointer-events-none"></div>
                <button 
                    onclick={close}
                    class="absolute top-4 right-4 p-2 text-zinc-400 hover:text-white bg-white/5 hover:bg-amber-500/20 rounded-xl transition-all border border-white/5 hover:border-amber-500/30 font-medium"
                    aria-label="Close guide modal"
                >
                    <X size={18} />
                </button>

                <div class="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4 relative z-10">
                    <div class="flex items-center gap-4">
                        <div class="w-14 h-14 rounded-2xl bg-gradient-to-br from-amber-500/20 to-orange-600/10 flex items-center justify-center border border-amber-500/30 shadow-inner">
                            <Link size={28} class="text-amber-400" />
                        </div>
                        <div>
                            <h2 class="text-2xl font-bold tracking-tight text-white">{m.sec_takeover_guide_title ? m.sec_takeover_guide_title() : 'Subdomain Takeover Anatomy'}</h2>
                            <p class="text-sm text-zinc-400 mt-1 max-w-md">{m.sec_takeover_guide_desc ? m.sec_takeover_guide_desc() : 'Learn how dangling DNS records lead to complete subdomain hijacking.'}</p>
                        </div>
                    </div>
                    
                    <!-- Tab Navigation -->
                    <div class="flex bg-black/40 p-1 rounded-lg border border-white/5 mt-4 sm:mt-0">
                        {#each tabs as tab (tab.id)}
                            <button
                                onclick={() => activeTab = tab.id}
                                class="flex items-center gap-2 px-3 py-1.5 rounded-md text-sm font-medium transition-all {activeTab === tab.id ? 'bg-amber-500/20 text-amber-300 border border-amber-500/20 shadow-sm' : 'text-zinc-500 hover:text-zinc-300 hover:bg-white/5 border border-transparent'}"
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
                        <h3 class="text-lg font-semibold text-amber-300 mb-4">The Danger of Dangling Records</h3>
                        
                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                            <div class="space-y-4">
                                <p class="text-zinc-300 text-sm leading-relaxed">
                                    Subdomain takeover occurs when a DNS record points to a service (like AWS S3, GitHub Pages, or Heroku) that has been deleted or expired, leaving the domain pointing to an unclaimed resource.
                                </p>
                                <div class="bg-amber-950/20 border border-amber-500/10 rounded-xl p-4">
                                    <h4 class="text-amber-400 font-medium mb-3 flex items-center gap-2"><Server size={16}/> Common Misconfigurations</h4>
                                    <ul class="space-y-3 text-sm text-zinc-400">
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/> <span><strong>Dangling CNAMEs:</strong> Example: <code>blog.company.com</code> CNAME points to <code>company-blog.github.io</code>, but the GitHub repo was deleted.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/> <span><strong>Abandoned Cloud Resources:</strong> S3 buckets, Elastic Beanstalk apps, or Azure websites that are torn down without removing their DNS entries.</span></li>
                                        <li class="flex items-start gap-2"><ArrowRight size={14} class="text-amber-500 mt-1 shrink-0"/> <span><strong>Stale Nameservers (NS):</strong> A subdomain delegates authority to Route53 zones that have been deleted, allowing attackers to create the zone in their own AWS account.</span></li>
                                    </ul>
                                </div>
                            </div>

                            <!-- Visual Diagram -->
                            <div class="bg-black/50 border border-white/5 rounded-xl p-6 flex flex-col items-center justify-center relative shadow-inner h-full">
                                <div class="flex items-center justify-between w-full mb-6 relative py-4">
                                    <div class="flex flex-col items-center z-10 w-24">
                                        <div class="w-12 h-12 rounded-full bg-zinc-800 border-2 border-zinc-600 flex items-center justify-center text-zinc-300 mb-2">User</div>
                                        <span class="text-xs text-zinc-400 text-center">visits<br>auth.target.com</span>
                                    </div>
                                    
                                    <div class="absolute top-[35%] left-16 w-[calc(100%-8rem)] h-0 border-t-2 border-dashed border-amber-500/50">
                                        <div class="w-1/3 h-[2px] bg-amber-500 shadow-[0_0_10px_#f59e0b] animate-[pulse_2s_ease-in-out_infinite]"></div>
                                    </div>

                                    <div class="flex flex-col items-center z-10 w-28">
                                        <div class="w-14 h-14 rounded-lg bg-orange-950 border-2 border-orange-500 flex items-center justify-center text-orange-300 mb-2 shadow-[0_0_15px_rgba(249,115,22,0.4)]">
                                            <Globe size={24} />
                                        </div>
                                        <span class="text-xs text-orange-400 font-mono text-center">auth.target.com<br>CNAME</span>
                                    </div>

                                    <div class="absolute top-[35%] right-16 w-[calc(100%-8rem)] h-0 border-t-2 border-dashed border-red-500/50"></div>

                                    <div class="flex flex-col items-center z-10 w-24">
                                        <div class="relative w-12 h-12 rounded-lg bg-red-950 border-2 border-red-500 flex items-center justify-center mb-2">
                                            <AlertTriangle size={18} class="text-red-400"/>
                                            <div class="absolute -right-2 -top-2 bg-red-600 text-white min-w-5 h-5 rounded flex items-center justify-center font-bold text-[10px] px-1">404</div>
                                        </div>
                                        <span class="text-xs text-red-400 text-center">target-auth.s3<br>Deleted Bucket</span>
                                    </div>
                                </div>
                                <div class="text-center text-xs text-amber-400 mt-4 bg-amber-500/10 px-4 py-2 rounded-full border border-amber-500/30 font-medium tracking-wide">DNS points to a non-existent 3rd party resource</div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 1}
                    <div transition:fade={{duration: 200}}>
                        <div class="flex items-center gap-3 mb-4">
                            <h3 class="text-lg font-semibold text-amber-300">Exploitation Process</h3>
                        </div>
                        <p class="text-sm text-zinc-400 mb-4 max-w-3xl">An attacker discovers the dangling record and claims the target resource on the third-party service provider, effectively gaining full control over content served at the victim's subdomain.</p>
                        
                        <!-- Terminal Mockup -->
                        <div class="bg-[#0d1117] rounded-xl border border-amber-500/10 shadow-2xl overflow-hidden font-mono text-xs sm:text-sm shadow-[0_0_30px_rgba(0,0,0,0.5)]">
                            <div class="bg-zinc-900/80 px-4 py-3 flex items-center gap-2 border-b border-white/5 backdrop-blur">
                                <div class="w-3 h-3 rounded-full bg-red-500/80"></div>
                                <div class="w-3 h-3 rounded-full bg-yellow-500/80"></div>
                                <div class="w-3 h-3 rounded-full bg-green-500/80"></div>
                                <span class="ml-2 text-zinc-500 text-xs tracking-wider">attacker@kali:~/recon</span>
                            </div>
                            <div class="p-5 space-y-3 text-zinc-300 h-72 overflow-y-auto custom-scrollbar">
                                <p class="text-emerald-400 flex items-center gap-2 font-semibold border-b border-white/5 pb-2"><span>➜</span> <span class="text-blue-400">dig</span> auth.target.com CNAME</p>
                                <div class="opacity-80 pl-4 py-2">
                                    <p class="text-zinc-500">; &lt;&lt;&gt;&gt; DiG 9.16.1 &lt;&lt;&gt;&gt; auth.target.com CNAME</p>
                                    <p class="text-zinc-400 mt-1">;; ANSWER SECTION:</p>
                                    <p class="text-amber-400 font-bold bg-amber-500/5 px-2 py-1 fit-content border border-amber-500/20 rounded">auth.target.com.    300  IN  CNAME   target-auth-app.s3.amazonaws.com.</p>
                                </div>

                                <p class="text-emerald-400 flex items-center gap-2 font-semibold border-b border-white/5 pb-2 pt-2"><span>➜</span> <span class="text-blue-400">curl</span> -I http://target-auth-app.s3.amazonaws.com</p>
                                <div class="opacity-80 pl-4 py-2">
                                    <p class="text-red-400 font-bold">HTTP/1.1 404 Not Found</p>
                                    <p class="text-zinc-400">&lt;Code&gt;NoSuchBucket&lt;/Code&gt;</p>
                                    <p class="text-zinc-400">&lt;Message&gt;The specified bucket does not exist&lt;/Message&gt;</p>
                                </div>

                                <p class="text-emerald-400 flex items-center gap-2 mt-4 font-semibold border-t border-white/5 pt-4"><span>➜</span> <span class="text-blue-400">aws</span> s3api create-bucket --bucket target-auth-app --region us-east-1</p>
                                <div class="pl-4 mt-2 mb-4">
                                <pre class="text-green-300 bg-black/60 p-4 rounded-xl border border-green-500/20 shadow-[0_0_15px_rgba(34,197,94,0.1)]">
&#123;
  "Location": "/target-auth-app"
&#125;
<span class="text-zinc-500"># Attacker now fully controls auth.target.com! They can serve malware, collect credentials, or read cookies.</span>
</pre>
                                </div>
                            </div>
                        </div>
                    </div>
                {:else if activeTab === 2}
                    <div transition:fade={{duration: 200}}>
                        <h3 class="text-lg font-semibold text-emerald-400 mb-4">Prevention & Remediation</h3>
                        <p class="text-sm text-zinc-400 mb-6 max-w-3xl">Subdomain takeovers are prevented by strict lifecycle management of infrastructure and continuous scanning of DNS zones against active assets.</p>

                        <!-- Code Example -->
                        <div class="relative bg-[#0d1117] rounded-xl border border-emerald-500/30 shadow-[0_0_30px_rgba(16,185,129,0.1)] overflow-hidden font-mono text-sm mb-6">
                            <div class="absolute top-0 right-0 bg-emerald-500/10 text-emerald-400 px-4 py-1.5 rounded-bl-xl text-xs font-bold border-b border-l border-emerald-500/30 tracking-widest uppercase">CI/CD Safety Guard (Terraform)</div>
                            <div class="p-5 overflow-x-auto pt-10 custom-scrollbar">
                                <pre class="text-zinc-300 leading-relaxed">
<span class="text-zinc-500 italic"># Instead of manual provisioning, use Infrastructure as Code (IaC)</span>
<span class="text-zinc-500 italic"># which tracks dependencies and ensures DNS is deleted before</span>
<span class="text-zinc-500 italic"># the cloud resource is destroyed.</span>

<span class="text-purple-400">resource</span> <span class="text-green-300">"aws_s3_bucket"</span> <span class="text-blue-400">"app_frontend"</span> &#123;
  <span class="text-teal-400">bucket</span> = <span class="text-green-300">"company-auth-frontend-xx921"</span>
&#125;

<span class="text-purple-400">resource</span> <span class="text-green-300">"aws_route53_record"</span> <span class="text-blue-400">"auth_cname"</span> &#123;
  <span class="text-teal-400">zone_id</span> = aws_route53_zone.primary.zone_id
  <span class="text-teal-400">name</span>    = <span class="text-green-300">"auth.company.com"</span>
  <span class="text-teal-400">type</span>    = <span class="text-green-300">"CNAME"</span>
  <span class="text-teal-400">ttl</span>     = <span class="text-orange-300">300</span>
  
  <span class="text-zinc-400 italic"># ✅ Explicit dependency ensures safe teardown order:</span>
  <span class="text-teal-400">records</span> = [aws_s3_bucket.app_frontend.bucket_domain_name]
&#125;</pre>
                            </div>
                        </div>

                        <!-- Mitigation Checklist -->
                        <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                            <div class="bg-emerald-950/10 border border-emerald-500/20 rounded-xl p-5 flex gap-4 hover:border-emerald-500/40 transition-colors">
                                <div class="bg-emerald-500/10 p-2 rounded-lg h-fit">
                                    <Search size={20} class="text-emerald-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-emerald-300 mb-1.5">Continuous Monitoring</h4>
                                    <p class="text-xs text-emerald-200/70 leading-relaxed">Regularly export your DNS definitions and scan them against a known list of provider edge-cases (e.g. using `subjack` or `nuclei`).</p>
                                </div>
                            </div>
                            <div class="bg-blue-950/10 border border-blue-500/20 rounded-xl p-5 flex gap-4 hover:border-blue-500/40 transition-colors">
                                <div class="bg-blue-500/10 p-2 rounded-lg h-fit">
                                    <Link size={20} class="text-blue-400 shrink-0"/>
                                </div>
                                <div>
                                    <h4 class="text-sm font-semibold text-blue-300 mb-1.5">Verification Mandates</h4>
                                    <p class="text-xs text-blue-200/70 leading-relaxed">Require TXT record verifications for external services to prove domain ownership before they will deploy code to the endpoint.</p>
                                </div>
                            </div>
                        </div>
                    </div>
                {/if}
            </div>

            <!-- Footer Area -->
            <div class="bg-[#0A0C10] p-4 border-t border-amber-500/10 flex items-center justify-between shrink-0 shadow-[0_-10px_20px_-10px_rgba(0,0,0,0.5)] z-20">
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
                            class="flex items-center gap-1.5 px-6 py-2.5 bg-amber-600 hover:bg-amber-500 text-white text-sm font-semibold rounded-xl shadow-[0_0_20px_rgba(245,158,11,0.4)] transition-all transform hover:scale-105 active:scale-95 focus:ring-2 focus:ring-amber-500/50 outline-none"
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
