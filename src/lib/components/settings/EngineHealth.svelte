<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { type } from '@tauri-apps/plugin-os';
  import { Shield, ShieldAlert, CheckCircle2, ChevronRight, Download, Server, Key, Search, Globe } from 'lucide-svelte';

  type DependencyStatus = {
      name: string;
      installed: boolean;
      version: string | null;
      error: string | null;
  };

  let osType = $state<string>("linux");
  let checking = $state(true);

  // The 4 pillars of the web-analyzer backend
  let deps = $state<{ [key: string]: DependencyStatus }>({
      nmap: { name: 'nmap', installed: false, version: null, error: null },
      openssl: { name: 'openssl', installed: false, version: null, error: null },
      dig: { name: 'dig', installed: false, version: null, error: null },
      subfinder: { name: 'subfinder', installed: false, version: null, error: null }
  });

  const toolsMeta: Record<string, any> = {
      nmap: { title: 'Nmap Mapper', desc: 'Port scanning and Zero-Day vulnerability detection.', icon: Server, color: 'text-cyan-400' },
      openssl: { title: 'OpenSSL Verifier', desc: 'Secure certificate and cryptographic analysis.', icon: Key, color: 'text-emerald-400' },
      dig: { title: 'DNS Resolver (Dig)', desc: 'DNS Record Extraction and Reconnaissance.', icon: Search, color: 'text-violet-400' },
      subfinder: { title: 'Subfinder', desc: 'Passive subdomain discovery tools.', icon: Globe, color: 'text-fuchsia-400' }
  };

  let selectedTool = $state<string | null>(null);

  onMount(async () => {
      osType = await type();
      
      const checks = ['nmap', 'openssl', 'dig', 'subfinder'].map(async (name) => {
          try {
              const res = await invoke<DependencyStatus>('check_dependency', { name });
              deps[name] = res;
          } catch (e) {
              if (deps[name]) {
                  deps[name].error = String(e);
              }
          }
      });

      await Promise.all(checks);
      checking = false;
  });

  function getInstallCommand(tool: string, os: string) {
      const isMac = os === 'macos';
      const isWin = os === 'windows';
      const isLinux = os === 'linux';

      switch (tool) {
          case 'nmap':
              if (isMac) return 'brew install nmap';
              if (isWin) return 'winget install Insecure.Nmap';
              return 'sudo apt install nmap';
          case 'openssl':
              if (isMac) return 'brew install openssl';
              if (isWin) return 'winget install ShiningLight.OpenSSL';
              return 'sudo apt install openssl';
          case 'dig':
              if (isMac) return 'brew install bind';
              if (isWin) return 'winget install ISC.BIND';
              return 'sudo apt install dnsutils';
          case 'subfinder':
              if (isMac || isLinux) return 'go install -v github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest';
              if (isWin) return 'go install -v github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest';
              return 'go install github.com/projectdiscovery/subfinder/v2/cmd/subfinder@latest';
          default:
              return 'Unknown install command';
      }
  }
</script>

<div class="space-y-6">
  <div class="border-b border-base pb-4">
      <h2 class="text-xl font-bold tracking-widest text-primary-text uppercase shadow-cyan-400 drop-shadow-sm flex items-center gap-3">
          <Shield class="w-6 h-6 text-cyan-500" /> Scanner Engine Status
      </h2>
      <p class="mt-2 text-sm text-muted">Real-time health telemetry for underlying system reconnaissance tools.</p>
  </div>

  <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mt-6">
      {#each Object.entries(deps) as [key, status] (key)}
          {@const meta = toolsMeta[key]}
          {@const Svg = meta.icon}
          
          <div class="bg-surface/30 border border-base rounded-xl p-5 hover:bg-surface/50 transition-colors shadow-lg relative overflow-hidden group">
              <!-- Background accent glow -->
              <div class="absolute -right-10 -top-10 w-32 h-32 bg-linear-to-br from-transparent to-cyan-500/5 rounded-full blur-2xl group-hover:to-cyan-400/10 transition-all"></div>
              
              <div class="flex items-start justify-between relative z-10">
                  <div class="flex items-center gap-4">
                      <div class="p-3 bg-background rounded-lg border border-base shadow-inner">
                          <Svg class="w-6 h-6 {meta.color}" />
                      </div>
                      <div>
                          <h3 class="text-lg font-bold text-primary-text">{meta.title}</h3>
                          <p class="text-sm text-muted">{meta.desc}</p>
                      </div>
                  </div>
                  
                  {#if checking}
                    <div class="animate-pulse w-6 h-6 rounded-full bg-surface"></div>
                  {:else if status.installed}
                    <CheckCircle2 class="w-6 h-6 text-emerald-500" />
                  {:else}
                    <ShieldAlert class="w-6 h-6 text-rose-500 animate-pulse" />
                  {/if}
              </div>

              <div class="mt-6 flex flex-col sm:flex-row sm:items-center justify-between gap-4 relative z-10">
                  {#if checking}
                      <span class="text-xs font-mono text-muted animate-pulse">Running telemetry scan...</span>
                  {:else if status.installed}
                      <span class="text-xs font-mono text-emerald-400 bg-emerald-500/10 px-2 py-1 rounded truncate max-w-[250px]">
                          {status.version || 'Version detected'}
                      </span>
                  {:else}
                      <span class="text-xs font-mono text-rose-400 bg-rose-500/10 px-2 py-1 rounded">
                          MISSING BINARY
                      </span>
                      <button 
                          onclick={() => selectedTool = key}
                          class="flex items-center gap-2 px-4 py-1.5 text-sm font-semibold rounded-md bg-rose-500/10 text-rose-400 border border-rose-500/30 hover:bg-rose-500/20 transition-colors"
                      >
                          <Download class="w-4 h-4" /> Install 
                      </button>
                  {/if}
              </div>
          </div>
      {/each}
  </div>

  <!-- Built-in Mini Fix Dialog -->
  {#if selectedTool}
      <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
          <div class="bg-background border border-base rounded-2xl w-full max-w-lg shadow-2xl p-6 overflow-hidden relative">
              <h2 class="text-2xl font-bold text-primary-text mb-2 flex items-center gap-2">
                  <Download class="w-6 h-6 text-cyan-400" /> Fix {toolsMeta[selectedTool].title}
              </h2>
              <p class="text-muted mb-6">Your operating system ({osType}) is missing the required binaries for this scan module.</p>
              
              <div class="space-y-3">
                  <span class="block text-sm font-semibold text-accent uppercase tracking-wider">Run this in your terminal:</span>
                  <div class="bg-black/50 border border-base rounded-lg p-4 font-mono text-sm text-cyan-300 select-all overflow-x-auto relative group">
                      {getInstallCommand(selectedTool, osType)}
                  </div>
              </div>
              
              <div class="mt-8 flex justify-end gap-3">
                  <button 
                      onclick={() => selectedTool = null}
                      class="px-5 py-2 rounded-lg bg-surface hover:bg-base text-primary-text font-medium transition-colors"
                  >
                      I installed it
                  </button>
              </div>
          </div>
      </div>
  {/if}
</div>
