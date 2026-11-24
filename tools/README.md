# Build and release tools

Shared by every acorn app. Each reads the app's settings from `acorn-app.env` in the app's
repository root (see `example.acorn-app.env`; the release scripts also read a private
`acorn-release.env`, see `example.acorn-release.env`), and runs from there. Apps call them through
`tools/acorn.sh`, which finds this checkout with `cargo metadata` (so it works with a path
dependency during development and a git dependency on CI):

    tools/acorn.sh macos/bundle.sh        dist/<Name>.app and dist/<Name>-<version>.zip
    tools/acorn.sh linux/appimage.sh      dist/<Name>-<version>-<arch>.AppImage (+ .sig)
    tools/acorn.sh linux/install.sh       build and install for the current user on Linux
    tools/acorn.sh release/snapshot.sh    the public source tree for a release
    tools/acorn.sh release/publish.sh     open a "Release vX" PR on the public repo

`github/linux-appimages.yml` is a workflow to copy into the app's public repo: it builds the
AppImages when a release is published and attaches them, signed for `acorn-update`.
