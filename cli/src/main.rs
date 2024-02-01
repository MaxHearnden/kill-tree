use clap::{
    builder::{styling::AnsiColor, Styles},
    command, value_parser, Arg, ArgAction,
};
use kill_tree::{kill_tree_with_config, Config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .name("kill-tree")
        .bin_name("kill-tree")
        .arg_required_else_help(true)
        .styles(
            Styles::styled()
                .header(AnsiColor::BrightGreen.on_default().bold())
                .usage(AnsiColor::BrightGreen.on_default().bold())
                .literal(AnsiColor::BrightCyan.on_default().bold())
                .placeholder(AnsiColor::Cyan.on_default()),
        )
        .arg(
            Arg::new("PROCESS_ID")
                .help("Process ID to kill with all children.")
                .value_parser(value_parser!(u32)),
        )
        .arg(
            Arg::new("SIGNAL")
                .help("Signal to send to the processes.")
                .default_value("SIGTERM"),
        )
        .arg(
            Arg::new("QUIET")
                .short('q')
                .long("quiet")
                .help("No logs are output.")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let process_id = *matches.get_one::<u32>("PROCESS_ID").unwrap();
    let signal = matches.get_one::<String>("SIGNAL").unwrap();
    let quiet = *matches.get_one::<bool>("QUIET").unwrap();
    let do_print = !quiet;
    if do_print {
        println!(
            "Killing process {} with all children using signal {}",
            process_id, signal
        );
    }
    let mut config = Config::default();
    config.signal = signal.to_string();
    let process_ids = kill_tree_with_config(process_id, config)?;
    if do_print {
        println!("Killed processes: {:?}", process_ids);
    }
    Ok(())
}
