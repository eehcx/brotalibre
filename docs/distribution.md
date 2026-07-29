# Distribution

BrotaLibre is released through Cargo, GitHub Release assets, and a Homebrew tap.

## Install

```bash
cargo install --locked brotalibre
sudo apt install ./brotalibre_<version>-1_amd64.deb
sudo dnf install ./brotalibre-<version>-1.x86_64.rpm
brew install eehcx/tap/brotalibre
```

GitHub Release assets include SHA-256 checksum files. Verify downloaded packages before installing them.

## Homebrew Tap

The public `eehcx/homebrew-tap` repository receives `Formula/brotalibre.rb` after the first fully configured release. Create a fine-grained GitHub token limited to `contents: write` on that tap and save it in this repository as `HOMEBREW_TAP_TOKEN` before releasing. The release workflow renders the formula from release checksums and commits it to the tap after GitHub Release assets are uploaded. If a formula update needs to be repaired, run the `Release` workflow manually with the existing release tag.

Release tags retain the legacy `ngseed-v<version>` format so Release Please can find the existing version history. Public assets and packages use the `brotalibre` name.

## AUR

AUR publication is intentionally deferred until a maintainer account and publication workflow are available.
