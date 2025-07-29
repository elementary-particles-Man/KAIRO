import subprocess
import time
import socket
import os

# 実行ファイルのパス (実際のパスに合わせて調整してください)
DAEMON_PATH = "D:/Dev/bin/target/debug/deps/kairo-daemon.exe"
SEED_NODE_PATH = "D:/Dev/bin/target/debug/deps/seed_node.exe"
MESH_NODE_PATH = "D:/Dev/KAIRO/go-p2p/cmd/main.exe" # Goのバイナリのパス

# ポート番号
DAEMON_PORT = 3030
SEED_NODE_PORT = 8000
MESH_NODE_PORT = 8080

def is_port_in_use(port):
    """指定されたポートが使用中かどうかを確認します。"""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        return s.connect_ex(('127.0.0.1', port)) == 0

def start_process(path, name, port):
    """プロセスをバックグラウンドで起動し、ポートがリッスンされるまで待機します。"""
    print(f"{name} を起動中...")
    process = subprocess.Popen([path], stdout=subprocess.PIPE, stderr=subprocess.PIPE, creationflags=subprocess.CREATE_NEW_PROCESS_GROUP)
    
    # ポートがリッスンされるまで待機
    timeout = 30 # タイムアウト秒数
    start_time = time.time()
    while not is_port_in_use(port):
        if time.time() - start_time > timeout:
            print(f"エラー: {name} が {port} で起動しませんでした。")
            process.terminate()
            return None
        time.sleep(1)
    print(f"{name} がポート {port} で起動しました。")
    return process

def main():
    processes = []

    # kairo-daemon 起動
    daemon_process = start_process(DAEMON_PATH, "kairo-daemon", DAEMON_PORT)
    if daemon_process:
        processes.append(daemon_process)

    # seed_node 起動
    seed_node_process = start_process(SEED_NODE_PATH, "seed_node", SEED_NODE_PORT)
    if seed_node_process:
        processes.append(seed_node_process)

    # mesh_node 起動 (Goのバイナリはgo buildで生成されるため、事前にビルドが必要です)
    # Goのバイナリをビルド
    print("Goのメッシュノードをビルド中...")
    go_build_command = "go build -o main.exe ."
    go_build_dir = "D:/Dev/KAIRO/go-p2p/cmd"
    try:
        subprocess.run(go_build_command, shell=True, check=True, cwd=go_build_dir)
        print("Goのメッシュノードのビルドが完了しました。")
    except subprocess.CalledProcessError as e:
        print(f"Goのメッシュノードのビルドに失敗しました: {e}")
        # ビルド失敗時は終了
        for p in processes:
            p.terminate()
        return

    mesh_node_process = start_process(MESH_NODE_PATH, "mesh_node", MESH_NODE_PORT)
    if mesh_node_process:
        processes.append(mesh_node_process)

    if not processes:
        print("すべてのサービスの起動に失敗しました。")
        return

    print("\nすべてのサービスが起動しました。")
    print("プロセスを停止するには、このスクリプトを終了してください (Ctrl+C)。")

    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("\nサービスを停止中...")
        for p in processes:
            p.terminate()
        print("すべてのサービスが停止しました。")

if __name__ == "__main__":
    main()
