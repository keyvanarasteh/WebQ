<script lang="ts">
  import { CheckCircle, Database, Server, Cpu, Navigation, ShieldPlus } from "lucide-svelte";

  // Port Mapping
  const portCategories = [
    {
      name: "Databases & Key-Value Stores",
      icon: Database,
      ports: [
        { code: 3306, label: "MySQL" }, { code: 5432, label: "PostgreSQL" }, 
        { code: 1433, label: "MSSQL" }, { code: 1434, label: "MSSQL Mgmt" },
        { code: 1521, label: "Oracle" }, { code: 27017, label: "MongoDB" },
        { code: 6379, label: "Redis" }, { code: 11211, label: "Memcached" },
        { code: 5984, label: "CouchDB" }, { code: 7474, label: "Neo4j" },
        { code: 8086, label: "InfluxDB" }, { code: 9200, label: "Elasticsearch" },
        { code: 9042, label: "Cassandra" }, { code: 7000, label: "Cassandra Intra" },
        { code: 50000, label: "SAP/DB2" }, { code: 4333, label: "mSQL" },
      ]
    },
    {
      name: "Remote Desktop & File Access",
      icon: Server,
      ports: [
        { code: 21, label: "FTP" }, { code: 22, label: "SSH" },
        { code: 23, label: "Telnet" }, { code: 3389, label: "RDP" },
        { code: 5900, label: "VNC" }, { code: 5800, label: "VNC HTTP" },
        { code: 5938, label: "TeamViewer" }, { code: 4899, label: "Radmin" },
        { code: 5632, label: "pcAnywhere" }, { code: 873, label: "Rsync" },
        { code: 143, label: "IMAP" }, { code: 110, label: "POP3" },
        { code: 139, label: "NetBIOS" }, { code: 445, label: "SMB/DS" },
        { code: 2049, label: "NFS" }, { code: 389, label: "LDAP" },
        { code: 636, label: "LDAPS" }, { code: 548, label: "AFP" }
      ]
    },
    {
      name: "DevOps & Cloud Infrastructure",
      icon: Cpu,
      ports: [
        { code: 6443, label: "K8s API" }, { code: 10250, label: "Kubelet" },
        { code: 30000, label: "K8s NodePort" }, { code: 2375, label: "Docker" },
        { code: 2376, label: "Docker SSL" }, { code: 9090, label: "Prometheus" },
        { code: 9092, label: "Kafka" }, { code: 2181, label: "ZooKeeper" },
        { code: 5672, label: "AMQP" }, { code: 61616, label: "ActiveMQ" },
        { code: 1883, label: "MQTT" }, { code: 50070, label: "Hadoop" },
        { code: 3690, label: "SVN" }, { code: 9443, label: "Portainer" },
        { code: 8090, label: "Confluence" }, { code: 9000, label: "SonarQube" }
      ]
    },
    {
      name: "Web, Proxies & Admin Panels",
      icon: Navigation,
      ports: [
        { code: 80, label: "HTTP" }, { code: 443, label: "HTTPS" },
        { code: 8080, label: "HTTP Proxy" }, { code: 8443, label: "HTTPS Alt" },
        { code: 8181, label: "GlassFish" }, { code: 2082, label: "cPanel" },
        { code: 2086, label: "WHM" }, { code: 10000, label: "Webmin" },
        { code: 2222, label: "DirectAdmin" }, { code: 1080, label: "SOCKS" },
        { code: 3128, label: "Squid" }, { code: 1194, label: "OpenVPN" },
        { code: 1723, label: "PPTP" }, { code: 4500, label: "IPSec NAT-T" },
        { code: 4444, label: "Metasploit" }, { code: 5000, label: "UPnP" }
      ]
    }
  ];
</script>

<div class="animate-fade-in space-y-8">
  <!-- Stats Overview relative to backend -->
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <div class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden flex flex-col justify-between">
      <div class="absolute -right-4 -top-4 w-24 h-24 bg-magenta-500/10 rounded-full blur-2xl"></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">Monitored Infra Ports</p>
      <p class="text-5xl font-black text-primary-text mt-2">+130</p>
    </div>
    <div class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden">
      <div class="absolute -right-4 -top-4 w-24 h-24 bg-green-500/10 rounded-full blur-2xl"></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">Async Execution</p>
      <p class="text-5xl font-black text-green-400 mt-2">10ms~</p>
    </div>
    <div class="bg-surface border border-base rounded-2xl p-6 relative overflow-hidden">
      <div class="absolute -right-4 -top-4 w-24 h-24 bg-purple-500/10 rounded-full blur-2xl"></div>
      <p class="text-sm font-semibold text-muted uppercase tracking-widest">Backend Driver</p>
      <p class="text-2xl font-black text-purple-400 mt-4">Web-Analyzer ThreadPool</p>
    </div>
  </div>

  <!-- Checklist Grid -->
  <div class="space-y-10 border-t border-base pt-10">
    {#each portCategories as category}
      <div>
        <h3 class="text-xl font-bold text-accent uppercase tracking-widest flex items-center gap-3 mb-6">
          {#if category.icon}<category.icon class="size-6 text-magenta-400" />{/if}
          {category.name}
        </h3>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-3">
          {#each category.ports as port}
            <div class="relative overflow-hidden flex flex-col items-start p-3 rounded-lg border bg-surface/50 border-magenta-500/30 hover:bg-magenta-500/10 group cursor-default shadow-sm transition-all duration-300">
              <div class="flex justify-between w-full items-center mb-1">
                <span class="text-lg font-mono text-primary-text font-black tracking-widest">{port.code}</span>
                <CheckCircle class="size-4 text-green-400" />
              </div>
              <span class="text-xs text-muted uppercase font-bold tracking-wider">{port.label}</span>
              <div class="absolute -bottom-2 -right-2 bg-magenta-500/10 w-12 h-12 rounded-full blur-xl group-hover:bg-magenta-500/20 transition-all"></div>
            </div>
          {/each}
        </div>
      </div>
    {/each}
  </div>
</div>
