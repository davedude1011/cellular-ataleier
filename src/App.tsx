import {
  createSignal,
  For,
  onMount,
  type Accessor,
  type Setter,
} from "solid-js";

import "./canvas.css";
import type { Engine } from "./wasm";

// must be a power of 2
const DOWNSCALE: number = 4;

export default function App() {
  let canvas: HTMLCanvasElement | undefined;
  let ctx: CanvasRenderingContext2D | undefined;

  let width: number;
  let height: number;

  let engine_width: number;
  let engine_height: number;

  let engine: Engine;
  let memory: WebAssembly.Memory;
  let view_len: number;

  const init_canvas = () => {
    if (!canvas) return;

    ctx = canvas.getContext("2d") ?? undefined;
    if (!ctx) return;

    let screen_width = window.innerWidth;
    let screen_height = window.innerHeight;

    width = 2 ** Math.floor(Math.log2(screen_width));
    height = 2 ** Math.floor(Math.log2(screen_height));

    engine_width = width / DOWNSCALE;
    engine_height = height / DOWNSCALE;

    canvas.width = width;
    canvas.height = height;

    ctx.imageSmoothingEnabled = false;
  };

  onMount(async () => {
    const { Engine, wasmMemory } = await import("./wasm");

    init_canvas();

    engine = Engine.new(engine_width, engine_height);
    memory = wasmMemory();

    view_len = engine_width * engine_height * 4;

    if (!USING_STEPPER) requestAnimationFrame(render);
  });

  const render = () => {
    const ptr = engine.render();

    const view = new Uint8ClampedArray(memory.buffer, ptr, view_len);
    const image = new ImageData(view, engine_width, engine_height);

    createImageBitmap(image).then((bitmap) => {
      if (!ctx) return;

      ctx.drawImage(bitmap, 0, 0, width, height);

      engine.step();
      if (!USING_STEPPER) requestAnimationFrame(render);
    });
  };

  const USING_STEPPER = false;

  let is_mouse_down = false;
  const [material, set_material] = createSignal<number>(2);

  const mouse_move = (e: MouseEvent) => {
    const pos_x = Math.floor(e.offsetX / DOWNSCALE);
    const pos_y = Math.floor(e.offsetY / DOWNSCALE);

    engine.set_cursor_position(pos_x, pos_y);
    if (is_mouse_down) draw();
  };

  const draw = () => engine.draw_at_mouse(material(), 15);

  const mouse_down = () => {
    draw();

    is_mouse_down = true;
  };

  const mouse_up = () => {
    is_mouse_down = false;
  };

  return (
    <div>
      <MaterialBar {...{ material, set_material }} />

      <canvas
        ref={canvas}
        onMouseMove={mouse_move}
        onMouseDown={mouse_down}
        onMouseUp={mouse_up}
      />
      <button onclick={() => USING_STEPPER && render()}>Step</button>
    </div>
  );
}

function MaterialBar({
  material,
  set_material,
}: {
  material: Accessor<number>;
  set_material: Setter<number>;
}) {
  const materials = [
    "Empty",
    "Barrier",

    "Sand",
    "Wet Sand",

    "Water",
    "Oil",

    "Steam",
    "Smog",

    "Fire",

    /*
    "AssetPlantGreen1",
    "AssetPlantGreen2",
    "AssetPlantGreen3",
    "AssetPlantGreen4",

    "AssetPlantYellow1",

    "AssetPlantBrown1",
    "AssetPlantBrown2",

    "DebugRay",

    "UserCursor",
    */
  ];

  return (
    <div>
      <For each={materials}>
        {(name, index) => (
          <button
            onclick={() => set_material(index())}
            style={{
              border: index() == material() ? "2px solid black" : undefined,
            }}
          >
            {name}
          </button>
        )}
      </For>
    </div>
  );
}
