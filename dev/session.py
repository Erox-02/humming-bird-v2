from hbp100 import HBP100
import time

def test_session_persistence():
    engine = HBP100()
    
    # First call - creates NAME_1
    result1 = engine.process("Patient John Doe")
    print(f"First call: {result1['masked_text']}")
    print(f"Metadata: {result1.get('metadata', {})}")
    result2 = engine.process("Patient Jane Smith")
    print(f"Second call: {result2['masked_text']}")
    print(f"Metadata: {result2.get('metadata', {})}")
    print("\n Python session API not yet implemented")
    print("Need to add session support to Python bindings")

def test_stateless_behavior():
    engine = HBP100()

    r1 = engine.process("Alice")
    r2 = engine.process("Bob")
    r3 = engine.process("Charlie")
    print(f"Alice → {r1['masked_text']}")
    print(f"Bob → {r2['masked_text']}")
    print(f"Charlie → {r3['masked_text']}")

def benchmark_session():
    engine = HBP100()
    text = "Patient John Doe"
    
    start = time.time()
    for _ in range(100000):
        engine.process(text)
    elapsed = time.time() - start
    
    avg_ms = (elapsed / 100000) * 1000
    throughput = 100000 / elapsed
    
    print(f"100,000 iterations: {elapsed:.4f}s")
    print(f"Average: {avg_ms:.4f} ms/text")
    print(f"Throughput: {throughput:,.0f} texts/sec")

if __name__ == "__main__":
    test_stateless_behavior()
    test_session_persistence()
    benchmark_session()