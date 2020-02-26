mod lib;
use lib::spdbify_proteomes;
extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("spdbify")
    .version("1.0")
    .author("Kevin Amses <amsesk@umich.edu>")
    .about("This crate simply prepares protein FASTAs and lineage information for incorporation into swissprot-style databases used by SCGid.")
    .arg(Arg::with_name("mappings")
    	.long("map")
    	.value_name("PATH")
    	.help("Path to three-columned map (tsv) mapping organism tags (1) to the full paths to proteome fastas (2), and organism lineage information (3).")
    	.takes_value(true))
    .arg(Arg::with_name("output")
    	.long("out")
    	.value_name("PATH")
    	.help("Path to output concatenated, renamed protein FASTA. Directories must exist.")
    	.takes_value(true))
    .get_matches();

    spdbify_proteomes(
        matches.value_of("mappings").unwrap(),
        matches.value_of("output").unwrap(),
    );
}
