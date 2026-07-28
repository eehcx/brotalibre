#!/usr/bin/env bash
# ============================================================
# test-scaffold.sh — Test brotalibre project generation in isolated
# tempdir, validate structure, optionally build and serve.
#
# Usage:
#   ./scripts/test-scaffold.sh [options]
#
# Options:
#   --architecture <clean|ddd>     Architecture profile (default: clean)
#   --ui <material|primeng|none>   UI library (default: none)
#   --styles <tailwindcss|none>    Style framework (default: none)
#   --package-manager <npm|pnpm|yarn|bun>  (default: npm)
#   --skip-git                     Pass --skip-git to brota
#   --build                        Run npm install + ng build
#   --serve                        Run ng serve + screenshot (implies --build)
#   --keep                         Keep tempdir after test
#   --all                          Run all architecture × UI combinations
#   --quick                        Structure check only, skip npm
# ============================================================
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

# ── Colors ──────────────────────────────────────────────────
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

ok()   { echo -e "  ${GREEN}✓${NC} $1"; }
fail() { echo -e "  ${RED}✗${NC} $1"; ERRORS=$((ERRORS + 1)); }
info() { echo -e "  ${CYAN}→${NC} $1"; }
warn() { echo -e "  ${YELLOW}⚠${NC} $1"; }

ERRORS=0
TOTAL_TESTS=0
PASSED_TESTS=0

# Globals used by --serve mode (set by run_test, consumed by main)
SERVE_TMPDIR=""
SERVE_PROJECT_DIR=""

# ── Defaults ────────────────────────────────────────────────
ARCHITECTURE="clean"
UI="none"
STYLES="none"
PACKAGE_MANAGER="npm"
SKIP_GIT=""
DO_BUILD=false
DO_SERVE=false
KEEP=false
ALL=false

# ── Parse flags ────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case "$1" in
    --architecture)  ARCHITECTURE="$2"; shift 2 ;;
    --ui)            UI="$2"; shift 2 ;;
    --styles)        STYLES="$2"; shift 2 ;;
    --package-manager) PACKAGE_MANAGER="$2"; shift 2 ;;
    --skip-git)      SKIP_GIT="--skip-git"; shift ;;
    --build)         DO_BUILD=true; shift ;;
    --serve)         DO_SERVE=true; DO_BUILD=true; shift ;;
    --keep)          KEEP=true; shift ;;
    --all)           ALL=true; shift ;;
    --quick)         DO_BUILD=false; shift ;;
    *)               echo "Unknown flag: $1"; exit 1 ;;
  esac
done

# ── Serve cleanup trap (set up after BROTA_BIN build) ──────
cleanup_serve() {
  if [ -n "$SERVE_TMPDIR" ] && [ "$KEEP" = false ]; then
    rm -rf "$SERVE_TMPDIR"
    echo ""
    info "Tempdir cleaned"
  fi
}

# ── Build binary once ──────────────────────────────────────
BROTA_BIN="target/debug/brota"
if [ ! -f "$BROTA_BIN" ]; then
  info "Building brotalibre..."
fi
cargo build -q 2>/dev/null || cargo build
BROTA_BIN="$(cd "$REPO_ROOT" && pwd)/target/debug/brota"

# ── Validation helpers ─────────────────────────────────────
validate_clean() {
  local dir="$1"
  info "Validating Clean Architecture structure..."
  local e=0

  if [ -f "$dir/src/app/app.ts" ]; then
    ok "src/app/app.ts"
    grep -q "readonly title" "$dir/src/app/app.ts" 2>/dev/null \
      && ok "  has title property" \
      || fail "  missing title property"
  elif [ -f "$dir/src/app/app.component.ts" ]; then
    ok "src/app/app.component.ts"
    grep -q "readonly title" "$dir/src/app/app.component.ts" 2>/dev/null \
      && ok "  has title property" \
      || fail "  missing title property"
  else
    fail "app.ts or app.component.ts not found"
    e=1
  fi

  if [ -f "$dir/src/app/app.html" ]; then
    ok "src/app/app.html"
    grep -q "brotalibre" "$dir/src/app/app.html" 2>/dev/null \
      && ok "  has brotalibre branding" \
      || fail "  missing brotalibre branding"
  elif [ -f "$dir/src/app/app.component.html" ]; then
    ok "src/app/app.component.html"
    grep -q "brotalibre" "$dir/src/app/app.component.html" 2>/dev/null \
      && ok "  has brotalibre branding" \
      || fail "  missing brotalibre branding"
  else
    fail "app.html or app.component.html not found"
    e=1
  fi

  if [ -f "$dir/src/app/app.config.ts" ]; then
    ok "src/app/app.config.ts"
    grep -q "provideRouter" "$dir/src/app/app.config.ts" 2>/dev/null \
      && ok "  provideRouter present" \
      || fail "  provideRouter missing"
  else
    fail "app.config.ts not found"
    e=1
  fi

  [ "$e" -eq 0 ] && ok "Clean Architecture structure OK"
}

validate_ddd() {
  local dir="$1"
  info "Validating DDD Architecture structure..."
  local e=0

  if [ -d "$dir/src/app/features" ]; then
    ok "src/app/features/"
  else
    fail "missing src/app/features/"
    e=1
  fi

  for f in \
    "src/app/app.config.ts" \
    "src/app/app.ts" \
    "src/app/app.html"
  do
    if [ -f "$dir/$f" ]; then ok "$f"; else fail "missing $f"; e=1; fi
  done

  [ "$e" -eq 0 ] && ok "DDD Architecture structure OK"
}

validate_basics() {
  local dir="$1"
  info "Validating Angular basics..."
  local e=0

  for f in \
    "angular.json" "tsconfig.json" "package.json" \
    "src/main.ts" "src/index.html" "src/styles.scss"
  do
    if [ -f "$dir/$f" ]; then ok "$f"; else fail "missing $f"; e=1; fi
  done

  if [ -f "$dir/src/app/app.config.ts" ]; then
    ok "app.config.ts"
    if grep -q "provideRouter" "$dir/src/app/app.config.ts" 2>/dev/null; then
      ok "  provideRouter present"
    else
      fail "  provideRouter missing"
    fi
  else
    fail "missing app.config.ts"
  fi

  if [ -f "$dir/src/app/app.ts" ]; then
    if grep -q "styleUrl" "$dir/src/app/app.ts" 2>/dev/null; then
      ok "app.ts uses styleUrl (singular)"
    else
      fail "app.ts missing styleUrl"
    fi
  elif [ -f "$dir/src/app/app.component.ts" ]; then
    if grep -q "styleUrl" "$dir/src/app/app.component.ts" 2>/dev/null; then
      ok "app.component.ts uses styleUrl (singular)"
    else
      fail "app.component.ts missing styleUrl"
    fi
  else
    fail "app.ts nor app.component.ts found"
  fi
}

validate_material() {
  local dir="$1"
  grep -q '"@angular/material"' "$dir/package.json" 2>/dev/null \
    && ok "@angular/material in package.json" \
    || fail "@angular/material not in package.json"
}

validate_primeng() {
  local dir="$1"
  grep -q '"primeng"' "$dir/package.json" 2>/dev/null \
    && ok "primeng in package.json" \
    || fail "primeng not in package.json"
  grep -q '"primeicons"' "$dir/package.json" 2>/dev/null \
    && ok "primeicons in package.json" \
    || fail "primeicons not in package.json"
  grep -q "@primeng/themes" "$dir/angular.json" 2>/dev/null \
    && ok "@primeng/themes in angular.json" \
    || fail "@primeng/themes not in angular.json"
}

validate_tailwind() {
  local dir="$1"
  [ -f "$dir/postcss.config.js" ] && ok "postcss.config.js" || fail "missing postcss.config.js"
  grep -q "@tailwindcss/postcss" "$dir/postcss.config.js" 2>/dev/null \
    && ok "  @tailwindcss/postcss plugin configured" \
    || fail "  @tailwindcss/postcss plugin missing"
  grep -q "tailwindcss" "$dir/src/styles.scss" 2>/dev/null \
    && ok "styles.scss imports tailwindcss" \
    || fail "styles.scss missing @import \"tailwindcss\""
}

validate_no_git() {
  local dir="$1"
  [ ! -d "$dir/.git" ] && ok "no .git directory" || fail ".git exists but --skip-git was used"
}

validate_feature() {
  local dir="$1"
  local arch="$2"
  local feature="$3"
  info "Validating generated feature '$feature' ($arch)..."
  local e=0

  local base
  case "$arch" in
    clean) base="$dir/src/app" ;;
    ddd)   base="$dir/src/app/features/$feature" ;;
  esac

  for f in \
    "domain/${feature}.entity.ts" \
    "domain/${feature}-repository.port.ts" \
    "domain/${feature}.errors.ts" \
    "domain/value-objects/${feature}-id.vo.ts" \
    "application/${feature}.store.ts" \
    "application/getall-${feature}.use-case.ts" \
    "application/getbyid-${feature}.use-case.ts" \
    "application/create-${feature}.use-case.ts" \
    "application/update-${feature}.use-case.ts" \
    "application/delete-${feature}.use-case.ts" \
    "infrastructure/dto/${feature}.request.dto.ts" \
    "infrastructure/dto/${feature}.response.dto.ts" \
    "infrastructure/mappers/${feature}.mapper.ts" \
    "infrastructure/${feature}.repository.ts" \
    "infrastructure/${feature}.provider.ts"
  do
    if [ -f "$base/$f" ]; then ok "$f"; else fail "missing $f"; e=1; fi
  done

  [ "$e" -eq 0 ] && ok "Feature '$feature' structure OK ($arch)"
}

# ── Run single test combo ──────────────────────────────────
run_test() {
  local arch="$1"
  local ui="$2"
  local styles="$3"
  local pm="$4"
  local skip_git="$5"
  local build="$6"
  local serve="$7"

  TOTAL_TESTS=$((TOTAL_TESTS + 1))

  local TMPDIR
  TMPDIR=$(mktemp -d)
  local TS=$(date +%s)
  local PROJECT_NAME="brotalibre-${arch}-${ui}-${styles}"
  local PROJECT_DIR="$TMPDIR/$PROJECT_NAME"

  # Symlink templates so the binary finds them via CWD
  ln -s "$REPO_ROOT/templates" "$TMPDIR/templates"

  echo ""
  echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"
  echo -e "${CYAN}  Test: arch=${arch}  ui=${ui}  styles=${styles}${NC}"
  echo -e "${CYAN}  Temp: ${PROJECT_DIR}${NC}"
  echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"

  # ── Generate (run from TMPDIR so project lands in temp) ──
  local BROTA_ARGS=("new" "$PROJECT_NAME" "--yes"
    "--architecture" "$arch"
    "--ui" "$ui"
    "--styles" "$styles"
    "--package-manager" "$pm"
    "--skip-install"
  )
  [ -n "$skip_git" ] && BROTA_ARGS+=("$skip_git")

  info "Running: brota ${BROTA_ARGS[*]}"
  if ! (cd "$TMPDIR" && "$BROTA_BIN" "${BROTA_ARGS[@]}") > /dev/null 2>&1; then
    fail "brota new command failed"
    [ "$KEEP" = false ] && rm -rf "$TMPDIR"
    return
  fi
  ok "brota new completed"

  if [ ! -d "$PROJECT_DIR" ]; then
    fail "Project directory not created at $PROJECT_DIR"
    [ "$KEEP" = false ] && rm -rf "$TMPDIR"
    return
  fi

  # ── Validate structure ──
  validate_basics "$PROJECT_DIR"

  case "$arch" in
    clean) validate_clean "$PROJECT_DIR" ;;
    ddd)   validate_ddd   "$PROJECT_DIR" ;;
  esac

  case "$ui" in
    material) validate_material "$PROJECT_DIR" ;;
    primeng)  validate_primeng  "$PROJECT_DIR" ;;
  esac

  case "$styles" in
    tailwindcss) validate_tailwind "$PROJECT_DIR" ;;
  esac

  [ -n "$skip_git" ] && validate_no_git "$PROJECT_DIR"

  # ── Generate feature (validates brota generate feature subcommand) ──
  # Run from $TMPDIR (where templates/ symlink lives) and pass --project-dir
  local FEATURE_NAME="test-entity"
  local GEN_ARGS=("generate" "feature" "$FEATURE_NAME"
    "--architecture" "$arch"
    "--prefix" "api"
    "--fields" "name:string,age:number,email:string"
    "--project-dir" "$PROJECT_DIR"
  )
  info "Running: brota ${GEN_ARGS[*]}"
  if ! (cd "$TMPDIR" && "$BROTA_BIN" "${GEN_ARGS[@]}") > /dev/null 2>&1; then
    fail "brota generate feature failed"
    [ "$KEEP" = false ] && rm -rf "$TMPDIR"
    return
  fi
  ok "brota generate feature completed"
  validate_feature "$PROJECT_DIR" "$arch" "$FEATURE_NAME"

  # ── Build (optional) ──
  if [ "$build" = true ]; then
    info "Running npm install (legacy-peer-deps)..."
    if (cd "$PROJECT_DIR" && npm install --silent --legacy-peer-deps) > /dev/null 2>&1; then
      ok "npm install completed"
    else
      fail "npm install failed"
      [ "$KEEP" = false ] && rm -rf "$TMPDIR"
      return
    fi

    info "Running ng build..."
    if (cd "$PROJECT_DIR" && npx ng build) > /dev/null 2>&1; then
      ok "ng build succeeded"
    else
      fail "ng build failed"
      [ "$KEEP" = false ] && rm -rf "$TMPDIR"
      return
    fi
  fi

  # ── Result ──
  if [ $ERRORS -eq 0 ]; then
    PASSED_TESTS=$((PASSED_TESTS + 1))
  fi

  # ── Cleanup (or handoff to --serve) ──
  if [ "$serve" = true ]; then
    SERVE_TMPDIR="$TMPDIR"
    SERVE_PROJECT_DIR="$PROJECT_DIR"
    echo ""
    echo -e "  ${YELLOW}⏸${NC} Handing off to serve mode"
  elif [ "$KEEP" = true ]; then
    echo ""
    echo -e "  ${YELLOW}⏸${NC} Keeping test at: $PROJECT_DIR"
  else
    rm -rf "$TMPDIR"
    info "Tempdir cleaned"
  fi
}

# ── Run tests ──────────────────────────────────────────────
if [ "$ALL" = true ]; then
  if [ "$DO_SERVE" = true ]; then
    warn "Ignoring --serve in --all mode (would run 12 servers). Use a single combo."
  fi
  for arch in clean ddd; do
    for ui in none material primeng; do
      for styles in none tailwindcss; do
        run_test "$arch" "$ui" "$styles" "$PACKAGE_MANAGER" "$SKIP_GIT" "$DO_BUILD" "$DO_SERVE"
      done
    done
  done
else
  run_test "$ARCHITECTURE" "$UI" "$STYLES" "$PACKAGE_MANAGER" "$SKIP_GIT" "$DO_BUILD" "$DO_SERVE"
fi

# ── Serve (foreground) ────────────────────────────────────
if [ "$DO_SERVE" = true ] && [ -n "$SERVE_PROJECT_DIR" ] && [ $ERRORS -eq 0 ]; then
  trap cleanup_serve EXIT

  echo ""
  echo -e "${CYAN}╔══════════════════════════════════════════════════════╗${NC}"
  echo -e "${CYAN}║  App ready — open in your browser                  ║${NC}"
  echo -e "${CYAN}║                                                    ║${NC}"
  echo -e "${CYAN}║  ${YELLOW}http://localhost:4200/${NC}                          ${CYAN}║${NC}"
  echo -e "${CYAN}║                                                    ║${NC}"
  echo -e "${CYAN}║  Press Ctrl+C here when done (tempdir cleaned)     ║${NC}"
  echo -e "${CYAN}╚══════════════════════════════════════════════════════╝${NC}"
  echo ""

  cd "$SERVE_PROJECT_DIR"
  npx ng serve --host 0.0.0.0
  exit 0
fi

# ── Summary (no serve mode) ────────────────────────────────
echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"
echo -e "${CYAN}  Summary: ${PASSED_TESTS}/${TOTAL_TESTS} test(s) passed, ${ERRORS} error(s)${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"
exit $([ $ERRORS -eq 0 ] && echo 0 || echo 1)
