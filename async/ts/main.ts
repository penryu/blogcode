import fetchURL from 'node-fetch';

const RequestCount: number = 50;
const TargetUrl: string = 'https://api.ipify.org?format=json';

async function fetchEndpoint(url: string): Promise<object> {
    const startTime = Date.now();
    const res = await fetchURL(url);
    const text = await res.text();
    return { time: Date.now() - startTime, size: text.length };
}

async function runSync(url: string, count: number): Promise<object> {
    console.log('Sync starting...');

    const startTime = Date.now();
    const responses = [];
    for (let i = 0; i < count; ++i) {
        responses.push(await fetchEndpoint(url));
    }
    const deltaT = Date.now() - startTime;
    const totalT = responses.reduce((acc, n) => (n.time + acc), 0)
    const meanT = totalT / responses.length;

    console.log('Sync done.');
    return { syncAverage: meanT, syncElapsed: deltaT };
}

async function runAsync(url: string, count: number): Promise<object> {
    console.log('Async starting...');

    const startTime = Date.now();
    const promises = [];
    for (let i = 0; i < count; ++i) {
        promises.push(fetchEndpoint(url));
    }
    const responses = await Promise.all(promises);
    const deltaT = Date.now() - startTime;
    const totalT = responses.reduce((acc, n) => (n.time + acc), 0)
    const meanT = totalT / responses.length;

    console.log('Async done.');
    return { asyncAverage: meanT, asyncElapsed: deltaT };
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
