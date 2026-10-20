//#region exports

export class Engine {
    constructor() {
        throw new Error('cannot invoke `new` directly');
    }
    static __wrap(ptr) {
        const obj = Object.create(Engine.prototype);
        obj.__wbg_ptr = ptr;
        EngineFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        EngineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_engine_free(ptr, 0);
    }
    /**
     * @param {MaterialId} material
     * @param {number} radius
     */
    draw_at_mouse(material, radius) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(material);
        _assertNum(radius);
        wasm.engine_draw_at_mouse(this.__wbg_ptr, material, radius);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {MaterialId} material
     */
    draw(x, y, material) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(x);
        _assertNum(y);
        _assertNum(material);
        wasm.engine_draw(this.__wbg_ptr, x, y, material);
    }
    log_material_buffer() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.engine_log_material_buffer(this.__wbg_ptr);
    }
    /**
     * @param {number} width
     * @param {number} height
     * @returns {Engine}
     */
    static new(width, height) {
        _assertNum(width);
        _assertNum(height);
        const ret = wasm.engine_new(width, height);
        return Engine.__wrap(ret);
    }
    /**
     * @returns {number}
     */
    render() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        const ret = wasm.engine_render(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} cursor_x
     * @param {number} cursor_y
     */
    set_cursor_position(cursor_x, cursor_y) {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        _assertNum(cursor_x);
        _assertNum(cursor_y);
        wasm.engine_set_cursor_position(this.__wbg_ptr, cursor_x, cursor_y);
    }
    step() {
        if (this.__wbg_ptr == 0) throw new Error('Attempt to use a moved value');
        _assertNum(this.__wbg_ptr);
        wasm.engine_step(this.__wbg_ptr);
    }
}
if (Symbol.dispose) Engine.prototype[Symbol.dispose] = Engine.prototype.free;

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18}
 */
export const MaterialId = Object.freeze({
    Empty: 0, "0": "Empty",
    Barrier: 1, "1": "Barrier",
    Sand: 2, "2": "Sand",
    WetSand: 3, "3": "WetSand",
    Water: 4, "4": "Water",
    Oil: 5, "5": "Oil",
    Steam: 6, "6": "Steam",
    Smog: 7, "7": "Smog",
    Fire: 8, "8": "Fire",
    AssetPlantGreen1: 9, "9": "AssetPlantGreen1",
    AssetPlantGreen2: 10, "10": "AssetPlantGreen2",
    AssetPlantGreen3: 11, "11": "AssetPlantGreen3",
    AssetPlantGreen4: 12, "12": "AssetPlantGreen4",
    AssetPlantYellow1: 13, "13": "AssetPlantYellow1",
    AssetPlantBrown1: 14, "14": "AssetPlantBrown1",
    AssetPlantBrown2: 15, "15": "AssetPlantBrown2",
    DebugRay: 16, "16": "DebugRay",
    UserCursor: 17, "17": "UserCursor",
    Count: 18, "18": "Count",
});

export function init_console_logger() {
    wasm.init_console_logger();
}

/**
 * @returns {any}
 */
export function wasmMemory() {
    const ret = wasm.wasmMemory();
    return ret;
}

//#endregion

//#region wasm imports
export function __wbg___wbindgen_memory_3f8442e22540244f() {
    const ret = wasm.memory;
    return ret;
}
export function __wbg___wbindgen_throw_5d9e815e6fdf150f(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
}
export function __wbg_error_757e9472f8410341() { return logError(function (arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
}, arguments); }
export function __wbg_log_363d83b9114c8831() { return logError(function (arg0) {
    console.log(arg0);
}, arguments); }
export function __wbg_new_227d7c05414eb861() { return logError(function () {
    const ret = new Error();
    return ret;
}, arguments); }
export function __wbg_stack_3b0d974bbf31e44f() { return logError(function (arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
}, arguments); }
export function __wbindgen_generic_0000000000000001() { return logError(function (arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
}, arguments); }
export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
}

//#endregion
const EngineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_engine_free(ptr, 1));


//#region intrinsics
function _assertNum(n) {
    if (typeof(n) !== 'number') throw new Error(`expected a number argument, found ${typeof(n)}`);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    return decodeText(ptr >>> 0, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function logError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        let error = (function () {
            try {
                return e instanceof Error ? `${e.message}\n\nStack:\n${e.stack}` : e.toString();
            } catch(_) {
                return "<failed to stringify thrown value>";
            }
        }());
        console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
        throw e;
    }
}

function passStringToWasm0(arg, malloc, realloc) {
    if (typeof(arg) !== 'string') throw new Error(`expected a string argument, found ${typeof(arg)}`);
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);
        if (ret.read !== arg.length) throw new Error('failed to pass whole string');
        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;


//#endregion

//#region wasm loading

let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

//#endregion
