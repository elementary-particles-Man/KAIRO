class KairoError(Exception):
    """Base class for KAIRO developer-facing errors."""

    def __init__(self, transaction_id: str, message: str) -> None:
        super().__init__(message)
        self.transaction_id = transaction_id
        self.message = message

    def as_dict(self) -> dict:
        return {
            "transaction_id": self.transaction_id,
            "error": self.__class__.__name__,
            "message": self.message,
        }


class AuthenticationError(KairoError):
    """Raised when authentication fails."""


class TimeoutError(KairoError):
    """Raised when an operation times out."""


class ConnectionLostError(KairoError):
    """Raised when the connection is lost."""


class _CryptographicError(KairoError):
    """Internal error for cryptographic failures."""
