use std::path::PathBuf;
use std::thread;

pub(crate) fn convert_path_buf_to_string(p: &PathBuf) -> String {
    p.to_str()
        .expect("There was an error while converting a path buf to a string.")
        .to_string()
}

pub(crate) fn split_vec_into_chunks<T>(paths: &Vec<T>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let num_threads = match thread::available_parallelism() {
        Ok(v) => v.get() / 3,
        Err(_) => 0,
    };

    let chunk_size = if num_threads > paths.len() {
        None
    } else {
        Some(paths.len() / num_threads)
    };

    return if chunk_size.is_some() {
        paths
            .chunks(chunk_size.unwrap())
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>()
    } else {
        let mut vec = Vec::with_capacity(1);
        vec.push(paths.clone());
        vec
    };
}
