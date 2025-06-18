import init, { EvalContext } from "./pkg/racket_rs.js";

async function runWasm() {
    await init();
    console.log("✅ WASM initialized");

    const ctx = EvalContext.new(); // persistent environment
    const input = document.getElementById("input");
    const output = document.getElementById("output");
    const button = document.getElementById("eval-button");

    function evaluate() {
        const source = input.value;
        try {
            const result = ctx.eval(source);
            output.textContent += `\nracket> ${source}\n=> ${result}\n`;
        } catch (e) {
            output.textContent += `\nracket> ${source}\n!! ${e}\n`;
        }
        input.value = "";
    }

    button.addEventListener("click", evaluate);

    // Allow Shift+Enter to evaluate
    input.addEventListener("keydown", (e) => {
        if (e.key === "Enter" && e.shiftKey) {
            e.preventDefault();
            evaluate();
        }
    });

    output.textContent = "Welcome to 🦀 Racket-rs Playground\nPress Shift+Enter to evaluate\n";
}

runWasm();
