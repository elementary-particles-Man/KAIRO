$packet = @{
    version = 1
    source_p_address = "10.0.0.23/24"
    destination_p_address = "gpt://main"
    sequence = 1 # 仮の値
    timestamp_utc = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds()
    payload_type = "application/json"
    payload = (@{
        type = "TEST"
        message = "Hello GPT, this is CLI."
    } | ConvertTo-Json -Compress)
    source_public_key = "4ced77c175ad3fd2d2ced979419e28635d179d0724e696350031c4e9b4912fb6"
    signature = "DUMMY_SIGNATURE_CLI" # 本来は署名が必要
} | ConvertTo-Json -Depth 5 -Compress

$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 8080)
$stream = $client.GetStream()
$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

$writer.WriteLine($packet)
$writer.Flush()

$response = $reader.ReadLine()
Write-Host "️ Response from KAIRO-P/GPT: $response"

$stream.Close()
$client.Close()