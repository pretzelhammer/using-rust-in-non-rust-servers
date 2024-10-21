const wasm = require('./qr_wasm_bindgen.js');

/**
 * @param {string} text - text to encode
 * @returns {Buffer} - qr code
 */
module.exports = function generateQrCode(text) {
    return Buffer.from(wasm.generateQrCode(text));
};