use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("proto_utils_descriptor.bin"))
        .build_transport(true)
        .build_server(false)
        .build_client(false)
        .compile(&["../proto/proto_utils.proto"], &["../proto"])
        .unwrap();

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("file_indexer_descriptor.bin"))
        .build_transport(true)
        .build_server(true)
        .build_client(false)
        .compile(&["../proto/file_indexer.proto"], &["../proto"])
        .unwrap();
}
