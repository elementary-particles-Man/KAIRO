import http.server
import json
import threading
import base64
import secrets
import hmac
from hashlib import sha256

# in-memory state for simplicity
EPHEMERAL_KEY = None
DENY_LIST = set()

class KairoHandler(http.server.BaseHTTPRequestHandler):
    def do_POST(self):
        global EPHEMERAL_KEY, DENY_LIST
        length = int(self.headers.get('Content-Length', 0))
        raw = self.rfile.read(length)
        try:
            payload = json.loads(raw) if raw else {}
        except json.JSONDecodeError:
            payload = {}

        client_ip = self.client_address[0]
        if client_ip in DENY_LIST and self.path != '/force_disconnect':
            self.send_response(403)
            self.end_headers()
            return

        if self.path == '/generate_ephemeral_key':
            EPHEMERAL_KEY = secrets.token_bytes(32)
            response = {
                'ephemeral_key': base64.b64encode(EPHEMERAL_KEY).decode()
            }
        elif self.path == '/verify_signature':
            data = payload.get('data', '')
            sig_b64 = payload.get('signature', '')
            try:
                signature = base64.b64decode(sig_b64)
            except Exception:
                signature = b''
            if EPHEMERAL_KEY is None:
                ok = False
            else:
                expected = hmac.new(EPHEMERAL_KEY, data.encode(), sha256).digest()
                ok = hmac.compare_digest(expected, signature)
            response = {'verified': ok}
        elif self.path == '/apply_deny_rules':
            cid = payload.get('client_id')
            if cid:
                DENY_LIST.add(cid)
                response = {'deny_list': list(DENY_LIST)}
            else:
                response = {'error': 'missing client_id'}
        elif self.path == '/log_vov':
            msg = payload.get('message', '')
            print(f'VOV: {msg}')
            response = {'logged': True}
        elif self.path == '/force_disconnect':
            response = {'disconnected': True}
        else:
            response = {'error': 'unknown command'}

        body = json.dumps(response).encode()
        self.send_response(200)
        self.send_header('Content-Type', 'application/json')
        self.send_header('Content-Length', str(len(body)))
        self.end_headers()
        self.wfile.write(body)

        if self.path == '/force_disconnect':
            threading.Thread(target=self.server.shutdown, daemon=True).start()

    def log_message(self, format, *args):
        return  # suppress default logging

def run_server(host='127.0.0.1', port=8000):
    server = http.server.ThreadingHTTPServer((host, port), KairoHandler)
    print(f'Server running on http://{host}:{port}')
    server.serve_forever()

if __name__ == '__main__':
    run_server()
