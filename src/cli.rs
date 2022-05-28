use clap::{App, Arg};

pub fn build_cli() -> App<'static> {
    App::new("meowdict")
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
                .min_values(1),
        )
        .subcommand(
            App::new("show")
                .about("Get dict result")
                .arg(
                    Arg::new("INPUT")
                        .help("Input the keyword to use")
                        .index(1)
                        .min_values(1)
                        .takes_value(true)
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
            App::new("translate")
                .alias("trans")
                .about("Get word translation")
                .arg(
                    Arg::new("INPUT")
                        .help("Input word here")
                        .takes_value(true)
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
            App::new("jyutping")
                .alias("jyut")
                .about("Get word jyutping")
                .arg(
                    Arg::new("INPUT")
                        .help("Input word here")
                        .takes_value(true)
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
            App::new("terminal")
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
            App::new("random")
                .alias("rand")
                .about("search random word")
                .arg(Arg::new("INPUT").help("Input word here").min_values(0))
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
            App::new("json")
                .about("Print result to JSON output")
                .arg(
                    Arg::new("INPUT")
                        .help("Input word here")
                        .takes_value(true)
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
