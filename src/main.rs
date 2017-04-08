extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("dialyze-rs")
        .version("0.1")
        .author("Nao YONASHIRO <owan.orisano@gmail.com>")
        .about("dialyzer implemented by rust")
        .arg(Arg::with_name("add_to_plt")
             .long("add_to_plt"))
        .arg(Arg::with_name("apps")
             .long("apps")
             .value_name("lib_dir")
             .takes_value(true))
        .arg(Arg::with_name("build_plt")
             .long("build_plt"))
        .arg(Arg::with_name("check_plt")
             .long("check_plt"))
        .arg(Arg::with_name("no_check_plt")
             .short("n")
             .long("no_check_plt"))
        .arg(Arg::with_name("no_native")
             .long("no_native"))
        .arg(Arg::with_name("no_native_cache")
             .long("no_native_cache"))
        .arg(Arg::with_name("plt_info")
             .long("plt_info"))
        .arg(Arg::with_name("get_warnings")
             .long("get_warnings"))
        .arg(Arg::with_name("output")
             .short("o")
             .long("output")
             .takes_value(true))
        .arg(Arg::with_name("output_plt")
             .long("output_plt")
             .takes_value(true))
        .arg(Arg::with_name("raw")
             .long("raw"))
        .arg(Arg::with_name("fullpath")
             .long("fullpath"))
        .get_matches();
    let add_to_plt = matches.is_present("add_to_plt");
    println!("{}", add_to_plt);
}
