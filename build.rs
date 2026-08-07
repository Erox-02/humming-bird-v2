fn main() {
    println!("cargo:rerun-if-changed=src/python_bridge/");
    println!("cargo:rerun-if-changed=../hbp100/");
}
