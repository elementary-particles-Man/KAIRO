from pathlib import Path
import sys

src_root = Path(__file__).resolve().parent
if str(src_root) not in sys.path:
    sys.path.insert(0, str(src_root))

helper_path = src_root / 'kairo_lib' / 'py'
if str(helper_path) not in sys.path:
    sys.path.insert(0, str(helper_path))

from .errors import *
from .log_recorder import *

__all__ = []
