#!/usr/bin/env bash
# App-specific installer and package assertions retained alongside shared packaging lint.
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

for want in '<UIRef Id="PicshotUI" />' 'Value="InstallDirDlg"' '<Property Id="WIXUI_INSTALLDIR" Value="INSTALLFOLDER" />' \
    'Key="Software\NephclickPicshot" Name="InstallFolder"' 'Value="LaunchPicshot"'; do
  grep -qF "$want" packaging/windows/picshot.wxs || { echo "::error::picshot.wxs lost: $want" >&2; exit 1; }
done
grep -qF '<DialogRef Id="ExitDialog" />' packaging/windows/picshot.wxs \
  || { echo "::error::ExitDialog is not referenced; installs and removes end without a completion dialog" >&2; exit 1; }

python3 - <<'PYEOF'
import yaml
manifests = []
for name in ("com.nephclick.picshot.yml", "com.nephclick.picshot.bundle.yml"):
    with open("packaging/linux/flatpak/" + name) as source:
        manifest = yaml.safe_load(source)
    assert manifest["id"] == "com.nephclick.picshot" and manifest["command"] == "picshot", name
    # Without this talk-name the sandbox hides the accessibility bus and Orca cannot see the app.
    assert "--talk-name=org.a11y.Bus" in manifest["finish-args"], f"{name} lost --talk-name=org.a11y.Bus"
    manifests.append(manifest)
for key in ("runtime", "runtime-version", "sdk", "finish-args"):
    assert manifests[0][key] == manifests[1][key], f"Flatpak manifests disagree on {key}"
print("Nephclick Picshot package identity ok")
PYEOF

# The macOS bundle must declare only file types Nephclick Picshot can open. It used to carry PhotoCraft's
# `.pcraft` document type and imported PSB/QOI declarations, so Launch Services typed PhotoCraft
# documents as Nephclick Picshot's and `.pcraft` had three claimant apps (issue #1951). plistlib parses the
# template as a property list too, which is the syntax check package.sh's `plutil -lint` repeats on
# the substituted file.
python3 - <<'PYEOF'
import plistlib
with open("packaging/macos/Info.plist.in", "rb") as source:
    plist = plistlib.load(source)
for key, value in {
    "CFBundleIdentifier": "com.nephclick.picshot",
    "CFBundleExecutable": "Nephclick Picshot",
    "CFBundleIconFile": "Nephclick Picshot",
    "LSMinimumSystemVersion": "11.0",
}.items():
    assert plist.get(key) == value, f"Info.plist {key}: {plist.get(key)!r}, expected {value!r}"
for key in ("CFBundleShortVersionString", "CFBundleVersion", "Nephclick PicshotVersion", "Nephclick PicshotBuildCommit"):
    assert plist.get(key), f"Info.plist lost {key}"
declared = set()
for section in ("UTExportedTypeDeclarations", "UTImportedTypeDeclarations"):
    for entry in plist.get(section, []):
        tags = entry.get("UTTypeTagSpecification", {})
        declared.update(tags.get("public.filename-extension", []))
        declared.update(tags.get("public.mime-type", []))
        declared.add(entry.get("UTTypeIdentifier", ""))
unsupported = {name.lower() for name in declared} & {
    "pcraft",
    "psb",
    "qoi",
    "application/x-picshot",
    "com.nephclick.picshot.document",
    "com.nephclick.picshot.qoi",
    "com.adobe.photoshop-large-image",
}
assert not unsupported, f"Info.plist declares types Nephclick Picshot cannot open: {sorted(unsupported)}"
print("Nephclick Picshot macOS bundle types ok")
PYEOF
