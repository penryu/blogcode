// a simple pretty-printer for iterables
function ppi<T>(it: Iterable<T>): string {
  return Array.from(it).join(', ');
}

function eagerOdds(n: number): number[] {
  const results: Array<number> = [];
  let i = 1;
  while (n-- > 0) {
    results.push(i);
    i += 2;
  }

  return results;
}

function* range(start: number, end: number, step = 1): Generator<number> {
  for (let i = start; i < end; i += step)
    yield i;
}

function* take<T>(n: number, it: Iterator<T>): Generator<T> {
  while (n-- > 0) {
    const result = it.next();
    if (result.done) break;
    yield result.value;
  }
}

// (start?: number) -> Generator<number>
function* naturals(start = 0): Generator<number> {
  while (true) yield start++;
}

function* filter<T>(f: (x: T) => boolean, it: Iterator<T>): Generator<T> {
  let result = it.next();
  while (!result.done) {
    if (f(result.value)) yield result.value;

    result = it.next();
  }
}

function isOdd(n: number): boolean {
  return (n % 2 !== 0);
}


function* allTheFibs(): Generator<number> {
  for (let [m, n] = [1, 1];; [m, n] = [n, m + n])
    yield m;
}

function drop<T>(n: number, it: Iterator<T>): Iterator<T> {
  while (n-- > 0) {
    const result = it.next();
    if (result.done) break;
  }

  return it;
}

function nth<T>(n: number, it: Iterator<T>): T {
  return take(1, drop(n - 1, it)).next().value;
}

function fib(n: number): number {
  return nth(n, allTheFibs());
}

function* bigFibs(): Generator<bigint> {
  for (let [m, n] = [1n, 1n];; [m, n] = [n, m + n])
    yield m;
}

