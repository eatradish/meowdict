use clap::{crate_version, App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    App::new("meowdict")
        .version(crate_version!())
        .author("Mag Mell")
        .about("Search chinese keyword from moedict.tw")
        .arg(
            Arg::with_name("inputs2t")
                .short("i")
                .long("input-s2t")
                .help("Convert input to traditional Chinese and search")
                .requires("INPUT"),
        )
        .arg(
            Arg::with_name("resultt2s")
                .short("r")
                .long("result-t2s")
                .help("Convert result to Simplified Chinese to display"),
        )
        .arg(
            Arg::with_name("inputs2tmode")
                .long("input-s2t-mode")
                .help("Open console with input-s2t mode"),
        )
        .arg(
            Arg::with_name("resultt2smode")
                .long("result-t2s-mode")
                .help("Open console with result-t2s mode"),
        )
        .arg(
            Arg::with_name("no-color-output")
                .long("no-color-output")
                .help("Print result with no color")
                .requires("INPUT"),
        )
        .arg(
            Arg::with_name("INPUT")
                .help("Input the keyword to use")
                .index(1)
                .min_values(1),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Get dict result")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Input the keyword to use")
                        .index(1)
                        .min_values(1)
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("inputs2t")
                        .short("i")
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("resultt2s")
                        .short("r")
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            SubCommand::with_name("translate")
                .alias("trans")
                .about("Get word translation")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Input word here")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("inputs2t")
                        .short("i")
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("resultt2s")
                        .short("r")
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            SubCommand::with_name("jyutping")
                .alias("jyut")
                .about("Get word jyutping")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Input word here")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("inputs2t")
                        .short("i")
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("resultt2s")
                        .short("r")
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            SubCommand::with_name("terminal")
                .alias("term")
                .about("Open meowdict terminal")
                .arg(
                    Arg::with_name("inputs2tmode")
                        .long("input-s2t-mode")
                        .help("Open console with input-s2t mode"),
                )
                .arg(
                    Arg::with_name("resultt2smode")
                        .long("result-t2s-mode")
                        .help("Open console with result-t2s mode"),
                )
                .arg(
                    Arg::with_name("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            SubCommand::with_name("random")
                .alias("rand")
                .about("search random word")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Input word here")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("inputs2t")
                        .short("i")
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("resultt2s")
                        .short("r")
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("no-color-output")
                        .long("no-color-output")
                        .help("Print result with no color")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            SubCommand::with_name("json")
                .about("Print result to JSON output")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Input word here")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("inputs2t")
                        .short("i")
                        .long("input-s2t")
                        .help("Convert input to traditional Chinese and search")
                        .requires("INPUT"),
                )
                .arg(
                    Arg::with_name("resultt2s")
                        .short("r")
                        .long("result-t2s")
                        .help("Convert result to Simplified Chinese to display")
                        .requires("INPUT"),
                ),
        )
        .subcommand(
            SubCommand::with_name("reverse")
                .about("Get word reverse result (data from wantwords.thunlp.org")
                .alias("rev")
                .arg(
                    Arg::with_name("INPUT")
                        .help("Input word here")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("all")
                        .help("Get all reverse result")
                        .short("a")
                        .requires("INPUT"),
                ),
        )
}
