# JADX Integration

## Status

Planned. This adds an authorized Android static-analysis workflow to WebQ: import APK/AAB/DEX/JAR files, run JADX decompilation, browse generated Java/Kotlin-like sources and resources, search across the project, extract Android metadata, run lightweight static security checks, and persist analysis history in SQLite.

## Objective

Build a first-class "Android Reverse Engineering" module around JADX without weakening WebQ's current local-first security model. The feature must be useful for apps the operator owns, lab APKs, CTF/coursework, malware-analysis sandboxes, and explicitly authorized security research. It must not ship with bypass-focused or third-party-targeting defaults.

## Non-Goals

- No app-store scraping or unauthorized APK acquisition.
- No automated exploit generation.
- No signing/repacking workflow in the JADX pass; APK rebuild belongs to a separate apktool-style feature if added later.
- No cloud upload of binaries, sources, manifests, reports, or hashes.
- No execution of decompiled code.

## Tooling Strategy

- [ ] Add a backend module, likely `src-tauri/src/jadx.rs`, that owns tool discovery, subprocess execution, project paths, and parsing.
- [ ] Detect Java with `java -version`; show an explicit "Java required for JADX" message and do not silently install a JRE/JDK.
- [ ] Detect an existing `jadx`/`jadx-gui` on `PATH` first.
- [ ] Add an app-managed JADX install path under Tauri app data, for example `<app_data>/tools/jadx/`.
- [ ] Pin a JADX CLI version in one place, for example a Rust constant or config object, and download only official release archives from `https://github.com/skylot/jadx/releases`.
- [ ] Verify downloaded archives before extraction at minimum by archive structure and zip-slip checks; add SHA-256 pinning if the release process exposes stable checksums.
- [ ] Extract JADX with path traversal protection: every archive entry must resolve under the destination directory.
- [ ] Keep the installed tool and all analysis projects outside the app bundle so updates do not wipe user data.
- [ ] Support settings override for a custom `jadx` binary path.
- [ ] Expose a dependency/status card in Settings/Engine Health for Java and JADX, matching the existing dependency check style.

## Storage Model

- [ ] Create an app-data workspace, for example `<app_data>/jadx/projects/<project_id>/`.
- [ ] Store uploaded/imported binaries under `<project>/input/`, never in `src/` or repo paths.
- [ ] Store decompiled output under `<project>/decompiled/`.
- [ ] Store derived metadata under `<project>/metadata.json`.
- [ ] Store static-analysis findings under `<project>/findings.json`.
- [ ] Store report exports under `<project>/reports/`.
- [ ] Add cleanup/delete project support that removes all local files for a project.
- [ ] Enforce maximum import size in settings, with a clear error before writing huge files.
- [ ] Never index or display absolute local paths in exportable reports unless the user explicitly asks for diagnostics.

## SQLite Persistence

- [ ] Add `android_projects` table with project id, display name, package name, version name/code, input type, input SHA-256, size, created/updated timestamps, status, and error.
- [ ] Add `android_analysis_results` table keyed by project id with manifest summary, permissions summary, component counts, finding counts, and raw JSON blob.
- [ ] Add `android_source_index` table or FTS virtual table for searchable class/resource paths and snippets if file-system search is too slow.
- [ ] Add migration code in `src-tauri/src/db.rs`.
- [ ] Add project list, project detail, delete project, and latest-project lookup commands.
- [ ] Decide whether JADX analyses should also create rows in the existing `scans`/`scan_results` tables using a module name such as `AndroidJadx`.
- [ ] Ensure failed analyses persist an error row without storing partial, misleading findings as successful results.

## Backend Commands

- [ ] `get_jadx_status() -> JadxStatus`
  - Java installed/version/path.
  - JADX installed/version/path/source.
  - Managed install version/path.
  - Workspace stats: project count and disk usage.
- [ ] `install_jadx() -> JadxStatus`
  - Download, verify, extract, chmod executable bits where needed.
  - Return status and audit/log the installed version/path.
- [ ] `import_android_artifact(path_or_dialog_result, display_name?) -> AndroidProject`
  - Prefer Tauri dialog/file permissions for user-selected files.
  - Accept `.apk`, `.aab`, `.dex`, `.jar`, and zipped source inputs only if explicitly supported by JADX.
  - Compute SHA-256 and reject unsupported extensions.
- [ ] `analyze_android_project(project_id, options) -> AndroidProject`
  - Run JADX CLI with argv arrays only, never shell strings.
  - Options: deobfuscation on/off, resources on/off, threads, comments, show inconsistent code, skip sources/resources for faster metadata-only pass.
  - Emit progress events such as `jadx-progress`.
- [ ] `list_android_projects(filter/sort/page) -> PaginatedProjects`
- [ ] `get_android_project(project_id) -> AndroidProjectDetail`
- [ ] `browse_android_project(project_id, relative_path) -> DirectoryListing`
- [ ] `read_android_project_file(project_id, relative_path) -> TextFile`
- [ ] `search_android_project(project_id, query, filters) -> SearchResults`
- [ ] `get_android_manifest_summary(project_id) -> ManifestSummary`
- [ ] `get_android_static_findings(project_id) -> StaticFindings`
- [ ] `delete_android_project(project_id) -> ()`
- [ ] `export_android_report(project_id, format) -> ExportPath`

## Subprocess Safety

- [ ] Use `tokio::process::Command` or `std::process::Command` with explicit argv lists.
- [ ] Never invoke host shell.
- [ ] Canonicalize project paths and reject path escapes before any read/write/delete.
- [ ] Reject absolute `relative_path` inputs and any `..` path segments.
- [ ] Treat symlink escapes as invalid.
- [ ] Bound command runtime and kill the child process on cancel/timeout.
- [ ] Capture stdout/stderr with bounded buffers so large JADX output cannot exhaust memory.
- [ ] Surface a concise error plus a "show details" diagnostic payload for failures.

## Progress And Cancellation

- [ ] Add a long-running job model for JADX analysis if the current scan-progress pattern is insufficient.
- [ ] Emit phases:
  - validating input
  - preparing workspace
  - running JADX
  - indexing sources
  - parsing manifest
  - running static checks
  - saving history
  - complete/error/cancelled
- [ ] Show progress in a `ScanTerminal`-style console or a dedicated analysis terminal.
- [ ] Add cancel support that terminates the JADX child process and marks the project as cancelled.
- [ ] Resume gracefully: a cancelled or failed project should be reopenable, deletable, and rerunnable.

## Metadata Extraction

- [ ] Parse `AndroidManifest.xml` from JADX output or from APK archive fallback.
- [ ] Extract package name, version code/name, min/target SDK, app label, debuggable flag, backup flag, network security config reference, exported components, permissions, uses-feature, providers, receivers, services, activities, and intent filters.
- [ ] Parse resources for string references used by labels, authorities, exported components, and network security config where practical.
- [ ] Detect split APK/AAB limitations and clearly label unsupported or partial cases.
- [ ] Compute cryptographic hashes of the input: SHA-256 at minimum, optionally SHA-1/MD5 for analyst compatibility.

## Static Security Checks

- [ ] Manifest checks:
  - exported components without permission protection.
  - `android:debuggable="true"`.
  - `android:allowBackup="true"` and backup rules context.
  - cleartext traffic allowed.
  - custom network security config with trust anchors.
  - risky permissions: SMS, contacts, location, microphone, camera, install packages, accessibility, overlays, notification listener, VPN.
- [ ] Code/source checks:
  - hardcoded URLs, IPs, API keys, tokens, secrets, and certificates.
  - weak crypto usage: DES, ECB, MD5/SHA1 signatures, static IVs, insecure random.
  - WebView risky settings: JavaScript enabled, file access, universal access from file URLs, addJavascriptInterface.
  - TLS bypass patterns: permissive TrustManager, HostnameVerifier returning true, certificate pinning disabled.
  - root/emulator/debug detection signals for analyst awareness, not bypass generation.
  - dynamic code loading: DexClassLoader, PathClassLoader, System.loadLibrary, reflection hot spots.
  - insecure storage hints: SharedPreferences secrets, world-readable/writeable flags, external storage secrets.
- [ ] Resource checks:
  - secrets in `res/values/*.xml`.
  - endpoint lists in strings and network security config.
  - suspicious domains/IPs grouped by scheme and host.
- [ ] Classify findings by severity and confidence with evidence snippets and file paths.
- [ ] Avoid presenting decompiler guesses as certainty; label JADX-inferred source accordingly.

## Frontend Navigation

- [ ] Add a sidebar entry, likely "Android Analysis" under Reconnaissance or Security Assessment.
- [ ] Add route group under `src/routes/android/` or `src/routes/assessment/android-jadx/`.
- [ ] Add dashboard card on `src/routes/+page.svelte` only if the module is considered a primary workflow.
- [ ] Add command palette entries for import, project list, and recent Android analyses.
- [ ] Add i18n message keys for visible labels if the surrounding route uses Paraglide messages.

## Frontend Views

- [ ] Landing/project list:
  - import/select APK button.
  - JADX status/install card.
  - recent projects table with package, version, hash prefix, status, finding counts, created date.
  - filters for package, status, severity, and date.
- [ ] Analysis run view:
  - options panel with toggles for sources/resources/deobfuscation/static checks.
  - progress terminal.
  - cancel/rerun controls.
- [ ] Project overview:
  - package identity card.
  - input hash/size/type.
  - manifest risk summary.
  - findings severity chart/list.
  - component counts.
  - top endpoints/secrets candidates.
- [ ] Manifest view:
  - permissions table.
  - exported components table with filters.
  - SDK/build flags.
  - network security config summary.
- [ ] Source browser:
  - breadcrumb directory browser.
  - searchable file list.
  - text/code viewer.
  - large-file handling and binary-file refusal.
  - copy file path/snippet buttons using existing clipboard/toast pattern.
- [ ] Search view:
  - query input with file-type filters.
  - regex toggle if safe and bounded.
  - result snippets with line numbers.
- [ ] Findings view:
  - severity tabs.
  - evidence snippet.
  - affected file/component.
  - remediation/analyst note.
  - false-positive/mark-reviewed local state if useful.
- [ ] Report view:
  - export JSON/Markdown/PDF/DOCX using existing report utilities where possible.
  - include manifest summary, findings, hashes, and environment/tool versions.

## File Browser And Search Safety

- [ ] Reuse the visual conventions from existing tables/cards, but keep source browsing dense and analyst-focused.
- [ ] Do not use a browser-side virtualized code editor dependency unless plain text viewing becomes inadequate.
- [ ] Add hard limits for displayed file size and search result count.
- [ ] Search only within the selected project root.
- [ ] Escape all snippets and paths before rendering.
- [ ] Do not auto-link arbitrary URLs from decompiled content unless sanitized.

## Reporting

- [ ] Define `AndroidJadxReport` TypeScript/Rust serializable shape.
- [ ] Export JSON with full structured metadata and findings.
- [ ] Export Markdown with analyst-readable sections.
- [ ] Export PDF/DOCX via existing report exporter patterns if compatible.
- [ ] Include a clear "authorized analysis only" statement in report metadata.
- [ ] Include JADX version, Java version, WebQ version, analysis options, input hash, and timestamp.
- [ ] Exclude local absolute paths by default.

## Dependency And Packaging Impact

- [ ] Update dependency alert/Engine Health to include Java and JADX.
- [ ] Document Java/JADX prerequisites in `docs/readme.md`, `docs/release.md`, and Settings dependency help.
- [ ] Update macOS release checklist for Java/JADX behavior in unsigned/notarized apps.
- [ ] Update Windows release checklist for JADX executable/archive handling and Defender false-positive considerations.
- [ ] Update Snap packaging notes if managed JADX install or Java runtime access requires extra plugs/content.
- [ ] Ensure app-data paths work in Tauri dev, macOS app bundle, Windows installer, Linux AppImage/deb/rpm, and Snap.

## Security And Abuse-Resistance Notes

- [ ] UI copy must say "authorized Android app analysis only" near import and report export.
- [ ] Do not include sample targets from real third-party apps.
- [ ] Do not ship bypass scripts, repackaging helpers, anti-cheat bypass guidance, DRM bypass workflows, or credential extraction automation.
- [ ] Findings should identify risky patterns and evidence, not generate exploit steps.
- [ ] Add warning when importing unknown APKs: decompilation is static, but analysts should still treat files as untrusted.
- [ ] Never execute imported APK code.

## Tests To Add

- [ ] Rust unit tests:
  - tool path resolution prefers settings override, managed install, then PATH.
  - Java/JADX status parsing for missing, present, and command-failure cases.
  - archive extraction rejects zip-slip paths and absolute members.
  - project path resolution rejects `..`, absolute paths, and symlink escapes.
  - manifest parser handles exported components, permissions, debuggable, backup, cleartext, and missing fields.
  - static finding classifiers map known fixture snippets to expected severity/confidence.
- [ ] Rust integration tests:
  - analyze fixture APK/JAR with mocked JADX command output.
  - cancellation kills the child process and leaves a cancellable project state.
  - SQLite project/result persistence and delete cascade.
  - export report contains hash, tool versions, and findings.
- [ ] Frontend tests/static checks:
  - `npm run check`.
  - project list renders empty/loading/error states.
  - source browser blocks unsafe paths at UI boundary before backend call.
  - findings filters and severity tabs render expected counts.
  - report export button handles success/failure toasts.
- [ ] End-to-end smoke tests:
  - import a tiny benign fixture APK.
  - run JADX analysis.
  - browse manifest/source.
  - search for a known string.
  - view findings.
  - export JSON/Markdown report.
  - delete project and confirm files/history are removed or marked deleted as designed.

## Acceptance Criteria

- [ ] `npm run check` passes.
- [ ] `npm run build` passes.
- [ ] `cargo test` or targeted Rust tests pass in `src-tauri`.
- [ ] `cargo check` passes in `src-tauri`.
- [ ] A benign fixture APK can be imported, analyzed, browsed, searched, reported, and deleted.
- [ ] Java missing, JADX missing, JADX install failure, analysis failure, and cancellation all produce clear UI states.
- [ ] No file read/write/delete can escape the project workspace.
- [ ] Reports and history include tool versions and input SHA-256.
- [ ] The feature is documented and linked from the active todo index.
