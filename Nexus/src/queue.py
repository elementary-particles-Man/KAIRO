from collections import deque
from typing import Deque, Optional


class StakQueue:
    def __init__(self) -> None:
        self._q: Deque[str] = deque()

    def push(self, item: str) -> None:
        self._q.append(item)

    def pop(self) -> Optional[str]:
        return self._q.popleft() if self._q else None

    def __len__(self) -> int:  # for len()
        return len(self._q)

    def peek_all(self) -> list[str]:
        return list(self._q)

    def clear(self) -> None:
        self._q.clear()

