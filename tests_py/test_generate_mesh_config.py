import subprocess
import sys
from pathlib import Path
import toml

SCRIPT = Path(__file__).resolve().parents[1] / "scripts" / "generate_mesh_config.py"


def test_cli_writes_config(tmp_path):
    out_dir = tmp_path
    subprocess.run([sys.executable, str(SCRIPT), "relay", "--out-dir", str(out_dir)], check=True)
    conf = toml.loads((out_dir / "relay.toml").read_text())
    assert conf["role"] == "relay"
    assert "forward" in conf["permissions"]
