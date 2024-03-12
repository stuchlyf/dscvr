use std::collections::HashMap;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::Error;

use log::info;
use tantivy::directory::MmapDirectory;
use tantivy::Index;
use tantivy::schema::{FAST, STORED, TEXT};
use tonic::transport::Server;

use dscvr_common::config::{AppSettings, init_config};
use dscvr_common::logger::init_logger;

use crate::conversion::clear_text_conversion::ClearTextConversion;
use crate::conversion::Conversion;
use crate::conversion::convert_to_clear_text_strategy::MimeType;
use crate::conversion::determine_file_type::{DefaultDetermineFileTypeFactory};
use crate::conversion::pdf_conversion::PdfConversion;
use crate::file_index::file_hash_strategy::DefaultFileHash;
use crate::file_index::FileIndexService;
use crate::file_index::find_duplicated_files_strategy::FindDuplicatedFiles;
use crate::file_index::index_file_strategy::TantivyIndexStrategy;
use crate::file_index::persist_metadata_strategy::SqlitePersistenceStrategy;
use crate::file_index::search_file_strategy::TantivySearchStrategy;
use crate::file_indexer::file_indexer_server::FileIndexerServer;

pub(crate) mod file_index;
pub(crate) mod conversion;

pub(crate) mod file_indexer {
    tonic::include_proto!("file_indexer");
}

pub(crate) mod proto_utils {
    tonic::include_proto!("proto_utils");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();
    let settings = init_config();
    let addr = format!("{}:{}", settings.indexer.host, settings.indexer.port)
        .parse()
        .unwrap();

    let mut schema_builder = tantivy::schema::Schema::builder();
    schema_builder.add_text_field("contents", TEXT | STORED);
    schema_builder.add_text_field("hash", STORED | FAST);
    schema_builder.add_text_field("path", STORED);
    let file_schema = schema_builder.build();

    let index_path = get_index_path(&settings);
    let index = Arc::new(Index::open_or_create(
        MmapDirectory::open(&index_path)?,
        file_schema.clone(),
    )?);

    let writer = index.writer(1_000_000_000)?; // TODO: lower the memory budget of the writer.
    let reader = index.reader()?;

    let determine_file_type_strategy = Arc::new(DefaultDetermineFileTypeFactory::create());

    let mut conversions_map = HashMap::<MimeType, Arc<dyn Conversion>>::new();
    conversions_map.insert(MimeType::TextPlain, Arc::new(ClearTextConversion::new()));
    conversions_map.insert(MimeType::ApplicationPdf, Arc::new(PdfConversion::new()));
    conversions_map.insert(MimeType::ApplicationMsWord, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationVndOpenxmlformatsOfficedocumentWordprocessingmlDocument, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationVndOasisOpendocumentPresentation, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationVndOasisOpendocumentSpreadsheet, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationVndOasisOpendocumentText, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationVndMsPowerpoint, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationVndOpenxmlformatsOfficedocumentPresentationmlPresentation, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationRtf, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationVndMsExcel, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ApplicationVndOpenxmlformatsOfficedocumentSpreadsheetmlSheet, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImageAvif, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImageBmp, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImageGif, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImageIcon, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImageJpeg, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImagePng, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImageSvgXml, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImageTiff, Arc::new(NoOpConversion {}));
    conversions_map.insert(MimeType::ImageWebp, Arc::new(NoOpConversion {}));


    let persist_metadata_strategy =
        Arc::new(SqlitePersistenceStrategy::build_with_settings(&settings)?);
    let file_hash_strategy = Arc::new(DefaultFileHash::new());

    let index_strategy = Arc::new(Mutex::new(TantivyIndexStrategy::new(
        file_schema.clone(),
        writer,
        persist_metadata_strategy,
        determine_file_type_strategy,
        conversions_map,
        file_hash_strategy
    )));
    let search_strategy = Arc::new(TantivySearchStrategy::new(
        index,
        reader,
        file_schema.clone(),
    ));
    let find_duplicated_files_strategy =
        Arc::new(FindDuplicatedFiles::build_with_settings(&settings)?);

    let file_indexer = FileIndexService::new(
        index_strategy,
        search_strategy,
        find_duplicated_files_strategy,
    );

    info!("listening on {}", addr);

    Server::builder()
        .max_frame_size(Some(16777215))
        .add_service(FileIndexerServer::new(file_indexer))
        .serve(addr)
        .await?;

    Ok(())
}

fn get_index_path(settings: &AppSettings) -> PathBuf {
    let index_path =
        Path::new(&settings.common.base_dir).join(&settings.indexer.index_directory_name);

    if !index_path.exists() {
        create_dir(&index_path).expect(&format!(
            "There was an error while trying to create the directory {:?}:",
            index_path
        ));
    }

    return index_path;
}


pub(crate) struct NoOpConversion;

impl Conversion for NoOpConversion {
    fn convert(&self, _: Vec<u8>) -> Result<String, Error> {
        return Ok("".to_string());
    }
}
