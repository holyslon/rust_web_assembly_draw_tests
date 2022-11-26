import * as wasm from "test";
import {
    memory
} from "test/test_bg"

wasm.start()

const canvas = document.getElementById('canvas');
const ctx = canvas.getContext('bitmaprenderer');


const board = wasm.Board.new(canvas.width, canvas.height);


function load() {
    const buffer = new Uint8ClampedArray(memory.buffer, board.buffer_pointer(), board.buffer_size());
    return new ImageData(buffer, canvas.width, canvas.height, {})
}

//const data = load()

function add_line(id, r, g, b, a, sx, sy, ex, ey) {
    return {
        id: id,
        fill: {
            red: r,
            green: g,
            blue: b,
            alpha: a,
        },
        from: {
            x: sx,
            y: sy
        },
        to: {
            x: ex,
            y: ey
        }
    }
}

function change_line_end(id, ex, ey) {
    return {
        id: id,
        to: {
            x: ex,
            y: ey
        }
    }
}

function batch(add, change) {
    return JSON.stringify({
        add: add,
        change: change,
        remove: []
    })
}

function loop() {
    board.do_draw();
    const data = load()
    createImageBitmap(data, 0, 0, canvas.width, canvas.height).
    then(bitmap => {
        ctx.transferFromImageBitmap(bitmap)
    }, err => console.error(err))
    requestAnimationFrame(loop)
}
loop()

function drawLine() {
    const id = "first line"
    board.batch(batch([add_line(id, 255, 255, 2, 255, canvas.width / 2, canvas.height / 2, 100, 50)], []))
    return id;
}
const lineId = drawLine();

var stepX = 0;
var stepY = 0;

function changeLine() {
    board.batch(batch([], [change_line_end(lineId, stepX, stepY)]))
    if (stepY == 0) {
        if (stepX < canvas.width - 1) {
            stepX += 1
            return
        }
    }
    if (stepX == canvas.width - 1) {
        if (stepY < canvas.height - 1) {
            stepY += 1
            return
        }
    }
    if (stepY == canvas.height - 1) {
        if (stepX > 0) {
            stepX -= 1;
            return
        }
    }
    if (stepX == 0) {
        if (stepY > 0) {
            stepY -= 1;
            return
        }
    }

}

setInterval(changeLine, 10)