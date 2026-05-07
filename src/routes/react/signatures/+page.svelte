<script lang="ts">
    import { ChevronDown, ChevronUp, Code, FileCode, ShieldAlert, FileSearch, Terminal } from "lucide-svelte";
    
    // Using a portion of the data from REQUEST.md to keep the file size reasonable
    const signatures = [
        {
            severity: "Critical",
            color: "red",
            items: [
                {
                    category: "SQL Injection",
                    ids: "sqli:classic_tautology, union_select, blind_time, error_based, stacked",
                    description: "Database dump, Auth bypass, Remote Code Execution (via xp_cmdshell).",
                    curl: 'curl -X POST -d "username=admin\' OR 1=1 --" http://target/api/login',
                    remediation: "Use Prepared Statements / Parameterized Queries. Employ ORMs safely."
                },
                {
                    category: "Command Injection",
                    ids: "cmdi:unix_pipe, unix_advanced, windows, blind_oob",
                    description: "Full Remote Code Execution (RCE) as the web user.",
                    curl: 'curl "http://target/api/ping?host=127.0.0.1;id"',
                    remediation: "Avoid exec/system; use language-specific APIs and strict input validation."
                },
                {
                    category: "RSC Flight Injection",
                    ids: "rsc_attack:flight_injection",
                    description: "Internal server action manipulation; potential RCE.",
                    curl: "curl -X POST -d '[[ \"$\", \"@action\", null, {\"type\": \"blob_handler\"} ]]' http://target/_rsc/page",
                    remediation: "Keep React/Next.js patched (CVE-2025-55182). Strictly validate RSC payloads."
                }
            ]
        },
        {
            severity: "High",
            color: "orange",
            items: [
                {
                    category: "XSS",
                    ids: "xss:reflected, polyglot, stored_payload",
                    description: "Session hijacking, client-side execution.",
                    curl: 'curl "http://target/search?q=<script>alert(1)</script>"',
                    remediation: "Context-aware output encoding (HTML, JS contexts); strict CSP."
                },
                {
                    category: "Path Traversal",
                    ids: "path_traversal:dot_dot_slash, absolute_path",
                    description: "Sensitive file disclosure.",
                    curl: 'curl "http://target/download?file=../../../../etc/shadow"',
                    remediation: "Use absolute paths; strip ../; validate input against allowed directories."
                }
            ]
        },
        {
            severity: "Medium",
            color: "yellow",
            items: [
                {
                    category: "Open Redirect",
                    ids: "open_redirect:url_param",
                    description: "Phishing, OAuth token theft.",
                    curl: 'curl "http://target/login?next=http://evil.com"',
                    remediation: "Whitelist redirect URLs; use relative paths exclusively."
                }
            ]
        },
        {
            severity: "Low",
            color: "blue",
            items: [
                {
                    category: "Fake Crawlers",
                    ids: "user_agent:fake_crawler",
                    description: "Reconnaissance gathering.",
                    curl: 'curl -A "sqlmap/1.5" http://target/',
                    remediation: "Block known malicious User-Agents at the WAF level."
                }
            ]
        }
    ];

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

    {#each signatures as group}
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
