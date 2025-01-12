import init, {calculate_parabola} from "../pkg/meth_leb.js";

async function main() {

    await init();

    const form = document.getElementById("form");
    const canvas = document.getElementById("canvas");
    const context = canvas.getContext("2d");
    const dropdown = document.getElementById("examples");

    function drawAxes() {
        context.beginPath();
        context.strokeStyle = "gray";
        context.lineWidth = 1;

        context.moveTo(0, 300);
        context.lineTo(800, 300);

        context.moveTo(400, 0);
        context.lineTo(400, 600);

        context.stroke();
        context.closePath();
    }

    drawAxes();

    function draw(a, b, c, x_min, x_max, steps) {

        const points = calculate_parabola(a, b, c, x_min, x_max, steps);

        context.clearRect(0, 0, canvas.width, canvas.height);
        drawAxes();
        context.beginPath();
        context.moveTo((points[0] - x_min) * 50, 300 - points[1] * 50);

        for (let i = 2; i < points.length; i += 2) {
            const x = (points[i] - x_min) * 50;
            const y = 300 - points[i + 1] * 50;
            context.lineTo(x, y);
        }

        context.stroke();
    }

    dropdown.addEventListener("change", (event) => {
        const [a, b, c, x_min, x_max, steps] = event.target.value.split(",").map(Number);
        draw(a, b, c, x_min, x_max, steps);
    });

    form.addEventListener("submit", (event) => {
        event.preventDefault();

        const a = parseFloat(document.getElementById("a").value);
        const b = parseFloat(document.getElementById("b").value);
        const c = parseFloat(document.getElementById("c").value);
        const x_min = parseFloat(document.getElementById("x_min").value);
        const x_max = parseFloat(document.getElementById("x_max").value);
        const steps = parseInt(document.getElementById("steps").value);

        draw(a, b, c, x_min, x_max, steps);
    });
}

main().catch((error) => console.error("Error on loading wasm module.", error));
