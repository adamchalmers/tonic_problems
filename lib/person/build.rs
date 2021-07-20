fn main() -> std::io::Result<()> {
    prost_build::compile_protos(&["../../protos/person.proto"], &["../../protos"])?;
    Ok(())
}
