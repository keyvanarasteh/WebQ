<script lang="ts">
  import { Shield, Github, Box, User, ExternalLink, Globe, Link } from 'lucide-svelte';
</script>

<div class="space-y-6 max-w-4xl mx-auto pb-10">
  <div class="border-b border-base pb-4">
      <h1 class="text-3xl font-black tracking-widest text-primary-text uppercase shadow-cyan-400 drop-shadow-sm flex items-center gap-3">
          <Shield class="w-8 h-8 text-cyan-500" /> About WebQ
      </h1>
      <p class="mt-2 text-muted">Web Intelligence and Cybersecurity Reconnaissance Platform.</p>
  </div>

  <section class="bg-surface/20 border border-base rounded-xl p-6 shadow-lg">
    <div class="flex items-center gap-4 mb-8">
      <div class="h-16 w-16 rounded-xl bg-cyan-400 flex items-center justify-center shadow-[0_0_20px_rgba(34,211,238,0.4)]">
          <Shield class="text-on-accent w-10 h-10" />
      </div>
      <div>
        <h2 class="text-2xl font-bold text-primary-text tracking-widest">WEBQ</h2>
        <p class="text-sm text-cyan-400 font-mono">v1.0.0-beta</p>
      </div>
    </div>

    <div class="space-y-4">
      <h3 class="text-lg font-semibold text-accent border-b border-base pb-2">Resources & Links</h3>
      
      <a href="https://github.com/keyvanarasteh/webq" target="_blank" rel="noopener noreferrer" class="flex items-center justify-between p-4 bg-background rounded-lg border border-base hover:bg-surface hover:border-cyan-500/50 transition-all group">
        <div class="flex items-center gap-4">
          <Github class="w-6 h-6 text-muted group-hover:text-primary-text transition-colors" />
          <div>
            <p class="font-medium text-primary-text">WebQ Source Code</p>
            <p class="text-sm text-muted">Contribute to the project or open issues on GitHub.</p>
          </div>
        </div>
        <ExternalLink class="w-5 h-5 text-muted group-hover:text-cyan-400 transition-colors" />
      </a>

      <a href="https://crates.io/crates/web-analyzer" target="_blank" rel="noopener noreferrer" class="flex items-center justify-between p-4 bg-background rounded-lg border border-base hover:bg-surface hover:border-cyan-500/50 transition-all group">
        <div class="flex items-center gap-4">
          <Box class="w-6 h-6 text-muted group-hover:text-primary-text transition-colors" />
          <div>
            <p class="font-medium text-primary-text">Engine: web-analyzer crate</p>
            <p class="text-sm text-muted">The high-performance Rust reconnaissance engine powering WebQ.</p>
          </div>
        </div>
        <ExternalLink class="w-5 h-5 text-muted group-hover:text-cyan-400 transition-colors" />
      </a>

      <h3 class="text-lg font-semibold text-accent border-b border-base pb-2 mt-8">Corporate</h3>
      
      <a href="https://qline.tech" target="_blank" rel="noopener noreferrer" class="flex items-center justify-between p-4 bg-background rounded-lg border border-base hover:bg-surface hover:border-cyan-500/50 transition-all group">
        <div class="flex items-center gap-4">
          <Globe class="w-6 h-6 text-muted group-hover:text-primary-text transition-colors" />
          <div>
            <p class="font-medium text-primary-text">QLine Tech</p>
            <p class="text-sm text-muted">Visit our main technology hub for advanced security solutions.</p>
          </div>
        </div>
        <ExternalLink class="w-5 h-5 text-muted group-hover:text-cyan-400 transition-colors" />
      </a>

      <a href="https://qline.tech/apps/webq/" target="_blank" rel="noopener noreferrer" class="flex items-center justify-between p-4 bg-background rounded-lg border border-base hover:bg-surface hover:border-cyan-500/50 transition-all group">
        <div class="flex items-center gap-4">
          <Link class="w-6 h-6 text-muted group-hover:text-primary-text transition-colors" />
          <div>
            <p class="font-medium text-primary-text">WebQ Product Page</p>
            <p class="text-sm text-muted">Official landing page, documentation and update changelogs.</p>
          </div>
        </div>
        <ExternalLink class="w-5 h-5 text-muted group-hover:text-cyan-400 transition-colors" />
      </a>
    </div>
  </section>

  <section class="bg-surface/20 border border-base rounded-xl p-6 shadow-lg mt-6">
    <h3 class="text-lg font-semibold text-accent border-b border-base pb-2 mb-4">Core Developer</h3>
    
    <div class="flex items-center justify-between p-4 bg-background rounded-lg border border-base">
        <div class="flex items-center gap-4">
          <div class="w-12 h-12 rounded-full bg-linear-to-tr from-cyan-500 to-fuchsia-500 flex items-center justify-center shadow-lg">
            <User class="w-6 h-6 text-white" />
          </div>
          <div>
            <p class="font-bold text-primary-text">Keyvan Arasteh <span class="text-xs font-mono text-cyan-400 bg-cyan-500/10 px-2 py-0.5 rounded ml-2">@k1ben</span></p>
            <p class="text-sm text-muted">Security Researcher & Software Engineer.</p>
          </div>
        </div>
        
        <a href="https://github.com/keyvanarasteh" target="_blank" rel="noopener noreferrer" class="p-2 rounded-lg bg-surface text-muted hover:text-cyan-400 hover:bg-cyan-500/10 transition-colors border border-transparent hover:border-cyan-500/30">
          <Github class="w-5 h-5" />
        </a>
    </div>
  </section>
</div>
