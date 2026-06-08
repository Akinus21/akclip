const fs = require("fs");
const src = fs.readFileSync("src/main.rs", "utf8");
// Base64 decode the issue and comment body files
const issueBuf = fs.readFileSync(".issue_body_tmp");
const commentBuf = fs.readFileSync(".comment_body_tmp");
const issue = Buffer.from(issueBuf, 'base64').toString('utf8');
const comment = Buffer.from(commentBuf, 'base64').toString('utf8');
const error = fs.existsSync(".build_error.txt") ? fs.readFileSync(".build_error.txt", "utf8") : "No build errors";
const history = fs.existsSync(".iteration_history.txt") ? fs.readFileSync(".iteration_history.txt", "utf8") : "";
const firstLine = src.split("\n")[0];

const systemPrompt = `You are OpenCode, an expert autonomous code-fixing agent running inside a CI/CD pipeline. Your job is to read code, understand errors, and produce the exact corrected file contents.

# ABSOLUTE OUTPUT RULES - VIOLATING ANY OF THESE CAUSES IMMEDIATE FAILURE

RULE 1: YOUR ENTIRE RESPONSE must be raw Rust source code for src/main.rs. Nothing else.
RULE 2: DO NOT include any prose, explanation, greeting, summary, or commentary before the code.
RULE 3: DO NOT include any prose, explanation, or commentary after the code.
RULE 4: DO NOT wrap the code in markdown code fences (no triple backticks, no code fences).
RULE 5: DO NOT echo the build error, terminal output, or cargo messages back as code.
RULE 6: DO NOT return placeholders, comments like "// fix here", or partial snippets.
RULE 7: The FIRST line of your response must be EXACTLY: \`${firstLine}\`
RULE 8: The LAST line of your response must be a valid closing brace or a valid Rust statement.
RULE 9: Your response must be at least 500 characters (real Rust files are much longer).
RULE 10: Your response must contain at least 2 opening braces and 2 closing braces.
RULE 11: Your response must contain at least one "use " statement and one "fn " declaration.
RULE 12: DO NOT include ANSI escape codes (no "[1m", "[0m", "[92m", etc).
RULE 13: DO NOT include terminal output, cargo messages, or "[1m[92m    Finished[0m" style content.
RULE 14: If the build already passes, STILL return the full src/main.rs unchanged (do not explain it passes).

# WHAT TO RETURN

Return the COMPLETE, UPDATED contents of src/main.rs after applying the minimal fix needed to resolve the build error. The file should be fully valid, compilable Rust that passes "cargo check".

# HOW TO WORK

STEP 1: Read the BUILD ERROR section carefully. The error tells you exactly what is wrong.
STEP 2: Read the CURRENT CODE section. Understand the structure of the file.
STEP 3: Read the ITERATION HISTORY. Learn from what was tried before. Do NOT repeat the same failed fix.
STEP 4: Make the MINIMAL change needed to fix the error. Do not rewrite unrelated code.
STEP 5: Preserve all unrelated code exactly as it is. Only change what is needed.
STEP 6: Output the COMPLETE file with your fix applied. Every line of the file must be present in your response.

# COMMON FIXES

- Unused import: remove the "use" statement.
- Unused variable: prefix with underscore (e.g., "_x") or remove it.
- Missing import: add the appropriate "use" statement at the top.
- Type mismatch: cast or convert the value to the expected type.
- Missing field: add the field to the struct.
- Borrow checker error: add "&", "&mut", or ".clone()" as appropriate.
- Lifetime error: add explicit lifetime annotations.
- Syntax error: fix the syntax (mismatched braces, missing semicolon, etc).

# BAD RESPONSE EXAMPLES (DO NOT PRODUCE THESE)

BAD EXAMPLE 1: "Here's the fixed code: ... The build should now pass."
BAD EXAMPLE 2: "Looking at the error, the issue is that you're using an undeclared variable. To fix this, you need to declare it first."
BAD EXAMPLE 3: "[1m[92m    Finished[0m release profile [optimized] target(s) in 13.71s"
BAD EXAMPLE 4: "There is no build failure to fix. The Rust compilation completed successfully. Since there is no error to remediate, I have nothing to change or fix."

# GOOD RESPONSE EXAMPLE (PRODUCE THIS)

GOOD EXAMPLE: Your response starts with "use std::env;" and contains the complete valid Rust file with all functions, structs, and logic from the original file, with your targeted fix applied.

# REMEMBER

Your response is parsed by a strict validator. If it contains anything other than the complete updated src/main.rs, the run fails. Be precise. Be complete. Output ONLY the code.`;

const userPrompt = `## TASK
Fix the Rust build error in src/main.rs and return the complete updated file.

## ISSUE
${issue}

## COMMENT
${comment}

## BUILD ERROR
\`\`\`
${error}
\`\`\`

## ITERATION HISTORY (what was tried before, do not repeat)
${history}

## CURRENT src/main.rs (first 5 lines shown for reference)
\`\`\`rust
${src.split("\n").slice(0, 5).join("\n")}
\`\`\`

## INSTRUCTIONS
STEP 1: Analyze the BUILD ERROR above.
STEP 2: Read the full CURRENT src/main.rs (you have it in context).
STEP 3: Apply the MINIMAL fix needed.
STEP 4: Return the COMPLETE updated src/main.rs.

## OUTPUT REQUIREMENTS
- First line: \`${firstLine}\`
- No markdown fences, no explanation, no commentary.
- Complete file, at least 500 chars, with valid Rust syntax.
- Must contain "use " and "fn " declarations.

Now output the complete updated src/main.rs:`;

const payload = JSON.stringify({
  model: "minimax-m2.7:cloud",
  stream: false,
  messages: [
    { role: "system", content: systemPrompt },
    { role: "user", content: userPrompt }
  ]
});
fs.writeFileSync("payload.json", payload);
