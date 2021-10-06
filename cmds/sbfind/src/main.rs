use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(
	name = "sbfind",
	about = "A utility to read all expected superblocks possibly based on another disk"
)]
struct Options {
	// clone_disk,
	device: std::path::PathBuf,
	sb_offset: Option<u64>,
}

fn main() {
	tracing_subscriber::fmt::init();
	let r = inner();
	if let Err(e) = r {
		println!("Error: {:?}", e);
	}
}

fn inner() -> std::io::Result<()> {
	use std::io::{Error, ErrorKind};

	let mut args = Options::from_args();
	let offsets = vec![8, 2056, args.sb_offset.unwrap_or(0)];
	tracing::debug!(?offsets);
	let _sb = offsets.iter().find_map(|i| read_offset_block(&args.device, *i).ok()).expect("No SuperBlock Found");
	// .expect("couldn't find superblock")?;
	let sb = _sb.sb();
	let layout = sb.layout;

	let blocks: Vec<_> = layout.sb_offset.iter().take(layout.nr_superblocks as usize).collect();
	dbg!(&blocks);
	for i in &blocks {

		let _sb = read_offset_block(&args.device, **i);
		dbg!(_sb.map(|i| i.sb().layout.magic));
	}
	Ok(())
}

fn read_offset_block(device: &std::path::Path, offset: u64) -> Result<rlibbcachefs::bcachefs::bch_sb_handle, std::io::Error> {
	let mut opts = rlibbcachefs::bcachefs::bch_opts {
		nochanges: 1,
		noexcl: 1,
		sb: offset,
		..Default::default()
	};
	opts.set_nochanges_defined(1);
	opts.set_noexcl_defined(1);
	opts.set_sb_defined(1);

	rlibbcachefs::rs::read_super_opts(device, opts)?.map(|i| i.0)
}