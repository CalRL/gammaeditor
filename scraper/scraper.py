import os
import requests
import time

# Make folders if they don't exist
os.makedirs("images/normal", exist_ok=True)
os.makedirs("images/shiny", exist_ok=True)

file = input("Enter the file name (no extension) ")

# Load names from file
with open(f"{file.strip()}.txt", "r", encoding="utf-8") as f:
    names = [line.strip() for line in f if line.strip()]


# Helper to normalize name into URL format
def normalize_name(name_str: str) -> str:
    return name_str.lower().replace(" ", "-").replace(".", "").replace("'", "").replace("♀", "-f").replace("♂", "-m")


# Download function
def download(mon_name):
    generation = "ruby-sapphire"
    if "/" in mon_name:
        arr = mon_name.split("/")
        mon_name = arr[1]
        generation = arr[0]

    url_name = normalize_name(mon_name)

    urls = {
        "normal": f"https://img.pokemondb.net/sprites/{generation}/normal/{url_name}.png",
        "shiny": f"https://img.pokemondb.net/sprites/{generation}/shiny/{url_name}.png"
    }

    for variant, url in urls.items():
        try:
            res = requests.get(url)
            if res.status_code == 200:
                filename = f"images/{variant}/{mon_name}.png"
                with open(filename, "wb") as f:
                    f.write(res.content)
                print(f"Downloaded: {Bcolors.GREEN} {mon_name} {Bcolors.ENDC} {Bcolors.YELLOW if variant == 'shiny' else Bcolors.ENDC}{variant}{Bcolors.ENDC} ({generation})")
            else:
                print(f"Not found {variant}: {mon_name} ({generation})")
        except Exception as e:
            print(f"⚠ Error downloading {variant} {mon_name}: {e}")


class Bcolors:
    GREEN = '\033[92m'
    ENDC = '\033[0m'
    YELLOW = '\033[33m'


# Iterate through all mons
print("Download starting")
for name in names:
    download(name)


print("Download Finished.")
