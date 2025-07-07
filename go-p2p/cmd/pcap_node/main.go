package main

import (
    "fmt"
    "github.com/elementary-particles-Man/KAIRO/go-p2p/pkg"
    "github.com/google/gopacket"
    "github.com/google/gopacket/pcap"
    libp2p "github.com/libp2p/go-libp2p"
)

func main() {
    host, err := libp2p.New()
    if err != nil {
        fmt.Println("libp2p init failed:", err)
        return
    }
    fmt.Println("Node ID:", host.ID())

    handle, err := pcap.OpenOffline("../../samples/kairof_sample.pcap")
    if err != nil {
        fmt.Println("pcap open failed:", err)
        pkg.LogParseError()
        return
    }
    defer handle.Close()

    source := gopacket.NewPacketSource(handle, handle.LinkType())
    count := 0
    for range source.Packets() {
        count++
    }
    fmt.Println("packets read:", count)
}
