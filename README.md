# PiSiPi — PiSi Package Installer

**PiSiPi** is a lightweight, modern Qt/QML-based Graphical User Interface (GUI) wrapper for the **PiSi package manager**.

It simplifies `.pisi` package installations by offering an intuitive interface with real-time log output, administrative privilege delegation, command-line argument handling, and desktop file-association integration.

---

## 📸 Features

- **Desktop Integration:** Double-click any `.pisi` package in your file manager to open and install it directly via PiSiPi.
- **CLI Support:** Pass file paths as terminal arguments (e.g., `pisipi ./package.pisi`).
- **Privilege Management:** Safe execution using `pkexec` for elevated root privileges during installation.
- **Live Installation Output:** Real-time log streaming from standard output (`stdout`) to monitor progress and errors.
- **Internationalization (i18n):** Auto-detects system language with full support for **English** and **Turkish**.

---

## 🛠️ Prerequisites

To build and run PiSiPi from source, ensure you have the following dependencies installed on your system:

- **Rust Toolchain:** `cargo` and `rustc` (1.70+)
- **Qt 6 / QML:** Development libraries for `qmetaobject` bindings
- **System Utilities:** `pkexec`, `pisi` package manager, and `shared-mime-info`

---

## 🚀 Building & Installing

```bash
# 1. Clone the Repository
git clone https://github.com/Antolun/pisipi.git
cd pisipi

# 2. Start Build
chmod +x ./build-pisi.sh
sudo ./build-pisi.sh

# 3. Install Package
sudo pisi it ./pisipi-*-x86_64.pisi
```
