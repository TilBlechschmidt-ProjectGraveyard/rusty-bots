cd client
cargo build --release

cd ../user1
cargo build --release

#cd ../server
#cargo build --release

cd ..
cp -R user1/target/release/libuser1.dylib server/user/user1.dylib
