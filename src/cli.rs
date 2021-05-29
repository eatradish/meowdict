use clap::{crate_version, App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("meowdict")
        .version(crate_version!())
        .author("Mag Mell")
        .about("Check chinese keyword from moedict.tw")
        .arg(
            Arg::with_name("INPUT")
                .help("Input the keyword to use")
                .index(1)
                .min_values(1),
        )
        .arg(
            Arg::with_name("translation")
                .short("t")
                .long("translation")
                .help("Get all translation"),
        )
        .arg(
            Arg::with_name("inputs2t")
            .short("i")
            .long("input-s2t")
            .help("Convert input to traditional Chinese and search"),
        )
}
