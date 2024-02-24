use std::io::Write;
use std::thread;

pub fn init_logger() {
    env_logger::builder()
        .default_format()
        .format(|buf, record| {
            let ts = buf.timestamp_seconds();
            let thread_id_as_string = format!("{:?}", thread::current().id());
            let thread_name = thread::current()
                .name()
                .unwrap_or_else(|| &thread_id_as_string)
                .to_string();
            let module_path = record.module_path().unwrap_or_else(|| "");

            writeln!(
                buf,
                "[{} {} {} {}]: {}",
                ts,
                buf.default_level_style(record.level())
                    .value(record.level()),
                thread_name,
                module_path,
                record.args()
            )
        })
        .init();
}
