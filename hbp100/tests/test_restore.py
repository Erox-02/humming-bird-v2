from hbp100 import restore, metadata_vault

def test_restore_single_placeholder():
metadata_vault.clear()

metadata_vault.add("[NAME_1]", "John Doe")

text = "Hello [NAME_1]"

restored = restore(text)

assert restored == "Hello John Doe"

def test_restore_multiple_placeholders():
metadata_vault.clear()

metadata_vault.add("[NAME_1]", "John Doe")
metadata_vault.add("[PHONE_1]", "9876543210")

text = "[NAME_1] number is [PHONE_1]"

restored = restore(text)

assert "John Doe" in restored
assert "9876543210" in restored