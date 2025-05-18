use flexi_logger::{
    default_format, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming, WriteMode,
};
use log::error;

pub fn init_logger() {
    if let Err(err) = Logger::try_with_str("trace")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("sv-analysis")
                .suffix("log"),
        )
        .rotate(
            Criterion::Size(10_000_000),
            Naming::Numbers,
            Cleanup::KeepLogFiles(3),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stderr(Duplicate::Info)
        .format_for_files(default_format)
        .start() {
        error!("Failed to initialize logging: {}", err);
    }
}