# Name of the application's binary.
name := 'cosmic-ext-redeye'
# The unique ID of the application.
appid := 'io.github.big-ol-pants.CosmicExtRedeye'

# Path to root file system, which defaults to `/`.
rootdir := ''
# The prefix for the `/usr` directory.
prefix := '/usr'
# The location of the cargo target directory.
cargo-target-dir := env('CARGO_TARGET_DIR', 'target')

# Application's appstream metadata
appdata := appid + '.metainfo.xml'
# Application's desktop entry
desktop := appid + '.desktop'
# Application icon
icon := 'Redeye.svg'

# Install destinations
base-dir := absolute_path(clean(rootdir / prefix))
appdata-dst := base-dir / 'share' / 'appdata' / appdata
bin-dst := base-dir / 'bin' / name
desktop-dst := base-dir / 'share' / 'applications' / desktop
icon-dst := base-dir / 'share' / 'icons' / 'hicolor' / 'scalable' / 'apps' / icon
user-base-dir := env('HOME') / '.local'
user-appdata-dst := user-base-dir / 'share' / 'metainfo' / appdata
user-bin-dst := user-base-dir / 'bin' / name
user-desktop-dst := user-base-dir / 'share' / 'applications' / desktop
user-icon-dst := user-base-dir / 'share' / 'icons' / 'hicolor' / 'scalable' / 'apps' / icon

# Default recipe which runs `just build-release`
default: build-release

# Runs `cargo clean`
clean:
    cargo clean

# Removes vendored dependencies
clean-vendor:
    rm -rf .cargo vendor vendor.tar

# `cargo clean` and removes vendored dependencies
clean-dist: clean clean-vendor

# Compiles with debug profile
build-debug *args:
    cargo build --locked {{args}}

# Compiles with release profile
build-release *args: (build-debug '--release' args)

# Compiles release profile with vendored dependencies
build-vendored *args: vendor-extract (build-release '--frozen --offline' args)

# Runs a clippy check
check *args:
    cargo clippy --all-features --locked {{args}} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

# Run the application for testing purposes
run *args:
    env RUST_BACKTRACE=full cargo run --release --locked {{args}}

# Installs files
install:
    install -Dm0755 {{ cargo-target-dir / 'release' / name }} {{bin-dst}}
    install -Dm0644 resources/app.desktop {{desktop-dst}}
    install -Dm0644 resources/app.metainfo.xml {{appdata-dst}}
    install -Dm0644 {{ 'resources/icons/hicolor/scalable/apps' / icon }} {{icon-dst}}

# Installs files for the current user so COSMIC Panel can discover the applet during development
install-user: build-release
    install -Dm0755 {{ cargo-target-dir / 'release' / name }} {{user-bin-dst}}
    install -Dm0644 resources/app.desktop {{user-desktop-dst}}
    install -Dm0644 resources/app.metainfo.xml {{user-appdata-dst}}
    install -Dm0644 {{ 'resources/icons/hicolor/scalable/apps' / icon }} {{user-icon-dst}}

# Uninstalls installed files
uninstall:
    rm -f {{bin-dst}} {{desktop-dst}} {{appdata-dst}} {{icon-dst}}

# Uninstalls current-user development files
uninstall-user:
    rm -f {{user-bin-dst}} {{user-desktop-dst}} {{user-appdata-dst}} {{user-icon-dst}}

# Vendor dependencies locally
vendor:
    mkdir -p .cargo
    cargo vendor | head -n -1 > .cargo/config.toml
    echo 'directory = "vendor"' >> .cargo/config.toml
    tar pcf vendor.tar vendor
    rm -rf vendor

# Extracts vendored dependencies
vendor-extract:
    rm -rf vendor
    tar pxf vendor.tar

# Bump cargo version, create git commit, and create tag
tag version:
    find -type f -name Cargo.toml -exec sed -i '0,/^version/s/^version.*/version = "{{version}}"/' '{}' \; -exec git add '{}' \;
    cargo check
    cargo clean
    git add Cargo.lock
    git commit -m 'release: {{version}}'
    git commit --amend
    git tag -a {{version}} -m ''
