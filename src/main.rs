use clickflow::engine;
use log::{Log, Record};

const BIN_NAME: &str = "clickflow";

fn set_up_logger() {
    let default_logger = env_logger::builder()
        .format_target(false)
        .format_module_path(true)
        .build();
    env_logger::builder()
        .format(move |_buf, record| {
            // modify record module_path to print file and line
            let new_module_path = format!(
                "{} {}:{}",
                record.module_path().unwrap_or("-"),
                record.file().unwrap_or("-"),
                record.line().unwrap_or_default()
            );
            let new_record = Record::builder()
                .args(*record.args())
                .file(record.file())
                .file_static(record.file_static())
                .level(record.level())
                .line(record.line())
                .metadata(record.metadata().clone())
                .module_path(Some(&new_module_path))
                .target(record.target())
                .build();
            default_logger.log(&new_record);
            Ok(())
        })
        .init();
}

fn main() {
    set_up_logger();

    let cmd = clap::Command::new(BIN_NAME)
        .bin_name(BIN_NAME)
        .subcommand_required(true)
        .subcommand(
            clap::command!("start")
                .about("Start executing tasks from task config files")
                .arg(clap::arg!(<files> ... "Path to the clickflow task configuration files")),
        );
    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some(("start", matches)) => {
            let paths = matches
                .get_many::<String>("files")
                .unwrap_or_default()
                .map(|v| v.as_str())
                .collect::<Vec<_>>();
            let e = engine::Engine {};
            e.start_tasks(paths);
        }
        _ => {}
    }
}
