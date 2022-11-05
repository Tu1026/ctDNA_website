fn main (){
    use std::env;
    env::set_var("OUT_DIR", ".");
    protobuf_codegen::Codegen::new()
    // Use `protoc` parser, optional.
    .protoc()
    .includes(&["protobuf"])
    // Use `protoc-bin-vendored` bundled protoc command, optional.
    // .protoc_path(&protoc_bin_vendored::protoc_bin_path().unwrap())
    // Inputs must reside in some of include paths.
    .input("protobuf/samples.proto")
    // .input("src/protos/banana.proto")
    // Specify output directory relative to Cargo output directory.
    .cargo_out_dir("protobuf/rust_code")
    .run_from_script();
}