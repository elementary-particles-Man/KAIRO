import sys
import json
from pathlib import Path

# Add nexus to path to allow imports
sys.path.append(str(Path.cwd()))

# json_daemonから必要な関数をインポート
from nexus.json_daemon import resolve_addresses

# resolve_addressesに渡すsettingsディクショナリ
settings = {"auto_detect_pids": True}

if __name__ == "__main__":
    # 関数を実行し、結果を整形して出力
    addresses = resolve_addresses(settings)
    print(json.dumps(addresses, indent=2, ensure_ascii=False))
