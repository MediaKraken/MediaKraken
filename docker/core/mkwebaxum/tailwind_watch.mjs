#!/usr/bin/env node

// ==== DYNAMIC IMPORTS ====
import { exec } from "child_process";
import chokidar from "chokidar";
import { globSync } from "glob";
// ==== CONFIGURATION ====
const contentGlobs = [
  "./templates/**/*.html",
  "./src/**/*.rs"
];
const inputCSS = "./tailwind_input.css";
const outputCSS = "./static/css/base_webapp.css";
const tailwindCommand = `npx tailwindcss -i ${inputCSS} -o ${outputCSS}`;

// ==== FUNCTION TO LIST MATCHED FILES ====
function listFiles() {
  const files = contentGlobs
    .map(pattern => globSync(pattern))
    .flat();
  console.log("\nTailwind scanning files:");
  files.forEach(f => console.log("-", f));
  return files;
}

// ==== FUNCTION TO RUN TAILWIND ====
function buildTailwind() {
  console.log("\nRunning Tailwind CLI...");
  const child = exec(tailwindCommand);

  child.stdout.on("data", data => process.stdout.write(data));
  child.stderr.on("data", data => process.stderr.write(data));

  child.on("exit", code => {
    console.log(`Tailwind CLI exited with code ${code}`);
  });
}

// ==== INITIAL RUN ====
listFiles();
buildTailwind();

// ==== WATCH MODE ====
const watcher = chokidar.watch(contentGlobs, {
  ignoreInitial: true
});

watcher.on("all", (event, path) => {
  console.log(`\nDetected change (${event}): ${path}`);
  listFiles();
  buildTailwind();
});
