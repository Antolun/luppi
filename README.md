# Luppo Package Installer

**Luppo Package Installer** is a lightweight, modern Qt/QML-based Graphical User Interface (GUI) wrapper for the **Luppo package manager**.

It simplifies `.luppo` package installations by offering an intuitive interface with real-time log output, administrative privilege delegation, command-line argument handling, and desktop file-association integration.

---

## 📸 Features

- **Desktop Integration:** Double-click any `.luppo` package in your file manager to open and install it directly via Luppo Package Installer.
- **CLI Support:** Pass file paths as terminal arguments (e.g., `luppo-package-installer ./package.luppo`).
- **Privilege Management:** Safe execution using `pkexec` for elevated root privileges during installation.
- **Live Installation Output:** Real-time log streaming from standard output (`stdout`) to monitor progress and errors.
- **Internationalization (i18n):** Auto-detects system language with full support for **English** and **Turkish**.

---

## 🛠️ Prerequisites

To build and run Luppo Package Installer from source, ensure you have the following dependencies installed on your system:

- **Rust Toolchain:** `cargo` and `rustc` (1.70+)
- **Qt 6 / QML:** Development libraries for `qmetaobject` bindings
- **System Utilities:** `pkexec`, `luppo` package manager, and `shared-mime-info`

---

## 🚀 Building & Installing

```bash
# 1. Clone the Repository
git clone https://github.com/Antolun/luppo-package-installer.git
cd luppo-package-installer

# 2. Start Build
chmod +x ./build-luppo.sh
sudo ./build-luppo.sh

# 3. Install Package
sudo luppo it ./luppo-package-installer-*-x86_64.luppo
```
