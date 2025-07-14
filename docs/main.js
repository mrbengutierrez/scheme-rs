import init, { eval_scheme } from "../pkg/scheme_rs.js";

async function runWasm() {
    await init();

    window.run = function () {
        const input = document.getElementById("input").value;
        const output = eval_scheme(input);
        document.getElementById("output").textContent = output;
    };
}

runWasm();
