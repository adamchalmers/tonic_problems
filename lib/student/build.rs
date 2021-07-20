fn main() {
    use person as libperson;
    use tonic_build::configure;

    // Compile the `student` proto.
    // to the `person` package we just compiled above.
    configure()
        // Declare the `website.com/protos/person.proto` Protobuf package and all
        // nested packages and types as externally provided by the `person` crate.
        .extern_path(".website.com/protos/person.proto", "libperson")
        .compile(&["../../protos/student.proto"], &["../../"])
        .unwrap();
}
