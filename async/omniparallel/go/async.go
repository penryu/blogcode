package main

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"os"
	"path"
	"sync"
	"time"
)

func fetchURL(url string) int {
	resp, err := http.Get(url)
	if err != nil {
		fmt.Println(err)
		return 0
	}

	body, err := ioutil.ReadAll(resp.Body)
	if err != nil {
		fmt.Println(err)
		return 0
	}

	resp.Body.Close()
	return len(body)
}

func fetchSync(url string, count int, wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("Sync starting...")

	start := time.Now()

	var bytes int
	for i := 0; i < count; i++ {
		bytes += fetchURL(url)
	}
	delta := time.Now().Sub(start)

	fmt.Printf("Sync: %d bytes in %s\n", bytes, delta)
}

func fetchWrapper(url string) chan int {
	c := make(chan int)

	go func() {
		defer close(c)
		c <- fetchURL(url)
	}()

	return c
}

func fetchAsync(url string, count int, wg *sync.WaitGroup) {
	defer wg.Done()
	fmt.Println("Async starting...")

	start := time.Now()
	var chans []chan int
	for i := 0; i < count; i++ {
		chans = append(chans, fetchWrapper(url))
	}
	var bytes int
	for _, c := range chans {
		bytes += <-c
	}
	delta := time.Now().Sub(start)

	fmt.Printf("Async: %d bytes in %s\n", bytes, delta)
}

func main() {
	if len(os.Args) < 2 {
		myname := path.Base(os.Args[0])
		fmt.Printf("Usage: %s URL\n", myname)
		os.Exit(1)
	}

	targetURL := os.Args[1]

	requestCount := 500

	var wg sync.WaitGroup
	wg.Add(2)

	// kick off routines in background
	go fetchSync(targetURL, requestCount, &wg)
	go fetchAsync(targetURL, requestCount, &wg)

	wg.Wait()
}
