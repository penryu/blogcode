#!/usr/bin/env python3

import aiohttp
import asyncio
import statistics
import threading
import time

from aiohttp import request

from typing import Dict, List


COUNT: int = 50
URL: str = "https://api.ipify.org?format=json"


async def fetchPage(url: str) -> int:
    async with aiohttp.request('GET', url) as resp:
        body = await resp.text()
    return len(str(body))


async def runSync():
    print("Starting sync...")
    start_ns = time.time_ns()
    responses: List = [await fetchPage(URL) for i in range(COUNT)]
    delta_ms = (time.time_ns() - start_ns) / 1000000
    bytes = sum(responses)
    print(f"Sync: {bytes} bytes in {delta_ms}ms")


async def runAsync():
    print("Starting async...")
    start_ns = time.time_ns()
    tasks = [fetchPage(URL) for i in range(COUNT)]
    responses = await asyncio.gather(*tasks)
    delta_ms = (time.time_ns() - start_ns) / 1000000
    bytes = sum(responses)
    print(f"Async: {bytes} bytes in {delta_ms}ms")


async def main():
    threads = (
        threading.Thread(target=lambda: asyncio.run(runSync())),
        threading.Thread(target=lambda: asyncio.run(runAsync())),
    )

    print("Starting threads...")
    [t.start() for t in threads]

    print("Joining threads...")
    [t.join() for t in threads]


if __name__ == '__main__':
    asyncio.run(main())
