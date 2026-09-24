fn main() {
    let file_descriptor_set = protox::compile(["marketdata.proto"], ["proto"]).unwrap();
    prost_build::compile_fds(file_descriptor_set).unwrap();
}
