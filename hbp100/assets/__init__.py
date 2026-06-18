from importlib.resources import files

def get_asset_path(filename: str):
    return files("hbp100.assets") / filename
