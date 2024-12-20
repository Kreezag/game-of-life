import { Universe } from "game-of-life";
import { memory } from "game-of-life/game_of_life_bg";


const CELL_SIZE = 2; // px
const GRID_COLOR = "#EEE";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#2511e0";

const UNIVERSE_WIDTH = 500;
const UNIVERSE_HEIGHT = 250

// Construct the universe, and get its width and height.
const universe = Universe.new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT, 0.97);
const width = UNIVERSE_WIDTH //universe.width();
const height = UNIVERSE_HEIGHT//universe.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

function drawGrid () {
    const ctx = canvas.getContext('2d');

    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

function drawCells () {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);
    const bitIsSet = (n, arr) => {
        const byte = Math.floor(n / 8);
        const mask = 1 << (n % 8);
        return (arr[byte] & mask) === mask;
    };

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            // This is updated!
            ctx.fillStyle = bitIsSet(idx, cells)
                ? ALIVE_COLOR
                : DEAD_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

function getIndex (row, column) {
    return row * width + column;
}