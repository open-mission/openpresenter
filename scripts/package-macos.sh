#!/usr/bin/env bash
set -e

# ==============================================================================
# Script de Empacotamento Oficial para macOS — OpenPresenter Minimal
# ==============================================================================

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

VERSION="0.4.1"
APP_NAME="OpenPresenter"
BUNDLE_DIR="$ROOT_DIR/dist/macos/${APP_NAME}.app"
CONTENTS_DIR="$BUNDLE_DIR/Contents"
MACOS_DIR="$CONTENTS_DIR/MacOS"
RESOURCES_DIR="$CONTENTS_DIR/Resources"

echo "==> Compilando frontend Svelte em produção..."
(cd frontend && npm run build)

echo "==> Compilando binário Rust em modo release..."
cargo build --release --package op-admin

BIN_SOURCE="$ROOT_DIR/target/release/open-presenter"
if [ ! -f "$BIN_SOURCE" ]; then
    echo "Erro: Binário não encontrado em $BIN_SOURCE"
    exit 1
fi

echo "==> Limpando diretório de distribuição anterior..."
rm -rf "$ROOT_DIR/dist/macos"
mkdir -p "$MACOS_DIR"
mkdir -p "$RESOURCES_DIR"

echo "==> Instalando binário no bundle..."
cp "$BIN_SOURCE" "$MACOS_DIR/open-presenter"
chmod +x "$MACOS_DIR/open-presenter"

echo "==> Copiando ícone oficial (.icns)..."
cp "$ROOT_DIR/assets/branding/open-presenter-app-icon.icns" "$RESOURCES_DIR/open-presenter-app-icon.icns"

echo "==> Gerando Info.plist..."
cat << EOF > "$CONTENTS_DIR/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDevelopmentRegion</key>
    <string>pt-BR</string>
    <key>CFBundleDisplayName</key>
    <string>OpenPresenter</string>
    <key>CFBundleExecutable</key>
    <string>open-presenter</string>
    <key>CFBundleIconFile</key>
    <string>open-presenter-app-icon.icns</string>
    <key>CFBundleIdentifier</key>
    <string>org.openpresenter.minimal</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>OpenPresenter</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>NSHumanReadableCopyright</key>
    <string>OpenPresenter — GPL-3.0-only</string>
</dict>
</plist>
EOF

echo "==> Aplicando ícone customizado no próprio binário avulso..."
cp "$BIN_SOURCE" "$ROOT_DIR/dist/macos/open-presenter"
swift -e '
import Cocoa

let iconPath = "'"$ROOT_DIR/assets/branding/open-presenter-app-icon-256.png"'"
let binaryPath = "'"$ROOT_DIR/dist/macos/open-presenter"'"
let appPath = "'"$BUNDLE_DIR"'"

if let image = NSImage(contentsOfFile: iconPath) {
    let ws = NSWorkspace.shared
    _ = ws.setIcon(image, forFile: binaryPath, options: [])
    _ = ws.setIcon(image, forFile: appPath, options: [])
    print("Ícone atribuído com sucesso via NSWorkspace.")
}
'

echo "==> Compactando pacote oficial de release..."
(
    cd "$ROOT_DIR/dist/macos"
    tar -czvf "open-presenter-macos-v${VERSION}.tar.gz" "${APP_NAME}.app" open-presenter
    zip -r "open-presenter-macos-v${VERSION}.zip" "${APP_NAME}.app" open-presenter
)

echo "==> Concluído com sucesso! Arquivos gerados em $ROOT_DIR/dist/macos:"
ls -lh "$ROOT_DIR/dist/macos"
