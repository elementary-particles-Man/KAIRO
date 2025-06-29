import threading
import time
import json
import base64
import hmac
from hashlib import sha256
import http.client

from server import run_server, EPHEMERAL_KEY

# start server in background
server_thread = threading.Thread(target=run_server, daemon=True)
server_thread.start()

# give the server a moment to start
time.sleep(1)

conn = http.client.HTTPConnection('127.0.0.1', 8000)

# 1. generate ephemeral key
conn.request('POST', '/generate_ephemeral_key', body='')
resp = conn.getresponse()
key_data = json.loads(resp.read())
key = base64.b64decode(key_data['ephemeral_key'])
print('Generated key')

# 2. verify signature
message = 'hello'
signature = base64.b64encode(hmac.new(key, message.encode(), sha256).digest()).decode()
body = json.dumps({'data': message, 'signature': signature})
conn.request('POST', '/verify_signature', body, headers={'Content-Type': 'application/json'})
print('Verify:', conn.getresponse().read())

# 3. log_vov
conn.request('POST', '/log_vov', json.dumps({'message': 'dummy traffic'}), headers={'Content-Type': 'application/json'})
print('Log VOV:', conn.getresponse().read())

# 4. apply deny rule to block this host
conn.request('POST', '/apply_deny_rules', json.dumps({'client_id': '127.0.0.1'}), headers={'Content-Type': 'application/json'})
print('Apply deny:', conn.getresponse().read())

# 5. attempt another request (should be denied)
conn.request('POST', '/verify_signature', body, headers={'Content-Type': 'application/json'})
print('Denied attempt code:', conn.getresponse().status)

# simulate dummy traffic on a loop; it should end once server disconnects
stop = False

def dummy_traffic():
    while not stop:
        try:
            c = http.client.HTTPConnection('127.0.0.1', 8000, timeout=1)
            c.request('POST', '/log_vov', json.dumps({'message': 'spam'}), headers={'Content-Type': 'application/json'})
            c.getresponse().read()
            time.sleep(0.2)
        except Exception:
            break

dummy_thread = threading.Thread(target=dummy_traffic)
dummy_thread.start()

# 6. force disconnect
conn.request('POST', '/force_disconnect', '{}', headers={'Content-Type': 'application/json'})
print('Force disconnect:', conn.getresponse().read())

# wait for server to shut down
server_thread.join()
stop = True
dummy_thread.join()
print('Server stopped')
