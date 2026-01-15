# SpyglassMC Zed
Minecraft Datapack language support for [Zed](https://zed.dev) powered by [Spyglass](https://github.com/SpyglassMC/Spyglass).

## Features
- **Automated Updates**: This extension automatically checks for and downloads the latest `server.js` from GitHub Releases every 24 hours.
- **Offline Support**: If an update fails (e.g., no internet), it will continue to use the last successfully downloaded version.
- **Full Support**: Provides completions, validation, and more for `.mcfunction`.

## Installation (Development)
1. Clone this repository.
2. Open Zed and run `dev: open extensions`.
3. Select this repository folder.

## Technical Details
This extension uses a WASM-based host to manage the SpyglassMC Language Server. The server is bundled using `@vercel/ncc` via GitHub Actions to ensure a lightweight and reliable experience on Windows, macOS, and Linux.

## Credits
- Language Server: [SpyglassMC](https://github.com/SpyglassMC/Spyglass)
- tree-sitter-mcfunction :[IoeCmcommc](https://github.com/IoeCmcomc/tree-sitter-mcfunction)

---