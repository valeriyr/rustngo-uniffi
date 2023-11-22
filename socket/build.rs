fn main() {
    uniffi::generate_scaffolding("./src/socket.udl").expect("Building the UDL file failed");
}
