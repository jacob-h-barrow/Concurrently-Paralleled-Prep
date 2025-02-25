import threading
import os
from dataclasses import dataclass, field
from datetime import datetime
from typing import Any, Optional

type LockType = threading.Lock | threading.RLock

@dataclass
class TypedLock:
    """Encapsulates a lock with metadata."""
    lock: LockType
    lock_type: str
    value: Any = None
    
    pid_made: int = field(default_factory=os.getpid)
    created: datetime = field(default_factory=datetime.now)

class AtomicVariable:
    """Provides an atomic variable with compare-and-swap functionality."""

    @staticmethod
    def get_lock(lock: Optional[LockType] = None, value: Any = None) -> TypedLock:
        """Creates and returns a TypedLock object with the appropriate lock type."""
        if lock is None:
            lock = threading.RLock()

        if isinstance(lock, type(threading.RLock())):
            return TypedLock(lock, 'RLock', value)
        elif isinstance(lock, type(threading.Lock())):
            return TypedLock(lock, 'Lock', value)
        else:
            raise ValueError(f'{type(lock)} is not supported!')

    @staticmethod
    def acquire_lock(typed_lock: TypedLock, timeout: float) -> Optional[LockType]:
        """Tries to acquire the given lock within the timeout period."""
        if typed_lock.lock.acquire(timeout=timeout):
            return typed_lock.lock
        return None

    def __init__(self, value: Any = None, lock: Optional[LockType] = None, timeout: float = 5.5):
        """Initializes the atomic variable with a lock and a default timeout."""
        self._timeout = timeout
        self._typed_lock = self.get_lock(lock, value)

    def compare_and_exchange(self, expected: Any, new_value: Any, timeout: Optional[float] = None) -> int:
        """Atomically sets the value to new_value if it is currently expected.
        
        Returns:
            -1 if the lock couldn't be acquired.
             0 if the expected value was different.
             1 if the exchange was successful.
        """
        res = -1
        lock = self.acquire_lock(self._typed_lock, timeout if timeout else self._timeout)
        
        if lock:
            if self._typed_lock.value == expected:
                self._typed_lock.value = new_value
                res = 1
            else:
                res = 0
                
            lock.release()
            
        return res

    def get(self, timeout: Optional[float] = None) -> Any:
        """Safely retrieves the current value."""
        res = None
        lock = self.acquire_lock(self._typed_lock, timeout if timeout else self._timeout)
        
        if lock:
            res = self._typed_lock.value
            lock.release()
            
        return res

    def set(self, new_value: Any, timeout: Optional[float] = None) -> int:
        """Atomically sets the value to new_value.
        
        Returns:
            -1 if the lock couldn't be acquired.
             1 if the value was successfully set.
        """
        res = -1
        lock = self.acquire_lock(self._typed_lock, timeout if timeout else self._timeout)
        
        if lock:
            self._typed_lock.value = new_value
            res = 1
            lock.release()
            
        return res

