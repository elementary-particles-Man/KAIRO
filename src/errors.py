class AuthenticationError(Exception):
    """認証失敗時に発生するエラー。"""
    def __init__(self, transaction_id, message):
        super().__init__(message)
        self.transaction_id = transaction_id

    def as_dict(self):
        return {"transaction_id": self.transaction_id, "message": str(self)}

class TimeoutError(Exception):
    """操作がタイムアウトした時に発生するエラー。"""
    def __init__(self, transaction_id, message):
        super().__init__(message)
        self.transaction_id = transaction_id

    def as_dict(self):
        return {"transaction_id": self.transaction_id, "message": str(self)}

class ConnectionLostError(Exception):
    """接続が失われた時に発生するエラー。"""
    def __init__(self, transaction_id, message):
        super().__init__(message)
        self.transaction_id = transaction_id

    def as_dict(self):
        return {"transaction_id": self.transaction_id, "message": str(self)}
