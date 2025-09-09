import subprocess
import time
import socket

# 起動対象とその確認ポート
services = [
    {
        "name": "kairo_daemon",
        "cmd": ["cargo", "run", "--package", "kairo_daemon"],
        "port": 8080
    },
    {
        "name": "seed_node",
        "cmd": ["cargo", "run", "--bin", "seed_node", "--", "--config", "config/seed_1.yaml"],
        "port": 8000
    },
    {
        "name": "mesh_node",
        "cmd": ["cargo", "run", "--bin", "mesh_node", "--", "--join-address", "127.0.0.1:9000", "--id", "mesh_01"],
        "port": 8080  # mesh_nodeも同じポートを使う場合、タイミング確認が必要
    },
    {
        "name": "kairobot",
        "cmd": ["cargo", "run", "--package", "kairo_core", "--bin", "kairobot"],
        "port": 4040
    },
]

def wait_for_port(port, host='127.0.0.1', timeout=180):
    print(f"[WAIT] Waiting for port {port} to open...", end="", flush=True)
    start = time.time()
    while time.time() - start < timeout:
        try:
            with socket.create_connection((host, port), timeout=1):
                print(" OK")
                return True
        except (ConnectionRefusedError, OSError):
            print(".", end="", flush=True)
            time.sleep(1)
    print(f"\n[ERROR] Timeout waiting for port {port}")
    return False

# 起動と確認
for service in services:
    print(f"[START] Launching {service['name']}")
    subprocess.Popen(service["cmd"])
    if not wait_for_port(service["port"]):
        print(f"[FAIL] {service['name']} failed to start properly.")
