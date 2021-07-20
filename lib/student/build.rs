fn main() -> std::io::Result<()> {
    prost_build::Config::new()
        // Declare the `person.proto` Protobuf package and all
        // nested packages and types as externally provided by the `person` crate.
        .extern_path(".person", "::person")
        .compile_protos(&["../../protos/student.proto"], &["../../protos"])?;
    Ok(())
}
