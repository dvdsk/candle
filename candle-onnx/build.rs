use std::io::Result;

fn main() -> Result<()> {
    // Disabled as it does not work on Mac & Windows. Instead we ship
    // pre-'compiled' protocol buffers.

    // std::env::set_var("PROTOC", protobuf_src::protoc());
    // prost_build::compile_protos(&["src/onnx.proto3"], &["src/"])?;
    Ok(())
}
