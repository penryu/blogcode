import { fetchWithCallback, generatePassage, URLS } from "./lib.ts";

function runCallback() {
  fetchWithCallback(URLS[0], (textA: string) => {
    fetchWithCallback(URLS[1], (textB: string) => {
      const fullPassage: string = generatePassage([textA, textB]);

      console.log(fullPassage);
    });
  });
}

export { runCallback };
