import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

from src.errors import AuthenticationError, TimeoutError, ConnectionLostError


def test_error_contains_transaction_id():
    err = AuthenticationError("tx1", "bad creds")
    assert err.transaction_id == "tx1"
    assert "bad creds" in str(err)


def test_as_dict():
    err = TimeoutError("abc", "timeout")
    info = err.as_dict()
    assert info["transaction_id"] == "abc"
    assert info["message"] == "timeout"


def test_connection_lost_error():
    err = ConnectionLostError("xyz", "lost")
    assert isinstance(err, ConnectionLostError)
    assert err.transaction_id == "xyz"

