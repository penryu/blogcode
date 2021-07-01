import { runAsync, runConcurrentAsync } from './async';
import { runCallback } from './callback';
import { runPromise } from './promise';

async function main() {
    const allMethods: Map<string, () => void> = new Map([
        ['callback', runCallback],
        ['promise', runPromise],
        ['async', runAsync],
        ['concurrentAsync', runConcurrentAsync],
    ]);

    console.log('Starting fetch routines...');
    allMethods.forEach(async (f, label) => {
        const start: number = Date.now();
        const result = await f();
        const delta: number = Date.now() - start;
        console.log(`${label} complete in ${delta}ms`);
        console.info(result);
    });
    console.log('Finished starting fetch routines...');
}

main();
