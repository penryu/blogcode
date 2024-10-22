import { runAsync, runConcurrentAsync } from "./async.ts";
import { runCallback } from "./callback.ts";
import { runPromise } from "./promise.ts";

const allMethods: Map<string, () => unknown> = new Map([
  ["callback", runCallback],
  ["promise", runPromise],
  ["async", runAsync],
  ["concurrentAsync", runConcurrentAsync],
]);

console.log("Starting fetch routines...");
allMethods.forEach(async (f, label) => {
  const start: number = Date.now();
  const result = await f();
  const delta: number = Date.now() - start;
  console.log(`${label} complete in ${delta}ms`);
  console.info(result);
});
console.log("Finished starting fetch routines...");
