const args = process.argv;
if (args.length !== 4) {
    console.error('Usage: node server.js {{tier}} {{mode}}');
    console.error('tier: "tier-0" | "tier-1" | "tier-2" | "tier-2-bindgen" | "tier-3"');
    console.error('mode: "single-threaded" | "multi-threaded"');
    process.exit(1);
}
const tierMap = {
    'tier-0': 'tier-0-no-rust',
    'tier-1': 'tier-1-rust-cli',
    'tier-2': 'tier-2-rust-wasm',
    'tier-2-bindgen': 'tier-2-rust-wasm-bindgen',
    'tier-3': 'tier-3-rust-native',
};
const tierArg = args[2];
if (tierArg === 'tier-4') {
    console.log('run this instead: just tier-4-server');
    process.exit(0);
}

const tier = tierMap[tierArg];
if (!tier) {
    console.error('tier: "tier-0" | "tier-1" | "tier-2" | "tier-2-bindgen" | "tier-3"');
    process.exit(1);
}

const modeArg = args[3];
if (!['single-threaded', 'multi-threaded'].includes(modeArg)) {
    console.error('mode: "single-threaded" | "multi-threaded"');
    process.exit(1);
}
const mode = modeArg;

const generateQrCode = require(`./${tier}/generate-qr.js`);
const express = require('express');

function startServer() {
    const app = express();
    app.disable('x-powered-by');
    const port = 42069;
    
    app.get('/qrcode', async (req, res) => {
        const { text } = req.query;
    
        if (!text) {
            return res.status(400).send('missing "text" query param');
        }

        if (text.length > 512) {
            return res.status(400).send('text must be <= 512 bytes');
        }
    
        try {
            const qr = await generateQrCode(text);
            res.setHeader('Content-Type', 'image/png');
            res.send(qr);
        } catch (err) {
            console.error('error generating qr:', err.message, err.stack);
            res.status(500).send('failed generating qr code');
        }
    });
    
    app.listen(port, () => {
        console.log(`server ${process.pid} running on http://localhost:${port}`);
    });
};

if (mode === 'single-threaded') {
    startServer();
} else if (mode === 'multi-threaded') {
    const cluster = require('cluster');
    const os = require('os');
    const numCPUs = os.cpus().length;

    if (cluster.isMaster) {
        console.log(`supervisor ${process.pid} is running`);

        console.log(`starting ${numCPUs} workers`);
        for (let i = 0; i < numCPUs; i++) {
            cluster.fork();
        }

        // restart worker if they exit
        cluster.on('exit', (worker, _code, _signal) => {
            console.log(`worker ${worker.process.pid} exited, starting new worker`);
            cluster.fork();
        });

    } else {
        // workers share the same server port
        startServer();
    }
} else {
    console.error('this is unreachable');
    process.exit(2);
}
