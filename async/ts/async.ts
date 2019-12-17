import fetchURL from 'node-fetch';

const RequestCount: number = 50;
const TargetUrl: string = 'https://api.ipify.org?format=json';

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

async function main() {
    const results = await Promise.all([
        runSync(TargetUrl, RequestCount),
        runAsync(TargetUrl, RequestCount),
    ]);

    for (const result of results) {
        console.log(result);
    }
}

main();
