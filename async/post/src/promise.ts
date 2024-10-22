import { fetchAsPromise, generatePassage, URLS } from "./lib.ts";

function runPromise(): Promise<void> {
  const quotes: Array<string> = [];

  return fetchAsPromise(URLS[0]).then((text: string) => {
    quotes.push(text);
    return fetchAsPromise(URLS[1]);
  }).then((text: string) => {
    quotes.push(text);
    const fullPassage = generatePassage(quotes);
    console.log(fullPassage);
  }).catch((err: unknown) => {
    console.error(err);
  });
}

export { runPromise };
