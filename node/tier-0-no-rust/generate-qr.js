const QRCode = require('qrcode');

/**
 * @param {string} text - text to encode
 * @returns {Promise<Buffer>} - qr code
 */
module.exports = function generateQrCode(text) {
    return QRCode.toBuffer(text, {
        type: 'png',
        errorCorrectionLevel: 'L',
        width: 200,
        rendererOpts: {
            deflateLevel: 9, // 0 - 9
            deflateStrategy: 3, // 1 - 4
        },
    });
};
