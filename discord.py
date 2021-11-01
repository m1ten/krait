class Package:
    def __init__(self, name = None, version = None, url = None, license = None):
        self.name = name
        self.version = version
        self.url = url
        self.license = license

    def __str__(self):
        return str(self.__dict__)

    def __repr__(self):
        return str(self.__dict__)

discord = Package(
    name="discord",
    version="1.0.0",
    url="discord.com/download"
)
