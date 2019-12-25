import fetchURL from 'node-fetch';
import { Observable } from 'rxjs';

async function fetchEndpoint(url: string): Promise<number> {
    const resp = await fetchURL(url);
    const text = await resp.text();
    return text.length;
}

async function runSync(url: string, count: number): Promise<number> {
    let bytes: number = 0;
    for (let i = 0; i < count; ++i) {
        bytes += await fetchEndpoint(url);
    }
    return bytes;
}

async function runAsync(url: string, count: number): Promise<number> {
    const promises = [];
    for (let i = 0; i < count; ++i) {
        promises.push(fetchEndpoint(url));
    }
    const results = await Promise.all(promises);
    return results.reduce((acc, n) => (acc + n), 0);
}

function runObservable(url: string, count: number): Promise<number> {
    return new Promise((resolve) => {
        const observableFetches = new Observable((subscriber) => {
            const promises = [];
            for (let i = 0; i < count; ++i) {
                promises.push(fetchEndpoint(url).then(b => subscriber.next(b)));
            }
            Promise.all(promises).then(() => subscriber.complete());
        });

        const results = [];
        observableFetches.subscribe({
            next(x) { results.push(x); },
            complete() {
                const bytes = results.reduce((acc, n) => (acc + n), 0);
                resolve(bytes);
            }
        });
    });
}

async function main() {
    const path = require('path');
    const process = require('process');
    const RequestCount: number = 100;

    if (process.argv.length < 3) {
        const myname = path.basename(process.argv[1]);
        console.log(`Usage: ${myname} URL`);
        process.exit(1);
    }
    const TargetUrl: string = process.argv[2];

    const operations = new Map<CallableFunction, string>([
        [runSync, 'Sync'],
        [runAsync, 'Async'],
        [runObservable, 'Observable'],
    ]);

    operations.forEach((label: string, f: CallableFunction) => {
        console.log(`${label} starting...`);
        const start = Date.now();
        f(TargetUrl, RequestCount).then((bytes) => {
            const delta = Date.now() - start;
            console.log(`${label}: ${bytes} bytes in ${delta}ms.`);
        });
    });
}

main();
