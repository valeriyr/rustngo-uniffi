# Build the socket Rust library
cargo build --manifest-path ./socket/Cargo.toml

# Generate bindings
cd ./socket
rm -fdr ../GoApp/Socket
uniffi-bindgen-go --library ./target/debug/libsocket.dylib --out-dir ../GoApp
cp ./target/debug/libsocket.dylib ../GoApp/Socket
cd ..

# Export paths
export CGO_LDFLAGS="-L$(pwd)/GoApp/Socket/"
export LD_LIBRARY_PATH="$(pwd)/GoApp/Socket/"

# Build the Go app
cd ./GoApp
rm -fdr ./bin
go build -o ./bin/app ./main.go

# Run the program
./bin/app
