import dash

name = "rustup"
url = f"https://sh.{name}.rs"

def install():
	print(f"Installing {name}...")
	dash.get(url=url, file=f"{name}-init.sh")

install()