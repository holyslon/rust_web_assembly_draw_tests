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

const data = load()

function loop() {
    board.do_draw();
    createImageBitmap(data, 0, 0, canvas.width, canvas.height).
    then(bitmap => {
        ctx.transferFromImageBitmap(bitmap)
    }, err => console.error(err))
    requestAnimationFrame(loop)
}
loop()