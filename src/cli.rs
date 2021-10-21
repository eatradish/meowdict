use clap::{crate_version, App, Arg, SubCommand};

macro_rules! meowdict_subcommand {
    ($subcommand:expr, $about:expr, $alias:expr) => {
        $subcommand
            .about($about)
            .alias($alias)
            .arg(Arg::with_name("INPUT").help("Input the keyword to use"))
    };
}

macro_rules! meowdict_subcommand_with_s2t2s {
    ($subcommand:expr) => {
        $subcommand
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
    };
}

macro_rules! meowdict_subcommand_with_no_color {
    ($subcommand:expr) => {
        $subcommand.arg(
            Arg::with_name("no-color-output")
                .long("no-color-output")
                .help("Print result with no color")
                .requires("INPUT"),
        )
    };
}

macro_rules! meowdict_subcommand_with_mode {
    ($subcommand:expr) => {
        $subcommand
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
    };
}

pub fn build_cli() -> App<'static, 'static> {
    let meowdict_subcommand = meowdict_subcommand_with_s2t2s!(App::new("meowdict")
        .version(crate_version!())
        .author("Mag Mell")
        .about("Search chinese keyword from moedict.tw")
        .arg(
            Arg::with_name("INPUT")
                .help("Input the keyword to use")
                .index(1)
                .min_values(1),
        )
        .subcommand({
            let meowdict_subcommand =
                meowdict_subcommand!(SubCommand::with_name("show"), "Get dict result", "show");
            let meowdict_subcommand = meowdict_subcommand_with_s2t2s!(meowdict_subcommand);
            let meowdict_subcommand = meowdict_subcommand_with_no_color!(meowdict_subcommand);

            meowdict_subcommand
        })
        .subcommand({
            let meowdict_subcommand = meowdict_subcommand!(
                SubCommand::with_name("translate"),
                "Get word translation",
                "trans"
            );
            let meowdict_subcommand = meowdict_subcommand_with_s2t2s!(meowdict_subcommand);
            let meowdict_subcommand = meowdict_subcommand_with_no_color!(meowdict_subcommand);

            meowdict_subcommand
        })
        .subcommand({
            let meowdict_subcommand = meowdict_subcommand!(
                SubCommand::with_name("jyutping"),
                "Get word jyutping",
                "jyut"
            );
            let meowdict_subcommand = meowdict_subcommand_with_s2t2s!(meowdict_subcommand);
            let meowdict_subcommand = meowdict_subcommand_with_no_color!(meowdict_subcommand);

            meowdict_subcommand
        })
        .subcommand({
            let meowdict_subcommand = meowdict_subcommand!(
                SubCommand::with_name("terminal"),
                "Open meowdict terminal",
                "term"
            );
            let meowdict_subcommand = meowdict_subcommand_with_no_color!(meowdict_subcommand);
            let meowdict_subcommand = meowdict_subcommand_with_mode!(meowdict_subcommand);

            meowdict_subcommand
        })
        .subcommand({
            let meowdict_subcommand = meowdict_subcommand!(
                SubCommand::with_name("random"),
                "search random word",
                "rand"
            );
            let meowdict_subcommand = meowdict_subcommand_with_s2t2s!(meowdict_subcommand);
            let meowdict_subcommand = meowdict_subcommand_with_no_color!(meowdict_subcommand);

            meowdict_subcommand
        })
        .subcommand({
            let meowdict_subcommand = meowdict_subcommand!(
                SubCommand::with_name("json"),
                "Print result to JSON output",
                "json"
            );
            let meowdict_subcommand = meowdict_subcommand_with_s2t2s!(meowdict_subcommand);

            meowdict_subcommand
        }));

    let meowdict_subcommand = meowdict_subcommand_with_no_color!(meowdict_subcommand);
    let meowdict_subcommand = meowdict_subcommand_with_mode!(meowdict_subcommand);

    meowdict_subcommand
}
