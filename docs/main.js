import init, { eval_racket } from "./pkg/racket_rs.js";

const evalBtn = document.querySelector("button");

evalBtn.disabled = true;

async function runWasm() {
    await init();

    console.log("WASM loaded");
    evalBtn.disabled = false;

    evalBtn.addEventListener("click", () => {
        const input = document.getElementById("input").value;
        const output = eval_racket(input);
        document.getElementById("output").textContent = output;
    });
}

runWasm();
