---
title: Install Nephclick Picshot
description: Download Nephclick Picshot for Windows 11, macOS or Linux, install it, and open your photo library.
order: 10
verified: 0.7.1
---

Download the file for your system from the [latest release](https://github.com/storytold/picshot/releases/latest) and install it. In the file names, `<version>` stands for the version number, for example `0.7.1`. The [Nephclick Picshot page on getartcraft.com](https://getartcraft.com/apps/picshot) also links the download.

## Windows 11

1. Open the [latest release](https://github.com/storytold/picshot/releases/latest).
2. Download `picshot-<version>-windows-x64.msi`. It fits most PCs (Intel and AMD, 64-bit).
3. Run the file. The first page asks who to install Nephclick Picshot for. **Only for me** is the default: it needs no administrator permission and installs to `%LOCALAPPDATA%\Programs\ArtCraft\Nephclick Picshot`. **Everyone on this computer** asks for an administrator and installs to `C:\Program Files\ArtCraft\Nephclick Picshot` (`Program Files (x86)` for the 32-bit build). Upgrades stay in the folder you chose.
4. At the end, choose the option to start Nephclick Picshot.

For a Snapdragon or other Arm PC, download `picshot-<version>-windows-arm64.msi` instead. For a 32-bit Windows, download `picshot-<version>-windows-x86.msi`.

Upgrading from an installer older than 0.6.0 also keeps Nephclick Picshot in the folder it was installed in. An upgrade uses the folder that the earlier install recorded and does not check that it still exists, so if you deleted that folder, the upgrade creates it again.

To run it without installing, download `picshot-<version>-windows-x64-portable.zip`. Portable builds for `arm64` and `x86` are on the release page too.

## macOS

1. Download `picshot-<version>-macos-universal.dmg`. It works on Apple silicon and Intel Macs, and is signed and notarized.
2. Open the file and install Nephclick Picshot.

## Linux

The release has these files for `x86_64` and `aarch64` (Arm). Replace `x86_64` with `aarch64` on Arm.

| File                                         | Notes                                                    |
|----------------------------------------------|----------------------------------------------------------|
| `picshot-<version>-linux-x86_64.AppImage` | Runs anywhere.                                           |
| `picshot-<version>-linux-x86_64.flatpak`  | Sandboxed. Install with `flatpak install --user <file>`. |
| `picshot-<version>-linux-x86_64.deb`      | For Debian and Ubuntu.                                   |
| `picshot-<version>-linux-x86_64.rpm`      | For Fedora, RHEL and openSUSE.                           |
| `picshot-<version>-linux-x86_64.tar.gz`   | Unpack anywhere.                                         |

RISC-V and FreeBSD builds are on the release page too. On a Wayland desktop, Nephclick Picshot runs through XWayland when it is available, because only X11 gives it file drag-and-drop.

## Install with ArtCraft Launcher

[ArtCraft Launcher](https://getartcraft.com/docs/craft-launcher) lists Nephclick Picshot and can install and update it for you. On Windows it installs the release `.msi`.

## In a web browser

The release also has `picshot-web-<version>.zip`, a static site that runs Nephclick Picshot in a modern browser. Host it on any static web server.

## First run

1. Start Nephclick Picshot. It opens your library at `~/Pictures/Nephclick Picshot Library`. A new library starts with demo photos.
2. To bring in your own photos, see [Import photos](../guides/import-photos.md).

## Still stuck?

Ask in [Discord](https://discord.gg/artcraft) or open an issue in the [issue tracker](https://github.com/storytold/picshot/issues).
