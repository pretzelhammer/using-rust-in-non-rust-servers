const path = require('path');
const fs = require('fs');

// fetch WASM file
const qrWasmPath = path.resolve(__dirname, './qr_wasm.wasm');
const qrWasmBinary = fs.readFileSync(qrWasmPath);

// instantiate WASM module
const qrWasmModule = new WebAssembly.Module(qrWasmBinary);
const qrWasmInstance = new WebAssembly.Instance(
    qrWasmModule,
    {},
);

// JS strings are UTF16, but we need to re-encode them
// as UTF8 before passing them to our WASM module
const textEncoder = new TextEncoder();

// tell WASM module to allocate two buffers for us:
// - 1st buffer: an input buffer which we'll
//               write UTF8 strings into that
//               the generateQrCode function
//               will read
// - 2nd buffer: an output buffer that the
//               generateQrCode function will
//               write QR code PNG bytes into
//               and that we'll read
const textMemLen = 1024;
const textMemOffset = qrWasmInstance.exports.alloc(textMemLen);
const outputMemLen = 4096;
const outputMemOffset = qrWasmInstance.exports.alloc(outputMemLen);

/**
 * @param {string} text - text to encode
 * @returns {Buffer} - QR code
 */
module.exports = function generateQrCode(text) {
    // convert UTF16 JS string to Uint8Array
    let encodedText = textEncoder.encode(text);
    let encodedTextLen = encodedText.length;

    // write string into WASM memory
    qrWasmMemory = new Uint8Array(qrWasmInstance.exports.memory.buffer);
    qrWasmMemory.set(encodedText, textMemOffset);

    const wroteBytes = qrWasmInstance.exports.generateQrCode(
        textMemOffset,
        encodedTextLen,
        outputMemOffset,
        outputMemLen,
    );

    if (wroteBytes === 0) {
        throw new Error('failed to generate qr');
    }

    // read QRr code PNG bytes from WASM memory & return
    return Buffer.from(
        qrWasmInstance.exports.memory.buffer,
        outputMemOffset,
        wroteBytes,
    );
};
