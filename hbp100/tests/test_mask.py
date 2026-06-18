

from hbp100 import mask, metadata_vault

def test_mask_returns_string():
text = "John Doe phone 9876543210"
masked = mask(text)

assert isinstance(masked, str)

def test_mask_creates_metadata():
metadata_vault.clear()

text = "John Doe phone 9876543210"
mask(text)

metadata = metadata_vault.get()

assert isinstance(metadata, dict)
assert len(metadata) > 0
