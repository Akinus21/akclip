---
name: Interactive Bug Fixer
on:
  issue_comment:
    types: [created]

permissions:
  contents: read
  issues: read
  pull-requests: read

safe-outputs:
  create-pull-request:
    title-prefix: "[ai-fix] "
    labels: [automation, bugfix]

engine:
  id: copilot
  env:
    COPILOT_PROVIDER_BASE_URL: "https://ollama.akinus21.com"
    COPILOT_PROVIDER_API_KEY: "ollama"
    COPILOT_MODEL: "akai-net"
---

You are an autonomous Rust Software Engineer. When a user types a comment containing "/fix":
1. Parse the issue descriptions and error reports.
2. Examine the Rust files in `src/`.
3. Apply code fixes to resolve bugs and run `cargo check` until it compiles cleanly.
4. Output your modifications using the `create_pull_request` tool to submit a PR back to the repository.
