import * as http from 'http';
import * as https from 'https';
import * as fetch from 'node-fetch';

const URLS = [
    "https://api.kanye.rest/?format=text",
    "https://loripsum.net/api/1/short/plaintext",
];

async function delay(ms: number): Promise<void> {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

function generatePassage(quotes: Array<string>): string {
    const samples = quotes.map((q) => q.trim());

    if (Math.round(Math.random()) === 1) {
        samples.reverse();
    }

    return samples.join(' - ');
}

function fetchWithCallback(url: string, cb: Function): void {
    // Adapted from the node.js documentation: https://nodejs.org/api/
    https.get(url, (res: http.IncomingMessage) => {
        if (res.statusCode !== 200) {
            const error = new Error(`Invalid response from ${url}`);
            console.log(error.message);
            res.resume();
            return;
        }

        let text = '';
        res.setEncoding('utf8');
        res.on('data', (chunk) => { text += chunk; });
        res.on('end', () => cb(text));
    });
}

function fetchAsPromise(url: string): Promise<string> {
    // for this application, only HTTP 200 responses are considered success
    const result: Promise<string> = fetch(url)
        .then((res: Response) => {
        if (!res.ok) {
            throw new Error(`Unexpected Response: ${url}`);
        }
        return res.text();
    });

    return result;
}

export {
    URLS,
    delay,
    fetchWithCallback,
    fetchAsPromise,
    generatePassage,
};
