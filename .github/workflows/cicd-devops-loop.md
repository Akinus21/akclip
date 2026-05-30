---
name: Autonomous DevOps Loop
on:
  workflow_run:
    workflows: ["Build and Test"]
    types: [completed]
    branches:
      - main

permissions:
  contents: read
  issues: read
  pull-requests: read

safe-outputs:
  create-pull-request:
    title-prefix: "[brew-bump] "
    labels: [dependencies, homebrew]
    allowed-repos: ["Akinus21/homebrew-tap"]

engine:
  id: crush
  model: "openai/akai-net"
  env:
    OPENAI_BASE_URL: "https://ollama.akinus21.com"
    OPENAI_API_KEY: "ollama"
---

You are an autonomous Rust SRE and Release Engineer. Monitor the incoming compilation state:

- IF THE BUILD FAILED: Download the broken build logs, analyze the Rust compiler error, make the necessary corrections to `src/`, and re-test until clean.
- IF THE BUILD SUCCEEDED: Manage the `Akinus21/homebrew-tap` repository. Check if `Formula/akclip.rb` exists; if it does not, create a new one using standard Cargo formula blocks. Compute the `sha256` of the release asset, update the version string, and use your `create_pull_request` tool targeting the `Akinus21/homebrew-tap` repository to deploy the change.
