#!/usr/bin/python3
from luppo.actionsapi import luppotools
from luppo.actionsapi import shelltools
import os

WorkDir = "."

def build():
    pass

def install():
    src_dir = os.environ.get("LUPPI_SRC_DIR", os.getcwd())

    possible_bins = [
        os.path.join(src_dir, "target/release/luppi"),
        os.path.join(src_dir, "luppi"),
        "target/release/luppi",
        "luppi",
    ]

    bin_path = None
    for p in possible_bins:
        if os.path.isfile(p):
            bin_path = p
            break

    if not bin_path:
        raise RuntimeError(f"luppi binary not found in any path! Searched: {possible_bins}")

    luppotools.dobin(bin_path)

    desktop_path = os.path.join(src_dir, "luppi.desktop")
    if not os.path.isfile(desktop_path):
        desktop_path = "luppi.desktop"
    if os.path.isfile(desktop_path):
        luppotools.insinto("/usr/share/applications", desktop_path)

    logo_path = os.path.join(src_dir, "luppi.png")
    if not os.path.isfile(logo_path):
        logo_path = "luppi.png"
    if os.path.isfile(logo_path):
        luppotools.insinto("/usr/share/icons/hicolor/128x128/apps", logo_path, "luppi.png")

    readme_path = os.path.join(src_dir, "README.md")
    if not os.path.isfile(readme_path):
        readme_path = "README.md"
    if os.path.isfile(readme_path):
        luppotools.dodoc(readme_path)

    license_path = os.path.join(src_dir, "LICENSE")
    if not os.path.isfile(license_path):
        license_path = "LICENSE"
    if os.path.isfile(license_path):
        luppotools.dodoc(license_path)
