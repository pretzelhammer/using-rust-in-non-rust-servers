const { spawn } = require('child_process');
const path = require('path');
const qrCliPath = path.resolve(__dirname, './qr-cli');

/**
 * @param {string} text - text to encode
 * @returns {Promise<Buffer>} - qr code
 */
module.exports = function generateQrCode(text) {
    return new Promise((resolve, reject) => {
        const qrCli = spawn(qrCliPath, [text]);
        const qrCodeData = [];
        qrCli.stdout.on('data', (data) => {
            qrCodeData.push(data);
        });
        qrCli.stderr.on('data', (data) => {
            reject(new Error(`error generating qr code: ${data}`));
        });
        qrCli.on('error', (err) => {
            reject(new Error(`failed to start qr-cli ${err}`));
        });
        qrCli.on('close', (code) => {
            if (code === 0) {
                resolve(Buffer.concat(qrCodeData));
            } else {
                reject(new Error('qr-cli exited unsuccessfully'));
            }
        });
    });
};
