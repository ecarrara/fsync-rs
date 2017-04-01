set -ex

curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain $TRAVIS_RUST_VERSION

if [ $TRAVIS_OS_NAME = linux ]; then
    TARGET=x86_64-unknown-linux-gnu
    sort=sort
else
    TARGET=x86_64-apple-darwin
    sort=gsort  # for `sort --sort-version`, from brew's coreutils.
fi

# This fetches latest stable release
TAG=$(git ls-remote --tags --refs --exit-code https://github.com/japaric/cross \
       | cut -d/ -f3 \
       | grep -E '^v[0-9.]+$' \
       | $sort --version-sort \
       | tail -n1)
echo cross version: $tag
curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- \
       --force \
       --git japaric/cross \
       --tag $TAG \
       --target $TARGET

