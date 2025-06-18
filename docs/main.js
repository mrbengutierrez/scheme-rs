import init, { eval_racket } from "../pkg/racket_rs.js";

async function runWasm() {
    await init();

    window.run = function () {
        const input = document.getElementById("input").value;
        const output = eval_racket(input);
        document.getElementById("output").textContent = output;
    };
}

runWasm();
