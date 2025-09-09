for ($i = 1; $i -le 10; $i++) {
    Write-Host "Sending packet $i..."
    $client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 8080)
    $stream = $client.GetStream()
    $writer = New-Object System.IO.StreamWriter($stream)
    $reader = New-Object System.IO.StreamReader($stream)

    $writer.WriteLine("TEST_PACKET_$i")
    $writer.Flush()

    $response = $reader.ReadLine()
    Write-Host "Received: $response"

    $stream.Close()
    $client.Close()
    Start-Sleep -Milliseconds 100 # 短い間隔で送信
}