#!/usr/bin/python3
from luppo.actionsapi import luppotools
from luppo.actionsapi import shelltools
import os

WorkDir = "."

def build():
    pass

def install():
    src_dir = os.environ.get("LUPPO_PACKAGE_INSTALLER_SRC_DIR", os.getcwd())

    possible_bins = [
        os.path.join(src_dir, "target/release/luppo-package-installer"),
        os.path.join(src_dir, "luppo-package-installer"),
        "target/release/luppo-package-installer",
        "luppo-package-installer",
    ]

    bin_path = None
    for p in possible_bins:
        if os.path.isfile(p):
            bin_path = p
            break

    if not bin_path:
        raise RuntimeError(f"luppo-package-installer binary not found in any path! Searched: {possible_bins}")

    luppotools.dobin(bin_path)

    desktop_path = os.path.join(src_dir, "luppo-package-installer.desktop")
    if not os.path.isfile(desktop_path):
        desktop_path = "luppo-package-installer.desktop"
    if os.path.isfile(desktop_path):
        luppotools.insinto("/usr/share/applications", desktop_path)

    logo_path = os.path.join(src_dir, "luppo-package-installer.png")
    if not os.path.isfile(logo_path):
        logo_path = "luppo-package-installer.png"
    if os.path.isfile(logo_path):
        luppotools.insinto("/usr/share/icons/hicolor/128x128/apps", logo_path, "luppo-package-installer.png")

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
