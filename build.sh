#!/bin/bash -e

if [[ $1 == -* || !$1 ]]; then
    ARCHITECTURE=$(uname -m)
else
    ARCHITECTURE=$1
fi

export DOCKER_SCAN_SUGGEST=false
export CROSS_CONFIG=$(mktemp)

echo $CROSS_CONFIG
echo '[target.armv7-unknown-linux-gnueabihf]' >$CROSS_CONFIG

case "$ARCHITECTURE" in
arm* | aarch64)
    echo 'image = "cross-compiler-mcar-arm"' >>$CROSS_CONFIG
    docker build -t cross-compiler-mcar-arm -f Dockerfile.arm .
    docker run --rm -v $(pwd):/src cross-compiler-mcar-arm /bin/bash -c "cd /src && cargo build ${@} --target armv7-unknown-linux-gnueabihf"
    ;;
*)
    echo 'image = "cross-compiler-mcar-x86_64"' >>$CROSS_CONFIG
    docker pull ghcr.io/cross-rs/armv7-unknown-linux-gnueabihf:latest --platform linux/amd64 >/dev/null
    docker build -t cross-compiler-mcar-x86_64 -f Dockerfile.x86_64 .
    cross build ${@} --target armv7-unknown-linux-gnueabihf
    ;;
esac

rm $CROSS_CONFIG
