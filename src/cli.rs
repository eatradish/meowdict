use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("meowdict")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Mag Mell")
        .about("Search chinese keyword from moedict.tw")
        .arg(
            Arg::new("inputs2t")
                .short('i')
                .long("input-s2t")
                .help("Convert input to traditional Chinese and search")
                .requires("INPUT"),
        )
        .arg(
            Arg::new("resultt2s")
                .short('r')
                .long("result-t2s")
                .help("Convert result to Simplified Chinese to display"),
        )
        .arg(
            Arg::new("inputs2tmode")
                .long("input-s2t-mode")
                .help("Open console with input-s2t mode"),
        )
        .arg(
            Arg::new("resultt2smode")
                .long("result-t2s-mode")
                .help("Open console with result-t2s mode"),
        )
        .arg(
            Arg::new("no-color-output")
                .long("no-color-output")
                .help("Print result with no color")
                .requires("INPUT"),
        )
        .arg(
            Arg::new("INPUT")
                .help("Input the keyword to use")
                .index(1)
                .num_args(1..),
        )
        .subcommand(
            Command::new("show")
                .about("Get dict result")
                .arg(
                    Arg::new("INPUT")
                        .help("Input the keyword to use")
                        .index(1)
                        .num_args(1..)
                        .action(clap::ArgAction::Set)
                        .required(true),
                )
                .arg(
                    Arg::new("inputs2t")
                        .short('i')
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("resultt2s")
                        .short('r')
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            Command::new("translate")
                .alias("trans")
                .about("Get word translation")
                .arg(
                    Arg::new("INPUT")
                        .help("Input word here")
                        .action(clap::ArgAction::Set)
                        .required(true),
                )
                .arg(
                    Arg::new("inputs2t")
                        .short('i')
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("resultt2s")
                        .short('r')
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            Command::new("jyutping")
                .alias("jyut")
                .about("Get word jyutping")
                .arg(
                    Arg::new("INPUT")
                        .help("Input word here")
                        .action(clap::ArgAction::Set)
                        .required(true),
                )
                .arg(
                    Arg::new("inputs2t")
                        .short('i')
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("resultt2s")
                        .short('r')
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            Command::new("terminal")
                .alias("term")
                .about("Open meowdict terminal")
                .arg(
                    Arg::new("inputs2tmode")
                        .long("input-s2t-mode")
                        .help("Open console with input-s2t mode"),
                )
                .arg(
                    Arg::new("resultt2smode")
                        .long("result-t2s-mode")
                        .help("Open console with result-t2s mode"),
                )
                .arg(
                    Arg::new("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            Command::new("random")
                .alias("rand")
                .about("search random word")
                .arg(Arg::new("INPUT").help("Input word here").num_args(0..))
                .arg(
                    Arg::new("inputs2t")
                        .short('i')
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("resultt2s")
                        .short('r')
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            Command::new("json")
                .about("Print result to JSON output")
                .arg(
                    Arg::new("INPUT")
                        .help("Input word here")
                        .action(clap::ArgAction::Set)
                        .required(true),
                )
                .arg(
                    Arg::new("inputs2t")
                        .short('i')
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::new("resultt2s")
                        .short('r')
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                ),
        )
}
