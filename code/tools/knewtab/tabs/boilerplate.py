from tabs.icons import Icon


def google_sheet(spreadsheet_id: str) -> str:
    return f"https://docs.google.com/spreadsheets/d/{spreadsheet_id}"


HEADER = """
<!DOCTYPE HTML>
<html>
  <head>
    <meta charset="utf-8" />
    <title>New Tab</title>
    <link rel="stylesheet" href="newtab.css" />
    <link rel="icon" href="icons/icon_16.png" />
  </head>
  <body>
    <div id="main">
      <div id="container">
"""

FOOTER = """
      </div>
    </div>
  </body>
</html>
"""


def sanitize_url(url: str):
    url = url.removeprefix("http://")
    url = url.removeprefix("https://")
    url = url.removesuffix("/")
    return "https://" + url + "/"


class Config:
    def __init__(self) -> None:
        self.data: list[tuple[str, str, str]] = []

    def __setitem__(self, name, value: tuple[str, Icon]):
        url, icon = value
        print(url)
        url2 = sanitize_url(url)
        print(url, "->", url2)
        self.data.append((name, url2, icon))

    def save(self, path: str):
        with open(path, "w") as f:
            p = lambda *v: print(*v, file=f)
            p(HEADER.strip())
            for name, url, icon in self.data:
                p(f'<a href="{url}" class="link">')
                p(f'<img class="icon" src="icons/feather/{icon}.svg"/>')
                p(name)
                p("</a>")
            print(FOOTER.strip(), file=f)


def update_manifest_version(version: str):
    import json

    with open("manifest.json", "r") as f:
        data = json.load(f)

    version = version.strip()
    print("Update manifest to version:", version)
    data["version"] = version

    with open("manifest.json", "w") as f:
        json.dump(data, f, indent=4)
