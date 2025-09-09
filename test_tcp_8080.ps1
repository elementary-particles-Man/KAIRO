$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 8080)
$stream = $client.GetStream()
$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

$writer.WriteLine("TEST_PACKET")
$writer.Flush()

$response = $reader.ReadLine()
Write-Host "Received: $response"

$stream.Close()
$client.Close()