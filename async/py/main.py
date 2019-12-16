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


async def fetchPage(url: str) -> Dict[str, int]:
    start_ns: int = time.time_ns()
    async with aiohttp.request('GET', url) as resp:
        text: str = await resp.text()
        stop_ns: int = time.time_ns()
    delta_ms = (stop_ns - start_ns) / 1000000
    return {
        'status': resp.status,
        'time': delta_ms,
        'size': len(text),
    }


async def runSync():
    print("Starting sync...")
    start_ns: int = time.time_ns()
    responses: List = [await fetchPage(f"{URL}?n={i}") for i in range(COUNT)]
    stop_ns: int = time.time_ns()
    delta_ms = (stop_ns - start_ns) / 1000000

    times = [resp['time'] for resp in responses]
    avg_time = statistics.mean(times)
    total_time = sum(times)
    print("Stopping sync...")

    print({"syncAverage": avg_time, "syncElapsed": delta_ms});


async def runAsync():
    print("Starting async...")
    start_ns: int = time.time_ns()
    tasks = [fetchPage(f"{URL}?n={i}") for i in range(COUNT)]
    responses = await asyncio.gather(*tasks)
    stop_ns: int = time.time_ns()
    delta_ms = (stop_ns - start_ns) / 1000000

    times = [resp['time'] for resp in responses]
    avg_time = statistics.mean(times)
    total_time = sum(times)
    print("Stopping async...")

    print({"asyncAverage": avg_time, "asyncElapsed": delta_ms});


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
