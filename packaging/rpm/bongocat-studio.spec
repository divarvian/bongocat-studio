Name:           bongocat-studio
Version:        %{_pkg_version}
Release:        1%{?dist}
Summary:        Pure Rust desktop control center and built-in OBS Browser Source server for Bongo Cat

License:        MIT AND MPL-2.0
URL:            https://github.com/divarvian/bongocat-studio

# Built from precompiled release binary staged by packaging/build-rpm.sh
%global debug_package %{nil}
%global __brp_strip %{nil}
%global _build_id_links none

%description
BongoCat Studio is a standalone Pure Rust desktop application and built-in
OBS Studio Browser Source server for Linux (Wayland and X11). It features
Slanted, Flat Desk, and Classic visual modes, 10-finger ergonomic typing,
clickpad gestures, real-time OBS config sync, and privacy controls.

%install
rm -rf %{buildroot}
install -Dm755 "%{_stage_dir}/bongocat-studio" "%{buildroot}%{_bindir}/bongocat-studio"
install -Dm644 "%{_stage_dir}/bongocat-studio.desktop" "%{buildroot}%{_datadir}/applications/bongocat-studio.desktop"
install -Dm644 "%{_stage_dir}/bongocat-studio.svg" "%{buildroot}%{_datadir}/icons/hicolor/scalable/apps/bongocat-studio.svg"
install -Dm644 "%{_stage_dir}/LICENSE" "%{buildroot}%{_datadir}/licenses/bongocat-studio/LICENSE"
install -Dm644 "%{_stage_dir}/THIRD_PARTY_LICENSES.md" "%{buildroot}%{_datadir}/licenses/bongocat-studio/THIRD_PARTY_LICENSES.md"

%post
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q %{_datadir}/applications || :
fi
if command -v gtk4-update-icon-cache >/dev/null 2>&1; then
    gtk4-update-icon-cache -q -f -t %{_datadir}/icons/hicolor || :
elif command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -f -t %{_datadir}/icons/hicolor || :
fi

%postun
if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database -q %{_datadir}/applications || :
fi
if command -v gtk4-update-icon-cache >/dev/null 2>&1; then
    gtk4-update-icon-cache -q -f -t %{_datadir}/icons/hicolor || :
elif command -v gtk-update-icon-cache >/dev/null 2>&1; then
    gtk-update-icon-cache -q -f -t %{_datadir}/icons/hicolor || :
fi

%files
%license %{_datadir}/licenses/bongocat-studio/LICENSE
%license %{_datadir}/licenses/bongocat-studio/THIRD_PARTY_LICENSES.md
%{_bindir}/bongocat-studio
%{_datadir}/applications/bongocat-studio.desktop
%{_datadir}/icons/hicolor/scalable/apps/bongocat-studio.svg

%changelog
* Thu Nov 26 2026 Arvian <divarvian@users.noreply.github.com> - 0.1.0-1
- Initial standalone Pure Rust release of BongoCat Studio
