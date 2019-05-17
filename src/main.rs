extern crate clap;
extern crate notify;

use clap::{Arg, App};
//use crossbeam_channel::{unbounded};
use std::path::{Path};

mod lib;
use lib::{watch_and_archive};


fn main() {

    let matches = App::new("SArchive")
        .version("0.1.0")
        .author("Andy Georges <itkovian+sarchive@gmail.com>")
        .about("Archive slurm user job scripts.")
        .arg(Arg::with_name("archive")
            .long("archive")
            .short("a")
            .takes_value(true)
            .help("Location of the job scripts' archive.")
        )
        .arg(Arg::with_name("cluster")
            .long("cluster")
            .short("c")
            .takes_value(true)
            .help("Name of the cluster where the jobs have been submitted to.")
        )
        .arg(Arg::with_name("spool")
            .long("spool")
            .short("s")
            .takes_value(true)
            .help("Location of the Slurm StateSaveLocation (where the job hash dirs are kept).")
        )
        .get_matches();

    let base = Path::new(matches.value_of("spool").unwrap());
    let archive = Path::new(matches.value_of("archive").unwrap());

    // TODO: check the base exists

    watch_and_archive(&archive, &base, 0);
}