# Publishing WebQ to the Snap Store (Comprehensive Guide)

To publish the **WebQ** (Tauri + Svelte 5 + Rust) application to the Snap Store in Linux across **all architectures**, you must configure `snapcraft.yaml` for multi-architecture targeting, handle CI/CD authentication, and leverage remote builders. 

Snap packages **do not require traditional manual GPG signing** by the developer; instead, the Snap Store uniquely signs the snapped binaries automatically upon successful upload, granting them an "Assertion" of trust. You only need to authenticate yourself to the store.

Here is the complete, comprehensive guide.

---

## 1. Prerequisites & Developer Authentication
You must first register the app name and export an authentication token to be used locally or injected into your CI/CD pipelines (GitHub Actions).

1. **Install Snapcraft & Multipass**:
   ```bash
   sudo snap install snapcraft --classic
   sudo snap install multipass
   ```
2. **Register Developer Account & App Name**:
   Log into [snapcraft.io](https://snapcraft.io/) and register the name `webq`.
   
3. **Export Login Token for CI/CD** *(Authentication/Signing Authority)*:
   To allow GitHub Actions (or your CLI) to upload packages to the Store automatically, issue an expiring token locked to your specific app:
   ```bash
   snapcraft export-login --snaps webq snapcraft-token.txt
   ```
   *Take the contents of `snapcraft-token.txt` and store it as a secure **GitHub Secret** named `SNAPCRAFT_STORE_CREDENTIALS`.*

---

## 2. The Comprehensive `snapcraft.yaml` (Multi-Architecture)
Create `snap/snapcraft.yaml`. To support **all architectures** (e.g., `amd64`, `arm64`, `armhf`), we add the `architectures` block.

```yaml
name: webq 
base: core22 
version: '1.0.0'
summary: A high-performance web intelligence and cybersecurity platform.
description: |
  WebQ is a cross-platform Web Intelligence and Cybersecurity platform 
  built with Rust, Svelte 5, and Tauri, powered by the web-analyzer crate.

grade: stable
confinement: strict

# Explicitly target multiple architectures for the build
architectures:
  - build-on: amd64
  - build-on: arm64
  # Uncomment if targeting 32-bit ARM (e.g., older Raspberry Pi)
  #- build-on: armhf

apps:
  webq:
    command: usr/bin/webq
    extensions: [gnome] 
    plugs:
      - network
      - network-bind
      - home

parts:
  node-deps:
    plugin: npm
    source: .
    npm-node-version: "20.10.0"

  webq-gui:
    after: [node-deps]
    plugin: rust
    source: src-tauri
    override-build: |
      npm run tauri build
      mkdir -p $CRAFT_PART_INSTALL/usr/bin
      cp target/release/webq $CRAFT_PART_INSTALL/usr/bin/webq
    build-packages:
      - libwebkit2gtk-4.1-dev
      - build-essential
      - curl
      - wget
      - file
      - libxdo-dev
      - libssl-dev
      - libayatana-appindicator3-dev
      - librsvg2-dev
```

---

## 3. Building for All Architectures (Remote Build)
If you run `snapcraft` locally, it will only build the snap for your current host architecture (`amd64`). To compile for **all defined architectures** natively without needing QEMU cross-compilation on your PC, utilize Canonical's free Launchpad build servers.

Run this command from your project root:
```bash
snapcraft remote-build
```
*   Snapcraft will package your source code, send it to Launchpad servers, and natively compile it for `amd64`, `arm64`, etc.
*   Once completed, it will download all the resulting `.snap` files (e.g., `webq_1.0.0_amd64.snap`, `webq_1.0.0_arm64.snap`) back to your directory.

To test an architecture locally before uploading:
```bash
sudo snap install ./webq_1.0.0_amd64.snap --dangerous
```

---

## 4. Publishing Across All Architectures
Once you have the `.snap` binaries returned from `remote-build`, you upload them one by one. The Store assigns the architectures based on the binaries uploaded.

1. **Upload `amd64` to edge channel**:
   ```bash
   snapcraft upload --release=edge webq_1.0.0_amd64.snap
   ```
2. **Upload `arm64` to edge channel**:
   ```bash
   snapcraft upload --release=edge webq_1.0.0_arm64.snap
   ```

At this stage, users running `sudo snap install webq --edge` on their Linux machine will automatically receive the correct architecture (AMD64 for standard PCs, ARM64 for Apple Silicon Linux VMs/Raspberry Pi 4s).

---

## 5. CI/CD GitHub Actions Automation (The Ultimate Way)
The most comprehensive way to achieve multi-architecture builds is to completely offload it to GitHub Actions utilizing the Login Token you exported in Step 1.

Create `.github/workflows/snap.yml`:

```yaml
name: Publish multi-arch Snap
on:
  push:
    tags:
      - 'v*' # Trigger build on version tags (e.g., v1.0.0)

jobs:
  build:
    strategy:
      matrix:
        # Define the architectures you want GitHub to build
        architecture: [amd64, arm64]
    
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@v4

      # Setup QEMU for ARM64 builds on standard GitHub Runners
      - name: Set up QEMU
        if: matrix.architecture == 'arm64'
        uses: docker/setup-qemu-action@v3
        
      - name: Build Snap
        uses: snapcore/action-build@v1
        id: snapBuild
        with:
          # Snapcraft needs to build specific to the matrix loop
          build-info: true
          snapcraft-args: --target-arch=${{ matrix.architecture }}

      - name: Publish to Snap Store
        uses: snapcore/action-publish@v1
        env:
          # Injected from the GitHub Secret created in Step 1
          SNAPCRAFT_STORE_CREDENTIALS: ${{ secrets.SNAPCRAFT_STORE_CREDENTIALS }}
        with:
          snap: ${{ steps.snapBuild.outputs.snap }}
          release: edge
```

### Summary of the Lifecycle:
1. You `git tag v1.0.0` and `git push`.
2. GitHub runs the matrix for `amd64` and `arm64`.
3. The Canonical Action builds the snap.
4. The Action uses `SNAPCRAFT_STORE_CREDENTIALS` to authenticate.
5. The Snap Store receives it, Cryptographically Signs it, and registers it to the `edge` channel for both architectures simultaneously.
