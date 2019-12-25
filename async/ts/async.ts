import fetchURL from 'node-fetch';
import { Observable } from 'rxjs';

async function fetchEndpoint(url: string): Promise<number> {
    const resp = await fetchURL(url);
    const text = await resp.text();
    return text.length;
}

async function runSync(url: string, count: number): Promise<string> {
    console.log('Sync starting...');

    const startTime = Date.now();
    let bytes: number = 0;
    for (let i = 0; i < count; ++i) {
        bytes += await fetchEndpoint(url);
    }
    const deltaT = Date.now() - startTime;

    return `Sync: ${bytes} bytes in ${deltaT}ms.`;
}

async function runAsync(url: string, count: number): Promise<string> {
    console.log('Async starting...');

    const startTime = Date.now();
    const promises = [];
    for (let i = 0; i < count; ++i) {
        promises.push(fetchEndpoint(url));
    }
    const results = await Promise.all(promises);
    const deltaT = Date.now() - startTime;
    const bytes = results.reduce((acc, n) => (acc + n), 0);

    return `Async: ${bytes} bytes in ${deltaT}ms`;
}

function runObservable(url: string, count: number): Promise<string> {
    console.log('Observable starting...');

    return new Promise((resolve) => {
        const startTime = Date.now();
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
                const deltaT = Date.now() - startTime;
                const bytes = results.reduce((acc, n) => (acc + n), 0);
                resolve(`Observable ${bytes} bytes in ${deltaT}ms`);
            }
        });
    });
}

async function main() {
    const path = require('path');
    const process = require('process');
    const RequestCount: number = 50;

    if (process.argv.length < 3) {
        const myname = path.basename(process.argv[1]);
        console.log(`Usage: ${myname} URL`);
        process.exit(1);
    }
    const TargetUrl: string = process.argv[2];
    runSync(TargetUrl, RequestCount).then(console.log);
    runAsync(TargetUrl, RequestCount).then(console.log);
    runObservable(TargetUrl, RequestCount).then(console.log);
}

main();
