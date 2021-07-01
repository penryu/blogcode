import { fetchAsPromise, generatePassage, URLS } from './lib';

async function runPromise(): Promise<void> {
    const quotes: Array<string> = [];

    return fetchAsPromise(URLS[0]).then((text: string) => {
        quotes.push(text);
        return fetchAsPromise(URLS[1]);
    }).then((text: string) => {
        quotes.push(text);
        const fullPassage = generatePassage(quotes);
        console.log(fullPassage);
    }).catch((err) => {
        console.error(err);
    });
}

export { runPromise };
