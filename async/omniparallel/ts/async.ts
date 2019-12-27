import fetchURL from 'node-fetch';
import { from, merge, Observable } from 'rxjs';
import { reduce } from 'rxjs/operators';

async function fetchEndpoint(url: string): Promise<number> {
    // start the fetch (header and first chunk only)
    const resp = await fetchURL(url);
    // ensure full text (may be multiple chunks)
    const text = await resp.text();
    return text.length;
}

async function runSync(url: string, count: number) {
    // invokes async function synchronously {count} times in sequence
    let bytes: number = 0;
    for (let i = 0; i < count; ++i) {
        bytes += await fetchEndpoint(url);
    }
    return bytes;
}

async function runAsync(url: string, count: number): Promise<number> {
    // starts {count} async operations concurrently, collecting their promises
    const promises = [];
    for (let i = 0; i < count; ++i) {
        promises.push(fetchEndpoint(url));
    }
    // wait for _all_ promises to complete
    const results = await Promise.all(promises);
    // sum and return
    return results.reduce((acc, n) => (acc + n), 0);
}

function runObservable(url: string, count: number): Promise<number> {
    // construct {count} observables from the {count} promises
    // (similar to runAsync)
    const requests: Array<Observable<number>> = Array(count).fill(url)
        .map((u) => from(fetchEndpoint(u)));
    // merge Array<Observable<number>> => Observable<Array<number>>
    return merge(...requests)
        // ... => Observable<number> containing sum total bytes
        .pipe(reduce((acc, x) => acc + x))
        // to Promise for async function
        .toPromise();
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

    // map operation to label for data-driven operation
    const operationMap: Map<CallableFunction, string> = new Map([
        [runSync, 'Sync'],
        [runAsync, 'Async'],
        [runObservable, 'Observable'],
    ]);

    operationMap.forEach((label, f) => {
        console.log(`${label} starting...`);

        const startTime: number = Date.now();
        f(TargetUrl, RequestCount).then((bytes) => {
            const delta = Date.now() - startTime;
            console.log(`${label}: ${bytes} bytes in ${delta}ms.`);
        });
    });
}

main();
