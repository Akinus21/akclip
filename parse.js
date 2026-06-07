const fs = require("fs");
try {
  const rawInput = fs.readFileSync("response.txt", "utf8").trim();
  let combinedContent = "";

  if (rawInput.startsWith("data:")) {
    const lines = rawInput.split("\n");
    for (let line of lines) {
      line = line.trim();
      if (!line || line === "data: [DONE]") continue;
      if (line.startsWith("data:")) {
        try {
          const parsedChunk = JSON.parse(line.replace(/^data:\s*/, ""));
          if (parsedChunk.choices && parsedChunk.choices[0] && parsedChunk.choices[0].delta) {
            const delta = parsedChunk.choices[0].delta.content || "";
            combinedContent += delta;
          }
        } catch (err) {}
      }
    }
  } else {
    const data = JSON.parse(rawInput);
    if (data.choices && data.choices[0] && data.choices[0].message) {
      combinedContent = data.choices[0].message.content || "";
    } else if (data.error) {
      console.error("AI returned error:", data.error);
      process.exit(1);
    }
  }

  if (!combinedContent) {
    console.error("Empty content stream received.");
    process.exit(1);
  }

  let cleaned = combinedContent.trim();

  // VALIDATION: Ensure response is valid code, not explanation text
  const firstLine = cleaned.split('\n')[0].trim();

  // Acceptable starting patterns for various file types
  const validStartPatterns = [
    'use ', 'fn ', 'struct ', 'impl ', 'enum ', 'mod ', 'pub ', 'const ', 'let ', 'static ', 'trait ', 'type ',  // Rust
    'name:', 'on:', 'jobs:', 'steps:', 'env:', 'run:', 'uses:', 'if:', 'with:', 'permissions:',  // YAML/workflow
    '<', '<?',  // XML/HTML
    '{', '['   // JSON
  ];
  const isLikelyCode = validStartPatterns.some(p => firstLine.startsWith(p));

  // Check if response contains explanation indicators
  const explanationIndicators = ['Looking at', 'The issue', 'The problem', 'I think', 'I need to', 'Here is', 'This is', 'The code', 'error:', 'Error:'];
  const hasExplanation = explanationIndicators.some(e => firstLine.startsWith(e));

  if (!isLikelyCode || hasExplanation) {
    console.error("INVALID RESPONSE: AI returned explanatory text instead of code");
    console.error("First line:", firstLine.substring(0, 100));
    console.error("Writing sentinel to trigger retry...");
    fs.writeFileSync("VALIDATION_FAILED", "true");
    process.exit(0);
  }

  if (cleaned.startsWith("```")) {
    cleaned = cleaned.replace(/^```[a-zA-Z]*\n?/, "");
    cleaned = cleaned.replace(/```$/, "");
  }

  // STRICT detection: must start with expected patterns
  if (firstLine.startsWith('name:') && cleaned.includes("on:") && cleaned.includes("jobs:")) {
    console.log("DETECTED: Workflow file needs fixing");
    fs.writeFileSync(".github/workflows/cicd-devops-loop.yml", cleaned);
    fs.writeFileSync("detected_file.txt", "workflow");
  } else if (firstLine.startsWith('use ') || firstLine.startsWith('fn ') || firstLine.startsWith('struct ') || firstLine.startsWith('impl ') || firstLine.startsWith('const ') || firstLine.startsWith('let ') || firstLine.startsWith('pub ') || firstLine.startsWith('mod ') || firstLine.startsWith('static ') || firstLine.startsWith('enum ') || firstLine.startsWith('trait ') || firstLine.startsWith('type ')) {
    console.log("DETECTED: Rust source file needs fixing");
    fs.writeFileSync("src/main.rs", cleaned);
    fs.writeFileSync("detected_file.txt", "rust");
  } else {
    console.error("INVALID RESPONSE: Cannot determine file type from response");
    console.error("First line:", firstLine.substring(0, 100));
    console.error("Writing sentinel to trigger retry...");
    fs.writeFileSync("VALIDATION_FAILED", "true");
    process.exit(0);
  }
} catch (e) {
  console.error("Failed processing response:", e.message);
  process.exit(1);
}
