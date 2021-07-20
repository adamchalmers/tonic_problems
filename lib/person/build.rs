fn main() {
    use tonic_build::{compile_protos, configure};

    // Compile the `person` proto
    compile_protos("../../protos/person.proto").unwrap();

    // Compile the `student` proto.
    // We need to map the proto import `website.com/protos/person.proto`
    // to the `person` package we just compiled above.
    // configure()
    //     .extern_path("website.com/protos/person.proto", "::person")
    //     .compile(&["protos/student.proto"], &["."])
    //     .unwrap();
}
