import requests
import time

url = "http://127.0.0.1:3030/send"
headers = {"Content-Type": "application/json"}

sequence_num = 0

def send_packet():
    global sequence_num
    sequence_num += 1

    data = {
        "version": 1,
        "source_p_address": "cli://test",
        "source_public_key": "dummy_public_key",
        "destination_p_address": "gpt://main",
        "sequence": sequence_num,
        "timestamp_utc": int(time.time()),
        "payload_type": "message",
        "payload": "test_from_CLI",
        "signature": "dummy_signature"
    }

    try:
        response = requests.post(url, headers=headers, json=data)
        response.raise_for_status()  # Raise an exception for HTTP errors (4xx or 5xx)
        print(f"Status Code: {response.status_code}")
        print(f"Response Body: {response.text}")
    except requests.exceptions.RequestException as e:
        print(f"Request failed: {e}")

# デーモン起動を待つ
time.sleep(5)

# 最初のパケットを送信
send_packet()

# 2番目のパケットを送信
send_packet()