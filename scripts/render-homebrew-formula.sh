#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -ne 5 ]; then
  printf 'usage: %s <version> <tag> <arm-sha256> <intel-sha256> <output>\n' "$0" >&2
  exit 64
fi

version="$1"
tag="$2"
arm_sha256="$3"
intel_sha256="$4"
output="$5"

if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  printf 'invalid version: %s\n' "$version" >&2
  exit 64
fi

if [[ ! "$tag" =~ ^(ngseed|brotalibre)-v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  printf 'invalid release tag: %s\n' "$tag" >&2
  exit 64
fi

if [[ ! "$arm_sha256" =~ ^[[:xdigit:]]{64}$ ]] || [[ ! "$intel_sha256" =~ ^[[:xdigit:]]{64}$ ]]; then
  printf '%s\n' 'release checksums must be 64 hexadecimal characters' >&2
  exit 64
fi

mkdir -p "$(dirname "$output")"

cat > "$output" <<FORMULA
class Brotalibre < Formula
  desc "Scaffold Angular projects with Clean Architecture or DDD"
  homepage "https://github.com/eehcx/brotalibre"
  version "$version"
  license "GPL-3.0-only"

  depends_on :macos

  on_macos do
    on_arm do
      url "https://github.com/eehcx/brotalibre/releases/download/$tag/brotalibre-v$version-aarch64-apple-darwin.tar.gz"
      sha256 "$arm_sha256"
    end

    on_intel do
      url "https://github.com/eehcx/brotalibre/releases/download/$tag/brotalibre-v$version-x86_64-apple-darwin.tar.gz"
      sha256 "$intel_sha256"
    end
  end

  def install
    bin.install "brota"
  end

  test do
    project = testpath/"sample-app"
    (project/"src/app").mkpath
    (project/"package.json").write "{\"dependencies\": {}}\n"

    system bin/"brota", "generate", "feature", "brew-widget",
           "--architecture", "clean",
           "--prefix", "/api/widgets",
           "--fields", "name:string,price:number",
           "--project-dir", project.to_s

    assert_path_exists project/"src/app/domain/brew-widget.entity.ts"
    assert_match "export interface BrewWidgetProps", (project/"src/app/domain/brew-widget.entity.ts").read
    assert_match "@ngrx/signals", (project/"package.json").read
  end
end
FORMULA
