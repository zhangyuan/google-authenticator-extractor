fn main() {
    protobuf_codegen::Codegen::new()
        .out_dir("src/protos")
        .inputs(["protos/google_auth.proto"])
        .include("protos")
        .run()
        .expect("protobuf codegen");
}
