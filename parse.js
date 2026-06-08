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

  const firstLine = cleaned.split('\n')[0].trim();
  const validStartPatterns = ['use ', 'fn ', 'struct ', 'impl ', 'enum ', 'mod ', 'pub ', 'const ', 'let ', 'static ', 'trait ', 'type ', 'name:', 'on:', 'jobs:', 'steps:', 'env:', 'run:', 'uses:', 'if:', 'with:', 'permissions:', '<', '<?', '{', '['];
  const isLikelyCode = validStartPatterns.some(p => firstLine.startsWith(p));
  const explanationIndicators = ['Looking at', 'The issue', 'The problem', 'I think', 'I need to', 'Here is', 'This is', 'The code', 'error:', 'Error:'];
  const hasExplanation = explanationIndicators.some(e => firstLine.startsWith(e));

  if (!isLikelyCode || hasExplanation) {
    console.error("INVALID RESPONSE: AI returned explanatory text instead of code");
    fs.writeFileSync("VALIDATION_FAILED", "true");
    process.exit(0);
  }

  if (cleaned.startsWith("```")) {
    cleaned = cleaned.replace(/^```[a-zA-Z]*\n?/, "");
    cleaned = cleaned.replace(/```$/, "");
  }

  if (cleaned.includes("name:") && cleaned.includes("on:") && cleaned.includes("jobs:")) {
    console.log("DETECTED: Workflow file");
    fs.writeFileSync(".github/workflows/cicd-devops-loop.yml", cleaned);
    fs.writeFileSync("detected_file.txt", "workflow");
  } else if (cleaned.includes("fn main") || cleaned.includes("use std")) {
    console.log("DETECTED: Rust source");
    fs.writeFileSync("src/main.rs", cleaned);
    fs.writeFileSync("detected_file.txt", "rust");
  } else {
    console.log("DETECTED: Defaulting to Rust");
    fs.writeFileSync("src/main.rs", cleaned);
    fs.writeFileSync("detected_file.txt", "rust");
  }
} catch (e) {
  console.error("Parse error:", e.message);
  process.exit(1);
}
