#!/bin/bash

cd client
cargo build --release

cd ../user1
cargo build --release

cd ../server
cargo build --release

cd ..
if [ ! -d "server/user/" ]; then
    mkdir -p server/user/
fi

LIB_SRC="user1/target/release"
if [ -f "$LIB_SRC/libuser1.dylib" ]; then
    cp -R $LIB_SRC/libuser1.dylib server/user/
elif [ -f "$LIB_SRC/libuser1.so" ]; then
    cp -R $LIB_SRC/libuser1.so server/user/
fi
