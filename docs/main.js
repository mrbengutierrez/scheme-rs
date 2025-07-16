import init, { EvalContext } from "./pkg/scheme_rs.js";

async function runWasm() {
    await init();
    console.log("WASM initialized");

    const ctx = new EvalContext();

    const inputEl = document.getElementById("input");
    const outputEl = document.getElementById("output");

    inputEl.addEventListener("keydown", (e) => {
        if (e.key === "Enter" && !e.shiftKey) {
            e.preventDefault();
            const line = inputEl.value.trim();
            inputEl.value = "";

            const result = ctx.eval_line(line);
            appendOutput(`scheme-rs> ${line}`);
            appendOutput(result);

            // If user typed "exit", disable further input
            if (line === "exit" || line === "quit") {
                inputEl.disabled = true;
                inputEl.placeholder = "Session ended.";
            }
        }
    });

    function appendOutput(text) {
        outputEl.textContent += text + "\n";
        outputEl.scrollTop = outputEl.scrollHeight;
    }

    appendOutput("🦀 Welcome to the Scheme REPL (WASM Edition)");
    appendOutput("💀 Type `exit` or `quit` when your existential dread sets in.");
    appendOutput("");
}

runWasm();
