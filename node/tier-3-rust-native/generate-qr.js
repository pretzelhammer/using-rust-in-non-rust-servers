const native = require('./qr_napi.node');

/**
 * @param {string} text - text to encode
 * @returns {Buffer} - QR code
 */
module.exports = function generateQrCode(text) {
    return Buffer.from(native.generateQrCode(text));
};
