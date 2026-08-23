from hbp100 import HBP100
import time

engine = HBP100()
text = "Patient John Doe, MRN: 123456, Phone: 9876543210"

for _ in range(100):
    engine.process(text)

iterations = 100_000

start = time.perf_counter()

for _ in range(iterations):
    engine.process(text)

elapsed = time.perf_counter() - start

avg_ms = elapsed / iterations * 1000
throughput = iterations / elapsed

print(f"{iterations:,} iterations in {elapsed:.4f}s")
print(f"Average: {avg_ms:.4f} ms/text")
print(f"Throughput: {throughput:,.0f} texts/sec")