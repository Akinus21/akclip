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
  if (cleaned.startsWith("```")) {
    cleaned = cleaned.replace(/^```[a-zA-Z]*\n?/, "");
    cleaned = cleaned.replace(/```$/, "");
  }

  if (cleaned.includes("name:") && cleaned.includes("on:") && cleaned.includes("jobs:")) {
    fs.writeFileSync(".github/workflows/cicd-devops-loop.yml", cleaned);
    console.log("Workflow file updated!");
  } else if (cleaned.includes("fn main") || cleaned.includes("use std")) {
    fs.writeFileSync("src/main.rs", cleaned);
    console.log("Rust source file updated!");
  } else {
    fs.writeFileSync("src/main.rs", cleaned);
    console.log("Defaulting to Rust source!");
  }
} catch (e) {
  console.error("Failed processing response:", e.message);
  process.exit(1);
}
