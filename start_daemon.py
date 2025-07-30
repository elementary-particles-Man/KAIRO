import subprocess
import os
import sys
import time

daemon_path = os.path.abspath("D:/Dev/bin/target/debug/kairo-daemon.exe")

# Windowsの場合
if sys.platform == "win32":
    # DETACHED_PROCESS フラグを使用して、親プロセスから分離して起動
    # CREATE_NEW_PROCESS_GROUP を使用して、新しいプロセスグループを作成
    creationflags = subprocess.DETACHED_PROCESS | subprocess.CREATE_NEW_PROCESS_GROUP
    # shell=True を使用して、コマンドプロンプトを介して実行
    subprocess.Popen(f'"{daemon_path}"', creationflags=creationflags, close_fds=True, shell=True)
    print(f"デーモンをバックグラウンドで起動しました: {daemon_path}")
else:
    # Unix系の場合
    subprocess.Popen([daemon_path], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, preexec_fn=os.setsid)
    print(f"デーモンをバックグラウンドで起動しました: {daemon_path}")

print("10秒間待機します...")
time.sleep(10)

print("ポート8080のリスニング状態を確認します...")
# netstat コマンドを実行してポート8080のリスニング状態を確認
result = subprocess.run(["netstat", "-an"], capture_output=True, text=True)
if ":8080" in result.stdout:
    print("ポート3030がリッスンされています。")
else:
    print("ポート3030がリッスンされていません。")
    print("netstat stdout:", result.stdout)
    print("netstat stderr:", result.stderr)
