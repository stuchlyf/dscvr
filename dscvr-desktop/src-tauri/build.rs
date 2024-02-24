use std::env;
use std::path::PathBuf;

fn main() {
    tauri_build::build();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("proto_utils_descriptor.bin"))
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]\n#[serde(rename_all = \"camelCase\")]")
        .build_transport(true)
        .build_server(false)
        .build_client(false)
        .compile(&["../../proto/proto_utils.proto"], &["../../proto"])
        .unwrap();

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("file_indexer_descriptor.bin"))
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]\n#[serde(rename_all = \"camelCase\")]")
        .build_transport(true)
        .build_server(false)
        .build_client(true)
        .compile(&["../../proto/file_indexer.proto"], &["../../proto"])
        .unwrap();
}
