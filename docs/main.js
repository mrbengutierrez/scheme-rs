import init, { eval_racket } from "./pkg/racket_rs.js";

async function runWasm() {
    await init();
    console.log("✅ WASM initialized");

    document.getElementById("eval-button").addEventListener("click", () => {
        const input = document.getElementById("input").value;
        const output = eval_racket(input);
        document.getElementById("output").textContent = output;
    });
}

runWasm();
