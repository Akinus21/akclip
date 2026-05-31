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
          const delta = parsedChunk.choices[0].delta.content || "";
          combinedContent += delta;
        } catch (err) {}
      }
    }
  } else {
    const data = JSON.parse(rawInput);
    combinedContent = data.choices[0].message.content || "";
  }

  if (!combinedContent) {
    console.error("Empty content stream received.");
    process.exit(1);
  }

  let cleaned = combinedContent.trim();
  // Remove markdown code blocks
  if (cleaned.startsWith("```")) {
    cleaned = cleaned.replace(/^```[a-zA-Z]*\n?/, "");
    cleaned = cleaned.replace(/```$/, "");
  }

  // Detect which file to fix based on content
  if (cleaned.includes("name:") && cleaned.includes("on:") && cleaned.includes("jobs:")) {
    console.log("DETECTED: Workflow file needs fixing");
    fs.writeFileSync(".github/workflows/cicd-devops-loop.yml", cleaned);
    fs.writeFileSync("detected_file.txt", "workflow");
  } else if (cleaned.includes("fn main") || cleaned.includes("use std")) {
    console.log("DETECTED: Rust source file needs fixing");
    fs.writeFileSync("src/main.rs", cleaned);
    fs.writeFileSync("detected_file.txt", "rust");
  } else {
    console.log("DETECTED: Defaulting to Rust source");
    fs.writeFileSync("src/main.rs", cleaned);
    fs.writeFileSync("detected_file.txt", "rust");
  }
} catch (e) {
  console.error("Failed processing response:", e.message);
  process.exit(1);
}
