package main

// #cgo LDFLAGS: -lsocket
import "C"

import (
	"GoApp/Socket"
	"fmt"
	"sync"
)

func main() {
	socket := Socket.CreateSocket("1.2.3.4")

	var wg sync.WaitGroup
	wg.Add(2)

	go func() {
		socket.Write("hello from go!")
		defer wg.Done()
	}()

	go func() {
		data := socket.Read();

		fmt.Println("<go> received:", *data)

		defer wg.Done()
	}()

	wg.Wait()

	socket.Destroy();
}
