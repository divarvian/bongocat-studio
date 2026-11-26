PREFIX ?= $(HOME)/.local
BINDIR := $(PREFIX)/bin
APPDIR := $(PREFIX)/share/applications
ICONDIR := $(PREFIX)/share/icons/hicolor/scalable/apps
LICDIR := $(PREFIX)/share/licenses/bongocat-studio

CARGO ?= $(if $(wildcard $(HOME)/.cargo/bin/cargo),$(HOME)/.cargo/bin/cargo,cargo)

.PHONY: all build release check fmt fmt-check lint test rpm deb tarball packages install uninstall clean

all: release

build:
	$(CARGO) build --workspace

release:
	$(CARGO) build --release -p bongocat-app

check: fmt-check lint test

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

test:
	$(CARGO) test --workspace

rpm: release
	bash packaging/build-rpm.sh

deb: release
	bash packaging/build-deb.sh

tarball: release
	bash packaging/build-tarball.sh

packages: rpm deb tarball
	@cd dist && sha256sum *.rpm *.deb *.tar.gz > SHA256SUMS
	@echo "Generated dist/SHA256SUMS"

install: release
	install -Dm755 target/release/bongocat-studio $(BINDIR)/bongocat-studio
	install -Dm644 assets/icons/bongocat-studio.svg $(ICONDIR)/bongocat-studio.svg
	install -Dm644 LICENSE $(LICDIR)/LICENSE
	install -Dm644 THIRD_PARTY_LICENSES.md $(LICDIR)/THIRD_PARTY_LICENSES.md
	@mkdir -p $(APPDIR)
	sed 's|@BIN_PATH@|$(BINDIR)/bongocat-studio|g' assets/desktop/bongocat-studio.desktop.in \
		> $(APPDIR)/bongocat-studio.desktop
	@chmod 644 $(APPDIR)/bongocat-studio.desktop
	@command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database $(APPDIR) || true
	@if command -v gtk4-update-icon-cache >/dev/null 2>&1; then \
		gtk4-update-icon-cache -f -t $(PREFIX)/share/icons/hicolor 2>/dev/null || true; \
	elif command -v gtk-update-icon-cache >/dev/null 2>&1; then \
		gtk-update-icon-cache -f -t $(PREFIX)/share/icons/hicolor 2>/dev/null || true; \
	fi
	@echo ""
	@echo "Installed binary:   $(BINDIR)/bongocat-studio"
	@echo "Installed desktop:  $(APPDIR)/bongocat-studio.desktop"
	@echo "Installed icon:     $(ICONDIR)/bongocat-studio.svg"
	@echo "Installed licenses: $(LICDIR)/"

uninstall:
	rm -f $(BINDIR)/bongocat-studio
	rm -f $(APPDIR)/bongocat-studio.desktop
	rm -f $(ICONDIR)/bongocat-studio.svg
	rm -f $(PREFIX)/share/icons/hicolor/icon-theme.cache
	rm -rf $(LICDIR)
	rm -f $(HOME)/.config/autostart/bongocat-studio.desktop
	@command -v update-desktop-database >/dev/null 2>&1 && update-desktop-database $(APPDIR) || true

clean:
	$(CARGO) clean
	rm -rf dist
