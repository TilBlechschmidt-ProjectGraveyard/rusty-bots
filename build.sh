cd client
cargo build

cd ../user1
cargo build

cd ../server
cargo build

cd ..
cp -R user1/target/debug/libuser1.dylib server/user/user1.dylib
