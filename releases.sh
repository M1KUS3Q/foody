export VERSION=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[0].version')

cargo b --release && cargo b --release --target aarch64-linux-android
mkdir -p releases
tar -czf releases/foody-v$VERSION-aarch64-linux-android.tar.gz -C target/aarch64-linux-android/release foody
tar -czf releases/foody-v$VERSION-aarch64-apple-darwin.tar.gz -C target/release foody

read -p "Really push new version tag $VERSION (y/n)? " choice;
if [[ "$choice" != "y" ]]; then
    echo "Aborting."
    exit 1
fi

git tag v$VERSION
git push origin v$VERSION

printf "Add release notes:\n"
read notes

gh release create v$VERSION --title "v$VERSION" --notes "$notes" \
    releases/foody-v$VERSION-aarch64-linux-android.tar.gz \
    releases/foody-v$VERSION-aarch64-apple-darwin.tar.gz