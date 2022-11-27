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
    createImageBitmap(data, 0, 0, canvas.width, canvas.height).
    then(bitmap => {
        ctx.transferFromImageBitmap(bitmap)
    }, err => console.error(err))
    requestAnimationFrame(loop)
}
loop()

class Line {
    constructor(id, r, g, b, x, y) {
        this.id = `${id}`
        this.r = r
        this.g = g
        this.b = b
        this.x = x
        this.y = y
    }

    add() {
        return add_line(this.id, this.r, this.g, this.b, 255, canvas.width / 2, canvas.height / 2, this.x, this.y)
    }

    change() {
        if (this.y == 0) {
            if (this.x < canvas.width - 1) {
                this.x += 1
                return change_line_end(this.id, this.x, this.y)
            }
        }
        if (this.x == canvas.width - 1) {
            if (this.y < canvas.height - 1) {
                this.y += 1
                return change_line_end(this.id, this.x, this.y)
            }
        }
        if (this.y == canvas.height - 1) {
            if (this.x > 0) {
                this.x -= 1;
                return change_line_end(this.id, this.x, this.y)
            }
        }
        if (this.x == 0) {
            if (this.y > 0) {
                this.y -= 1;
                return change_line_end(this.id, this.x, this.y)
            }
        }
        return change_line_end(this.id, this.x, this.y)
    }
}

const cell = 10

const x_grid = [...Array(canvas.width / cell).keys()].map(function (index) {
    return add_line(`x-grid-${index}`, 0, 0, 0, 100, index * cell, 0, index * cell, canvas.height)
})
const y_grid = [...Array(canvas.height / cell).keys()].map(function (index) {
    return add_line(`y-grid-${index}`, 0, 0, 0, 100, 0, index * cell, canvas.width, index * cell)
})


const lines = [...Array(100).keys()].map(function (index) {
    return new Line(`z-${index}`, index % 255, (index * 2) % 255, (index * 3) % 255, index % canvas.width, 0)
})

const lines_add = lines.map(function (line) {
    return line.add()
})

const lines_add_batch = batch(x_grid.concat(y_grid).concat(lines_add), [])

board.batch(lines_add_batch)


function changeLine() {
    try {
        if (lines.length == 0) {
            return
        }
        const changes = lines.map(function (line) {
            return line.change()
        })
        if (changes == null) {
            return
        }
        const batch_s = batch([], changes)
        board.batch(batch_s)
    } catch (err) {
        console.log("Error", err)
    }

}

setInterval(changeLine, 10)