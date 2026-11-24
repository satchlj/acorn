#!/bin/zsh
# Builds dist/<Name>.app from a `dist`-profile build, and dist/<Name>-<version>.zip.
#   signed with $MAC_SIGN_CERT when the keychain has it (a stable identity, so Keychain
#   access and acorn-update's signature check survive updates), else ad hoc. Gatekeeper
#   warns on first open either way.
#   <PREFIX>_SIGN_ID="Developer ID Application: …"   signed with hardened runtime, ready to notarize
#   <PREFIX>_NOTARY_PROFILE=<keychain profile>        also notarizes and staples (notarytool)
set -e
source ./acorn-app.env
SIGN_ID_VAR=${ENV_PREFIX}_SIGN_ID; SIGN_ID=${(P)SIGN_ID_VAR}
NOTARY_VAR=${ENV_PREFIX}_NOTARY_PROFILE; NOTARY=${(P)NOTARY_VAR}
VERSION=$(awk -F\" '/^version/ {print $2; exit}' Cargo.toml)
BUILD=$(git rev-list --count HEAD)
cargo build --profile dist -p $APP_PACKAGE
APP=dist/$APP_NAME.app
rm -rf $APP
mkdir -p $APP/Contents/MacOS $APP/Contents/Resources
cp target/dist/$APP_CARGO_BIN $APP/Contents/MacOS/$APP_BIN
cp $MAC_ICNS $APP/Contents/Resources/
sed -e "s/@VERSION@/$VERSION/" -e "s/@BUILD@/$BUILD/" $MAC_INFO_PLIST > $APP/Contents/Info.plist
if [[ -n "$SIGN_ID" ]]; then
  codesign --force --options runtime --timestamp --sign "$SIGN_ID" $APP
elif [[ -n "$MAC_SIGN_CERT" ]] && security find-certificate -c "$MAC_SIGN_CERT" >/dev/null 2>&1; then
  codesign --force --options runtime --sign "$MAC_SIGN_CERT" $APP
else
  codesign --force --sign - $APP
fi
codesign --verify --deep --strict $APP
if [[ -n "$NOTARY" && -n "$SIGN_ID" ]]; then
  ditto -c -k --keepParent $APP dist/$APP_NAME.zip
  xcrun notarytool submit dist/$APP_NAME.zip --keychain-profile "$NOTARY" --wait
  xcrun stapler staple $APP
fi
ditto -c -k --keepParent $APP dist/$APP_NAME-$VERSION.zip
echo "$APP  (version $VERSION, build $BUILD)"
