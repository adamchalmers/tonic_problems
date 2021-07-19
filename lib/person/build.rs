fn main() {
    use tonic_build::compile_protos;
    compile_protos("../../protos/person.proto").unwrap();
}
