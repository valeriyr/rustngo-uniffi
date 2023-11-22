package main

// #cgo LDFLAGS: -lsocket
import "C"

import (
	"GoApp/Socket"
	"fmt"
	"sync"
)

type SocketWatcher struct{}

func (w SocketWatcher) OnReceived(data string) {
	fmt.Println("<go> watched:", data)
}

func main() {
	watcher := SocketWatcher{}
	socket := Socket.CreateSocket("1.2.3.4", watcher)

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
