# Feature Request: Infrastructure Port Matrix & SSL Monitor

## Overview
Inside `domain_info.rs`, the engine executes an incredibly fast asynchronous port scan against a hardcoded array of over 100 `COMMON_PORTS`. This isn't just an Nmap scan; this is a rapid internal connectivity check that maps ports directly to services (e.g., `6443 -> Kubernetes-API`, `9200 -> Elasticsearch`, `5432 -> PostgreSQL`). Additionally, the engine does deep OpenSSL querying to calculate the precise `days_until_expiry` for SSL certificates.

## Missing Engine Controls & Visualization
1. **Port Service Matrix**: The UI should display the open ports not just as a raw list, but visually map them to the hardcoded service names defined in `COMMON_PORTS`. 
2. **Database & DevOps Flags**: If highly sensitive ports are open (e.g., Kubelet API on 10250, Docker on 2375, Redis on 6379), the UI should raise immediate red flags before deeper vulnerability scanning even begins.
3. **SSL Expiry Countdown**: The backend natively parses OpenSSL `notAfter` dates into `days_until_expiry`. The UI must visualize this as a countdown widget, with critical color coding if the certificate expires in < 30 days.

## Implementation Request
1. Enhance the `Intelligence Gathering` dashboard to include an "Infrastructure map".
2. Read the mapped ports from the `DomainInfoResult.open_ports` and cross-reference them with threat levels.
3. Build an `SslChainViewer` widget that prominently displays the expiry countdown derived from the backend data.
