# AKClip – Rust Clipboard Utility – Agent Instructions

## Overview
**AKClip** is a lightweight, cross‑platform command‑line tool for reading from and writing to the system clipboard.  
It provides a single binary (`akclip`) that can be installed via Homebrew, Docker, or directly from the compiled release.

## Project Purpose
- **Read** the current clipboard contents and output to stdout.  
- **Write** text supplied via stdin (or a file) to the clipboard.  
- Support macOS, Linux, and Windows (via platform‑specific commands or APIs).  

## Architecture
- **Platform modules** (`src/clipboard/*.rs`) implement `get_clipboard` and `set_clipboard` using native tools (`pbpaste/pbcopy` on macOS, `xclip`/`wl-copy` on Linux, `powershell` on Windows).  
- **CLI entry point** (`src/main.rs`) parses command‑line arguments, selects the appropriate platform module via `#[cfg]`, and forwards data.  
- **Configuration** (e.g., custom clipboard commands) can be overridden via environment variables.

## Key Files & Directories
```
akclip/
├── Cargo.toml                # Crate metadata, version, dependencies
├── README.md                 # User documentation
├── AGENTS.md                 # This file – agent instructions
├── .github/                  # CI/CD workflows and scripts
│   └── workflows/
│       └── ci.yml           # GitHub Actions CI (cargo build --release)
├── .secrets                  # Symlinked to /home/akinus/dockge-stacks/dev-stack/.secrets
├── src/
│   ├── main.rs               # CLI entry point
│   └── clipboard/
│       ├── mod.rs            # Public interface
│       ├── macos.rs          # macOS implementation
│       ├── linux.rs          # Linux implementation
│       └── windows.rs        # Windows implementation
└── scripts/
    ├── fix.js                # Automated fix helper (used by CI)
    └── parse.js              # Parses AI‑generated responses
```

## Build System
- **Language:** Rust  
- **Build command:** `cargo build --release`  
- **Binary name:** `akclip`  
- **Binary location after build:** `target/release/akclip`  
- **Version source:** `Cargo.toml` (update the `version = "x.y.z"` field for releases)

### Local Build (for debugging)
```bash
cd /home/akinus/dockge-stacks/dev-stack/projects/akclip
cargo build --release
./target/release/akclip --help
```

> **NOTE:** Do **not** install Rust on the host machine for normal development. CI runs the build automatically. Use the local build only for quick sanity checks.

## CI / Webhook Integration
- **GitHub Actions** runs on every push, executing the build command above.
- **Webhook endpoint:** `https://webhook.akinus21.com/webhook/akclip-build`
- **Webhook secret:** `4d82982b0a0010a706a40cf272f49c9ddfee93162a2c4b714eebc6ded10038f5`
- The CI pipeline posts build status and artifacts to the webhook for downstream deployment (e.g., Homebrew tap update).

## Homebrew Distribution
- **Tap repository:** `Akinus21/homebrew-tap`
- After a successful CI build, the binary is uploaded to the tap via a release script (handled by the webhook).  
- Users can install with:
  ```bash
  brew tap Akinus21/homebrew-tap
  brew install akclip
  ```

## Git Push Workflow
The `gh` CLI is not authenticated in the CI environment, so use SSH directly with the provided key.

```bash
cd /home/akinus/dockge-stacks/dev-stack/projects/akclip
git add -A
git commit -m "<description>"
GIT_SSH_COMMAND="ssh -i /home/akinus/.ssh/github -o StrictHostKeyChecking=no" \
git push origin main
```

**Always push** after verifying changes; the CI will automatically run the build and webhook steps.

## Secrets & Configuration Management
- **SSH key** for Git operations: `/home/akinus/.ssh/github`
- **Project‑specific secrets** (e.g., API tokens for the webhook) are stored in:
  ```
  /home/akinus/dockge-stacks/dev-stack/.secrets
  ```
  The repository contains a symlink `.secrets` pointing to that location. Do **not** commit real secrets; rely on the symlink.

## Documentation Updates
Whenever you add features, change CLI flags, or modify platform behavior:

1. **Update `README.md`** with:
   - New command‑line options.
   - Example usage for each OS.
   - Installation instructions (Cargo, Homebrew, Docker).
2. **Add or adjust** any relevant comments in `src/clipboard/*.rs` to keep the code self‑documenting.
3. **Run `cargo doc --open`** locally to verify generated docs (optional).

## Conventions & Best Practices
- **Error handling:** Return `Result<(), String>` from clipboard functions; propagate errors to the CLI with clear messages.
- **Logging:** Use `eprintln!` for user‑visible errors; avoid noisy debug prints in release builds.
- **Testing:** Add unit tests under `src/clipboard/tests.rs` using `#[cfg(test)]`. CI runs `cargo test --release`.
- **Formatting:** Run `cargo fmt` before committing.
- **Linting:** Enforce `cargo clippy` warnings as errors in CI (`cargo clippy -- -D warnings`).

---

*End of AGENTS.md*