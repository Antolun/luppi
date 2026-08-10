#!/usr/bin/python3
from pisi.actionsapi import pisitools
from pisi.actionsapi import shelltools
import os

WorkDir = "."

def build():
    pass

def install():
    src_dir = os.environ.get("PISIPI_SRC_DIR", "/home/solzic0/Projeler/LupuS/pisi-package-installer")

    possible_bins = [
        os.path.join(src_dir, "target/release/pisipi"),
        os.path.join(src_dir, "pisipi"),
        "target/release/pisipi",
        "pisipi",
    ]

    bin_path = None
    for p in possible_bins:
        if os.path.isfile(p):
            bin_path = p
            break

    if not bin_path:
        raise RuntimeError(f"pisipi binary not found in any path! Searched: {possible_bins}")

    pisitools.dobin(bin_path)

    desktop_path = os.path.join(src_dir, "pisipi.desktop")
    if not os.path.isfile(desktop_path):
        desktop_path = "pisipi.desktop"
    if os.path.isfile(desktop_path):
        pisitools.insinto("/usr/share/applications", desktop_path)

    logo_path = os.path.join(src_dir, "pisipi.png")
    if not os.path.isfile(logo_path):
        logo_path = "pisipi.png"
    if os.path.isfile(logo_path):
        pisitools.insinto("/usr/share/icons/hicolor/128x128/apps", logo_path, "pisipi.png")

    readme_path = os.path.join(src_dir, "README.md")
    if not os.path.isfile(readme_path):
        readme_path = "README.md"
    if os.path.isfile(readme_path):
        pisitools.dodoc(readme_path)

    license_path = os.path.join(src_dir, "LICENSE")
    if not os.path.isfile(license_path):
        license_path = "LICENSE"
    if os.path.isfile(license_path):
        pisitools.dodoc(license_path)
