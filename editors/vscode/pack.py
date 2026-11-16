"""Packs this folder into uw-lang-<version>.vsix without npm/vsce (a .vsix is a zip with two
manifest files). Install: `code --install-extension editors/vscode/uw-lang-<version>.vsix`.

Run: python editors/vscode/pack.py   (from anywhere; writes next to this file)
"""
import json
import os
import zipfile

HERE = os.path.dirname(os.path.abspath(__file__))
# Only what the extension needs at run time; tests, the packer and old packages stay out.
SKIP_DIRS = {"test", "node_modules", ".vscode"}
SKIP_FILES = {"pack.py"}


def files():
    for root, dirs, names in os.walk(HERE):
        dirs[:] = sorted(d for d in dirs if d not in SKIP_DIRS and not d.startswith("."))
        for name in sorted(names):
            if name in SKIP_FILES or name.endswith(".vsix"):
                continue
            path = os.path.join(root, name)
            yield path, os.path.relpath(path, HERE).replace(os.sep, "/")


def main():
    with open(os.path.join(HERE, "package.json"), encoding="utf-8") as f:
        pkg = json.load(f)
    name, version, publisher = pkg["name"], pkg["version"], pkg["publisher"]
    display = pkg.get("displayName", name)
    description = pkg.get("description", "")
    engine = pkg["engines"]["vscode"]
    manifest = f"""<?xml version="1.0" encoding="utf-8"?>
<PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011">
  <Metadata>
    <Identity Language="en-US" Id="{name}" Version="{version}" Publisher="{publisher}"/>
    <DisplayName>{display}</DisplayName>
    <Description xml:space="preserve">{description.replace('&', '&amp;').replace('<', '&lt;')}</Description>
    <Categories>Programming Languages</Categories>
    <Properties>
      <Property Id="Microsoft.VisualStudio.Code.Engine" Value="{engine}"/>
    </Properties>
  </Metadata>
  <Installation>
    <InstallationTarget Id="Microsoft.VisualStudio.Code"/>
  </Installation>
  <Dependencies/>
  <Assets>
    <Asset Type="Microsoft.VisualStudio.Code.Manifest" Path="extension/package.json" Addressable="true"/>
  </Assets>
</PackageManifest>
"""
    content_types = """<?xml version="1.0" encoding="utf-8"?>
<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension=".json" ContentType="application/json"/>
  <Default Extension=".md" ContentType="text/markdown"/>
  <Default Extension=".js" ContentType="application/javascript"/>
  <Default Extension=".vsixmanifest" ContentType="text/xml"/>
</Types>
"""
    out = os.path.join(HERE, f"{name}-{version}.vsix")
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        z.writestr("extension.vsixmanifest", manifest)
        z.writestr("[Content_Types].xml", content_types)
        for path, rel in files():
            z.write(path, "extension/" + rel)
    print(out)


if __name__ == "__main__":
    main()
