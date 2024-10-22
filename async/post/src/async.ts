import { fetchAsPromise, generatePassage, URLS } from "./lib.ts";

async function runAsync(): Promise<void> {
  const textA: string = await fetchAsPromise(URLS[0]);
  const textB: string = await fetchAsPromise(URLS[1]);
  const passage: string = generatePassage([textA, textB]);
  console.log(passage);
}

async function runConcurrentAsync(): Promise<void> {
  const promises: Array<Promise<string>> = URLS.map(fetchAsPromise);
  const quotes: Array<string> = await Promise.all(promises);
  const passage: string = generatePassage(quotes);
  console.log(passage);
}

export { runAsync, runConcurrentAsync };
