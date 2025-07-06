#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import time
import json
import shutil
import uuid
import subprocess
from datetime import datetime
from pathlib import Path
import hashlib

# === 固定パス設定 ===
REPO_ROOT = Path("D:/dev/KAIRO")
CLI_INSTRUCTION = REPO_ROOT / "cli_instruction"
CLI_ARCHIVES = REPO_ROOT / "cli_archives"
NEW_TASK_JSON = CLI_INSTRUCTION / "new_task.json"
COMPLETE_FLAG = CLI_INSTRUCTION / "complete.flag"

# === 初期化 ===
CLI_ARCHIVES.mkdir(parents=True, exist_ok=True)

print(f"[INFO] KAIRO_Autoloop.py - Last updated: 2025-06-30 JST")
print(f"[INFO] 監視対象: {NEW_TASK_JSON}")

try:
    while True:
        if NEW_TASK_JSON.exists():
            print(f"\n[INFO] new_task.json 検知: {NEW_TASK_JSON}")

            # === タスク内容をロード ===
            with NEW_TASK_JSON.open('r', encoding='utf-8') as f:
                task_content = f.read()
                task_data = json.loads(task_content)
                task = task_data.get("tasks", [{}])[0]
                task_id = task.get("task_id", "unknown")
                local_exec = task.get("execution_target", {}).get("local_exec", None)

            # === ハッシュ生成 ===
            content_hash = hashlib.sha256(task_content.encode('utf-8')).hexdigest()
            task_uuid = str(uuid.uuid4())
            timestamp = datetime.now().astimezone().isoformat()

            # === コマンド実行 ===
            exec_status = "skipped"
            stdout_data = ""
            stderr_data = ""

            if local_exec:
                print(f"[INFO] 実行コマンド: {local_exec}")
                result = subprocess.run(local_exec, shell=True, capture_output=True, text=True)
                exec_status = "success" if result.returncode == 0 else "failed"
                stdout_data = result.stdout
                stderr_data = result.stderr

                print(f"[INFO] 実行結果: {exec_status}")
                if stdout_data:
                    print(f"[STDOUT]\n{stdout_data}")
                if stderr_data:
                    print(f"[STDERR]\n{stderr_data}")
            else:
                print("[WARN] local_exec が未設定です。コマンド実行をスキップします。")

            # === 完了フラグ生成 ===
            flag_content = {
                "task_id": task_id,
                "status": exec_status,
                "uuid": task_uuid,
                "timestamp": timestamp,
                "hash": content_hash,
                "stdout": stdout_data.strip(),
                "stderr": stderr_data.strip()
            }

            with COMPLETE_FLAG.open('w', encoding='utf-8') as f:
                json.dump(flag_content, f, indent=2, ensure_ascii=False)
            print(f"[INFO] complete.flag 生成: {COMPLETE_FLAG}")

            # === アーカイブ ===
            ts = datetime.now().strftime("%Y%m%d_%H%M%S")
            dest_json = CLI_ARCHIVES / f"new_task_{ts}.json"
            dest_flag = CLI_ARCHIVES / f"complete_{ts}.flag"

            shutil.move(str(NEW_TASK_JSON), dest_json)
            shutil.move(str(COMPLETE_FLAG), dest_flag)
            print(f"[INFO] アーカイブ完了: {dest_json.name}, {dest_flag.name}")

            # === Git Push ===
            subprocess.run(["git", "add", "."], cwd=REPO_ROOT)
            subprocess.run([
                "git", "commit",
                "-m", f"chore: Archive task {task_id} with complete.flag"
            ], cwd=REPO_ROOT)
            subprocess.run(["git", "push"], cwd=REPO_ROOT)
            print("[INFO] Git Push 完了")

        else:
            print(".", end="", flush=True)

        time.sleep(10)

except KeyboardInterrupt:
    print("\n[INFO] ユーザー停止: KAIRO_Autoloop.py 終了")
