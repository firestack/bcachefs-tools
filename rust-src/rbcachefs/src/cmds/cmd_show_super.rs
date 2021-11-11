pub fn show_super(opts: Options) {

}

use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub struct Options {
	#[structopt(short, long)]
	layouts: bool,

	#[structopt(short, long)]
	fields: Vec<String>,
}