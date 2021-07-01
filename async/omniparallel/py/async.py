#!/usr/bin/env python3

import aiohttp
import asyncio
import sys
import threading
import time

from typing import List


COUNT: int = 500


async def fetchPage(url: str) -> int:
    async with aiohttp.request('GET', url) as resp:
        body = await resp.text()
    return len(str(body))


async def runSync(url: str):
    print("Starting sync...")
    start_ns = time.time_ns()
    responses: List = [await fetchPage(url) for i in range(COUNT)]
    delta_ms = (time.time_ns() - start_ns) / 1000000
    bytes = sum(responses)
    print(f"Sync: {bytes} bytes in {delta_ms}ms")


async def runAsync(url: str):
    print("Starting async...")
    start_ns = time.time_ns()
    tasks = [fetchPage(url) for i in range(COUNT)]
    responses = await asyncio.gather(*tasks)
    delta_ms = (time.time_ns() - start_ns) / 1000000
    bytes = sum(responses)
    print(f"Async: {bytes} bytes in {delta_ms}ms")


async def main():
    if len(sys.argv) < 2:
        myname = sys.argv[0]
        print(f"Usage: {myname} URL")
        sys.exit(1)

    url = sys.argv[1]

    threads = (
        threading.Thread(target=lambda: asyncio.run(runSync(url))),
        threading.Thread(target=lambda: asyncio.run(runAsync(url))),
    )

    print("Starting threads...")
    [t.start() for t in threads]

    print("Joining threads...")
    [t.join() for t in threads]


if __name__ == '__main__':
    asyncio.run(main())
