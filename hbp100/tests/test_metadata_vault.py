from hbp100.placeholders.metadata_vault import metadata_vault

def test_add_and_get():
metadata_vault.clear()

metadata_vault.add("[NAME_1]", "John Doe")

data = metadata_vault.get()

assert data["[NAME_1]"] == "John Doe"

def test_clear():
metadata_vault.add("[PHONE_1]", "1234567890")
metadata_vault.clear()

assert metadata_vault.get() == {}
