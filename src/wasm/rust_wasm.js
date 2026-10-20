/* @ts-self-types="./rust_wasm.d.ts" */
import * as wasm from "./rust_wasm_bg.wasm";
import { __wbg_set_wasm } from "./rust_wasm_bg.js";

__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    Engine, MaterialId, init_console_logger, wasmMemory
} from "./rust_wasm_bg.js";
export { wasm as __wasm }
