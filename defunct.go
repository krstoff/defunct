package main

import (
	"fmt"
	"os"
    "bufio"
    "io"
    "strings"
)

const (
    initialBufferSize = 1024
)

type data int

func read(buf []byte, reader io.Reader, symbuf strings.Builder) (data, error) {
    _, _ = reader.Read(buf)
    return data(len(buf) + len(symbuf.String())), nil
}

func loadFile(filename string) (data, error) {
    var file, err = os.OpenFile(filename, os.O_RDONLY, 0)
    if err != nil {
        return 0, err
    }
    defer file.Close()

    var symbuf strings.Builder
    fileStat, err := file.Stat()
    if err != nil { return 0, err }
    symbuf.Grow(int(fileStat.Size() / 2))
    var reader = bufio.NewReader(file)
    var buf = make([]byte, initialBufferSize)
    data, err := read(buf, reader, symbuf)
    return data, nil
}

func main() {
	var args = os.Args[1:]
    if len(args) < 1 {
        fmt.Println("usage: defunct FILE...")
        os.Exit(1)
    }

    // one file for now
    var filename = args[0]
    var data, err = loadFile(filename)
    if err != nil {
        fmt.Printf("Could not load file %s: %s", filename, err.Error())
    }

    fmt.Printf("buffer size: %d", int(data))
}
