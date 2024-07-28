import init, * as wasm from "./wasm.js"

const WIDTH = 64;
const HEIGHT = 32;
const SCALE = 12;
const TICKS_PER_FRAME = 5;
let anim_frame = 0;

const ROMS = [
    "15PUZZLE",
    "BLINKY",
    "BLITZ",
    "BRIX",
    "CONNECT4",
    "GUESS",
    "HIDDEN",
    "INVADERS",
    "KALEID",
    "MAZE",
    "MERLIN",
    "MISSILE",
    "PONG",
    "PONG2",
    "PUZZLE",
    "SYZYGY",
    "TANK",
    "TETRIS",
    "TICTAC",
    "UFO",
    "VBRIX",
    "VERS",
    "WIPEOFF"
];

const canvas = document.getElementById("canvas");
canvas.width = WIDTH * SCALE;
canvas.height = HEIGHT * SCALE;

const ctx = canvas.getContext("2d");
ctx.fillStyle = "black";
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

const roms = document.getElementById("roms");
const startButton = document.getElementById("start");
const stopButton = document.getElementById("stop");
const restartButton = document.getElementById("restart");

ROMS.forEach(rom => {
    let option = document.createElement("option");
    option.value = rom;
    option.text = rom;
    roms.appendChild(option);
});

async function run() {
    await init();

    let chip8 = new wasm.EmuWasm();
    let currentRom;

    function loadRom(rom) {
        fetch(`roms/${rom}`)
            .then(response => {
                // Check if the request was successful
                if (!response.ok) {
                    throw new Error(`Failed to load ROM: ${response.status} ${response.statusText}`);
                }
                return response.arrayBuffer();
            })
            .then(buffer => {
                // write the ROM to memory
                const rom = new Uint8Array(buffer);
                currentRom = rom;

                chip8.reset();
                chip8.load_game(rom);
                mainloop(chip8);
            })
            .catch(error => {
                // Handle any errors that occurred while fetching the ROM
                console.error(`Failed to load ROM: ${error}`);
            });
    }

    // Load PUZZLE as default rom
    loadRom(ROMS[[0]]);

    document.addEventListener("keydown", (evt) => {
        chip8.keypress(evt, true);
    });

    document.addEventListener("keyup", (evt) => {
        chip8.keypress(evt, false);
    });

    startButton.addEventListener("click", () => {
        anim_frame = window.requestAnimationFrame(() => mainloop(chip8));
    });

    stopButton.addEventListener("click", () => {
        window.cancelAnimationFrame(anim_frame);
    });

    restartButton.addEventListener("click", () => {
        window.cancelAnimationFrame(anim_frame)
        chip8.reset();
        chip8.load_game(currentRom);
        mainloop(chip8);
    })

    roms.addEventListener("change", (evt) => {
        // Stop previous game from rendering, if one exists if (anim_frame != 0) {
        window.cancelAnimationFrame(anim_frame);

        let selectedValue = evt.target.value;

        loadRom(selectedValue);
    }, false);

    function mainloop(chip8) {
        // Only draw every few ticks
        for (let i = 0; i<  TICKS_PER_FRAME; i++) {
            chip8.tick();
        }
        chip8.tick_timers();

        // Clear the canvas before drawing
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);

        // Set the draw color back to white before we render our frame;
        ctx.fillStyle = "white";
        chip8.draw_screen(SCALE);

        anim_frame = window.requestAnimationFrame(() => {
           mainloop(chip8);
        });
    }
}

run().catch(console.error);