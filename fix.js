const fs = require("fs");
const logs = process.env.RAW_COMPILER_LOGS;
const mainRs = fs.readFileSync("src/main.rs", "utf8");
let agentsMd = "";
let cargoToml = "";
let readmeMd = "";
let cicdWorkflow = "";
let issueWorkflow = "";
let releaseWorkflow = "";
let cleanupWorkflow = "";

try { agentsMd = fs.readFileSync(".github/agents/devops-sre.md", "utf8"); } catch (e) {}
try { cargoToml = fs.readFileSync("Cargo.toml", "utf8"); } catch (e) {}
try { readmeMd = fs.readFileSync("README.md", "utf8"); } catch (e) {}
try { cicdWorkflow = fs.readFileSync(".github/workflows/cicd-devops-loop.yml", "utf8"); } catch (e) {}
try { issueWorkflow = fs.readFileSync(".github/workflows/issue-resolver.yml", "utf8"); } catch (e) {}
try { releaseWorkflow = fs.readFileSync(".github/workflows/release-sync.yml", "utf8"); } catch (e) {}
try { cleanupWorkflow = fs.readFileSync(".github/workflows/issue-cleanup.yml", "utf8"); } catch (e) {}

const payload = {
  model: "minimax-m2.7:cloud",
  stream: false,
  messages: [
    {
      role: "system",
      content: "You are an expert Rust engineer and DevOps engineer. Your role is to diagnose and fix build failures autonomously.\n\n" +
        "## PROJECT CONTEXT\n\n" +
        "This is akclip - a Rust CLI tool that captures stdin to clipboard. It uses arboard for clipboard access.\n\n" +
        "## PROJECT FILES\n\n" +
        "[Cargo.toml]:\n" + cargoToml + "\n\n" +
        "[src/main.rs]:\n" + mainRs + "\n\n" +
        "## WORKFLOW FILES\n\n" +
        "[cicd-devops-loop.yml]:\n" + cicdWorkflow + "\n\n" +
        "[issue-resolver.yml]:\n" + issueWorkflow + "\n\n" +
        "[release-sync.yml]:\n" + releaseWorkflow + "\n\n" +
        "[issue-cleanup.yml]:\n" + cleanupWorkflow + "\n\n" +
        "## AGENTS.md (Agent Guidelines):\n" + agentsMd + "\n\n" +
        "## BUILD ERROR\n\n" + logs + "\n\n" +
        "## YOUR TASK\n\n" +
        "1. DIAGNOSE the build failure from the error logs\n" +
        "2. FIX the appropriate file(s):\n" +
        "   - Rust compilation errors -> fix src/main.rs\n" +
        "   - CI/CD workflow errors -> fix relevant workflow file\n\n" +
        "## IMPORTANT RULES\n\n" +
        "1. Only return code for the file that NEEDS fixing\n" +
        "2. Rust code: start with 'use std::env;' or 'fn main'\n" +
        "3. Workflow code: start with 'name:' and contain 'jobs:'\n" +
        "4. Do NOT use markdown code blocks (no ```)\n" +
        "5. Make sure Rust code compiles before returning\n\n" +
        "Analyze the build error and respond with ONLY the raw file content that fixes it."
    },
    {
      role: "user",
      content: "[BUILD ERROR LOGS]:\n" + logs
    }
  ]
};
fs.writeFileSync("payload.json", JSON.stringify(payload));
