from bs4 import BeautifulSoup
from bs4.element import PageElement, Tag
import re
from os import makedirs, path, walk

__cwd__ = path.dirname(__file__)
__root__ = __cwd__


xhtml_dir = path.join(__root__, "xhtml")
pretty_dir = path.join(__root__, "pretty")

xhtml_files = []
for root, _, files in walk(xhtml_dir):
    files = map(lambda f: path.join(root, f), files)
    files = filter(lambda f: f.endswith("html"), files)
    xhtml_files.extend(files)

re_structures = re.compile("^STRUCTURE")


class Child:
    node: PageElement
    children: list["Child"] | None

    def __init__(self, node: PageElement) -> None:
        self.node = node
        if type(node) == Tag:
            self.children = list(map(Child, node.children))
        else:
            self.children = None

    def print_struct(self) -> bool:
        """
        Returns True if the print occurred.
        """
        text = self.node.text
        want_to_print = "struct" in text and "{" in text and "}" in text
        if not want_to_print:
            return False
        # We do want to print a subchild of this current node, possibly itself.
        if self.children is not None:
            for child in self.children:
                if child.print_struct():
                    return True
        print(text)
        return True


href = re.compile('href="[A-Za-z0-9:/.#-_]*"')
see_also = re.compile("<h2.*SEE ALSO")


for file in xhtml_files:
    outfile = path.join(pretty_dir, path.basename(file))

    with open(file, "r") as f:
        text = f.read()
    text = re.sub(href, "", text)
    text = re.split(see_also, text, maxsplit=1)[0]
    makedirs(path.dirname(outfile), exist_ok=True)
    with open(outfile, "w") as f:
        f.write(text.strip())

    #     soup = BeautifulSoup(text, features="html.parser")
    # for tag in soup.descendants:
    #     if type(tag) == Tag:
    #         if "href" in tag.attrs:
    #             del tag.attrs["href"]
    #
    # with open(file, "w") as f:
    #     f.write(soup.prettify())
    #
    # for child in map(Child, soup.children):
    #     if child.print_struct():
    #         print("---------------------------------")
