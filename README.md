# My AstralRinth

A fork of AstralRinth (Modrinth launcher fork) — a Minecraft launcher with support for offline authentication.

## Releases

Release builds are produced by GitHub Actions for these platforms:

- **Linux:** `.AppImage` and `.deb`
- **Windows:** `.exe` installer and `.msi`
- **macOS:** universal `.dmg`

Push a `v*` or `build*` tag to build all three platforms and publish the artifacts to a GitHub Release. You can also start a build manually from the Actions tab; manual builds upload downloadable workflow artifacts but do not publish a release.

**Releases:** https://github.com/jhanter555/AstralRinth/releases

## Features

- Authentication with a valid Minecraft license and support for offline accounts.
- Automatic detection of the recommended Java version, with manual JVM configuration.
- Update notifications and built-in launcher updates.
- Discord Rich Presence (RPC) with randomized status messages, an in-game timer, and an AFK counter.
- Forced disabling of Modrinth telemetry.
- Advertisement removal across the launcher.
- Package and archive optimizations.
- Custom SVG interface elements.
- Support for both licensed and offline accounts.

## Purpose

AstralRinth was taken down, and the "proxy" repo containing it is using an old Releases page.
I made this repo not to claim the authorship or anything like that, but to fix a bug in the launcher that I experienced and to save people from compiling the launcher from source.
