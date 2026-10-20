/* tslint:disable */
/* eslint-disable */

export class Engine {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    draw_at_mouse(material: MaterialId, radius: number): void;
    draw(x: number, y: number, material: MaterialId): void;
    log_material_buffer(): void;
    static new(width: number, height: number): Engine;
    render(): number;
    set_cursor_position(cursor_x: number, cursor_y: number): void;
    step(): void;
}

export enum MaterialId {
    Empty = 0,
    Barrier = 1,
    Sand = 2,
    WetSand = 3,
    Water = 4,
    Oil = 5,
    Steam = 6,
    Smog = 7,
    Fire = 8,
    AssetPlantGreen1 = 9,
    AssetPlantGreen2 = 10,
    AssetPlantGreen3 = 11,
    AssetPlantGreen4 = 12,
    AssetPlantYellow1 = 13,
    AssetPlantBrown1 = 14,
    AssetPlantBrown2 = 15,
    DebugRay = 16,
    UserCursor = 17,
    Count = 18,
}

export function init_console_logger(): void;

export function wasmMemory(): any;
