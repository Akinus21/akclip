const fs = require("fs");
const issueBody = process.env.ISSUE_BODY || "";
const commentBody = process.env.COMMENT_BODY || "";
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

const userMessage = "[ISSUE DESCRIPTION]:\n" + issueBody + "\n\n[COMMENT FROM USER]:\n" + commentBody + "\n\nThe user has reported an issue with akclip. Please diagnose and fix it using the project context provided above.";

const systemPrompt = "You are an expert Rust engineer and DevOps engineer. Your role is to diagnose and fix issues autonomously.\n\n" +
  "## PROJECT CONTEXT\n\n" +
  "This is akclip - a Rust CLI tool that captures stdin to clipboard. It uses arboard for clipboard access.\n\n" +
  "## PROJECT FILES\n\n" +
  "[Cargo.toml]:\n" + cargoToml + "\n\n" +
  "[src/main.rs]:\n" + mainRs + "\n\n" +
  "[README.md]:\n" + readmeMd + "\n\n" +
  "## WORKFLOW FILES\n\n" +
  "[cicd-devops-loop.yml]:\n" + cicdWorkflow + "\n\n" +
  "[issue-resolver.yml]:\n" + issueWorkflow + "\n\n" +
  "[release-sync.yml]:\n" + releaseWorkflow + "\n\n" +
  "[issue-cleanup.yml]:\n" + cleanupWorkflow + "\n\n" +
  "## AGENTS.md (Agent Guidelines):\n" + agentsMd + "\n\n" +
  "## YOUR TASK\n\n" +
  "1. DIAGNOSE the issue based on user's description\n" +
  "2. FIX the appropriate file(s):\n" +
  "   - Rust compilation/runtime errors -> fix src/main.rs\n" +
  "   - CI/CD, GitHub Actions, Homebrew, releases, checksums -> fix relevant workflow file\n" +
  "   - Logic errors -> fix src/main.rs\n\n" +
  "## IMPORTANT RULES\n\n" +
  "1. Your ENTIRE response must be ONLY raw code - no backticks, no markdown, no text\n" +
  "2. For Rust files: start directly with 'use std::env;' or 'fn main' - nothing else\n" +
  "3. For Workflow files: start directly with 'name:' - nothing else\n" +
  "4. Do NOT explain what you changed or why - ONLY output the fixed file content\n" +
  "5. If you include any text, commentary, or formatting, the fix will fail\n\n" +
  "Analyze the issue and respond with ONLY the raw file content that fixes it. Your response must be ONLY the raw code - NO backticks, NO markdown, NO explanations, NO analysis - just pure raw code starting from the first character of the file.";

const payload = {
  model: "minimax-m2.7:cloud",
  stream: false,
  messages: [
    { role: "system", content: systemPrompt },
    { role: "user", content: userMessage }
  ]
};
fs.writeFileSync("payload.json", JSON.stringify(payload));
