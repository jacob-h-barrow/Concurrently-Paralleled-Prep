import threading
import unittest

from atomic_variable import AtomicVariable

class TestAtomicVariable(unittest.TestCase):
    def test_get_set(self):
        """Test basic get and set functionality."""
        atomic_var = AtomicVariable(10)
        self.assertEqual(atomic_var.get(), 10)
        atomic_var.set(20)
        self.assertEqual(atomic_var.get(), 20)
    
    def test_compare_and_exchange_success(self):
        """Test successful compare-and-exchange operation."""
        atomic_var = AtomicVariable(10)
        result = atomic_var.compare_and_exchange(10, 30)
        self.assertEqual(result, 1)
        self.assertEqual(atomic_var.get(), 30)
    
    def test_compare_and_exchange_failure(self):
        """Test compare-and-exchange when expected value does not match."""
        atomic_var = AtomicVariable(10)
        result = atomic_var.compare_and_exchange(5, 30)
        self.assertEqual(result, 0)
        self.assertEqual(atomic_var.get(), 10)
    
    def test_concurrent_compare_and_exchange(self):
        """Test concurrent compare-and-exchange operations."""
        atomic_var = AtomicVariable(0)
        success_count = 0
        
        def worker():
            nonlocal success_count
            if atomic_var.compare_and_exchange(0, 1) == 1:
                success_count += 1
        
        threads = [threading.Thread(target=worker) for _ in range(10)]
        for t in threads:
            t.start()
        for t in threads:
            t.join()
        
        self.assertEqual(success_count, 1)
        self.assertEqual(atomic_var.get(), 1)

if __name__ == "__main__":
    unittest.main()
