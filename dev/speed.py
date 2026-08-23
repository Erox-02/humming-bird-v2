from hbp100 import HBP100
import time
engine = HBP100()
engine.process("Test")
iterations = 100
text = "Patient John Doe, MRN: 123456, Phone: 9876543210"

start = time.time()
for _ in range(iterations):
    engine.process(text)
end = time.time()

print(f"{iterations} iterations in {end-start:.2f}s")
print(f"Average: {(end-start)/iterations*1000:.2f}ms per text")