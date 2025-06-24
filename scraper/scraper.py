import os
import requests
import time

# Make folders if they don't exist
os.makedirs("images/normal", exist_ok=True)
os.makedirs("images/shiny", exist_ok=True)

# Load names from list.txt
with open("list.txt", "r", encoding="utf-8") as f:
    names = [line.strip() for line in f if line.strip()]


# Helper to normalize name into URL format
def normalize_name(name_str: str) -> str:
    return name_str.lower().replace(" ", "-").replace(".", "").replace("'", "").replace("♀", "-f").replace("♂", "-m")


# Download function
def download(mon_name):
    url_name = normalize_name(mon_name)

    urls = {
        "normal": f"https://img.pokemondb.net/sprites/ruby-sapphire/normal/{url_name}.png",
        "shiny": f"https://img.pokemondb.net/sprites/ruby-sapphire/shiny/{url_name}.png"
    }

    for variant, url in urls.items():
        try:
            res = requests.get(url)
            if res.status_code == 200:
                filename = f"images/{variant}/{mon_name}.png"
                with open(filename, "wb") as f:
                    f.write(res.content)
                print(f"✅ Downloaded {variant}: {mon_name}")
            else:
                print(f"❌ Not found {variant}: {mon_name}")
        except Exception as e:
            print(f"⚠ Error downloading {variant} {mon_name}: {e}")


# Iterate through all mons
for name in names:
    download(name)
    time.sleep(0.5)
