import os

# Path to your sprites folder
base_path = "./images"

# Walk through all subdirectories
for root, dirs, files in os.walk(base_path):
    for filename in files:
        old_path = os.path.join(root, filename)
        new_filename = filename.lower()
        new_path = os.path.join(root, new_filename)
        if old_path != new_path:
            os.rename(old_path, new_path)
            print(f"Renamed: {filename} -> {new_filename}")
