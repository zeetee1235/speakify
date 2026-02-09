import init, { SpeakifyWasm } from './pkg/speakify.js';

let wasmReady = null;
let wasmInstance = null;

async function getWasm() {
    if (wasmInstance) return wasmInstance;
    if (!wasmReady) {
        wasmReady = (async () => {
            await init();
            wasmInstance = new SpeakifyWasm();
            return wasmInstance;
        })();
    }
    return wasmReady;
}

self.onmessage = async (event) => {
    const { id, bytes, resolution, frames } = event.data;
    try {
        const wasm = await getWasm();
        const inputBytes = new Uint8Array(bytes);
        const gifBytes = wasm.convert(inputBytes, resolution, frames);
        self.postMessage({ id, ok: true, bytes: gifBytes.buffer }, [gifBytes.buffer]);
    } catch (error) {
        const message = error && error.message ? error.message : String(error);
        self.postMessage({ id, ok: false, error: message });
    }
};
