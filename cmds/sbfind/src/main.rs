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
		tracing::error!(fatal_error=?e);
	}
}

#[tracing::instrument]
fn inner() -> std::io::Result<()> {
	// use std::io::{Error, ErrorKind};

	let args = Options::from_args();
	use std::os::unix::io::IntoRawFd;

	// Want device filedescriptor for reuse
	let _sb = find_working_superblock(&args.device, args.sb_offset)??;
	// let _sb = offsets.iter().map(|i| read_offset_block(&args.device, *i)).expect("No SuperBlock Found")?;
	// .expect("couldn't find superblock")?;
	let sb = _sb.sb();
	let layout = sb.layout;

	let blocks: Vec<_> = layout.sb_offset.iter().take(layout.nr_superblocks as usize).copied().collect();

	Ok(check_block_locations(&args.device, &blocks[..])??)

}

#[tracing::instrument]
fn find_working_superblock(device: &std::path::Path, search: Option<u64>) -> RResult<rlibbcachefs::bcachefs::bch_sb_handle> {
	let offsets = vec![search.unwrap_or(8), 2056];
	tracing::debug!(msg="testing offsets", ?offsets);

	let mut handle = None;
	for offset in offsets { // can't map because of outter error propogation

		match check_super_offset(device, offset)? {
			Ok(sbhandle) => { handle = Some(sbhandle); break; },
			Err(e) => { tracing::debug!(msg="Failed to read superblock"); continue; }
		}
	}
	Ok(Ok(handle.expect("No SuperBlock Found")))
}

#[tracing::instrument]
fn check_block_locations(device: &std::path::Path, blocks: &[u64]) -> RResult<()> {
	tracing::info!("searching blocks");
	for offset in blocks {
		let rhandle = check_super_offset(device, *offset)?;
		tracing::debug!(?rhandle);
	}
	Ok(Ok(()))
}

#[tracing::instrument]
fn check_super_offset(device: &std::path::Path, offset: u64) -> RResult<rlibbcachefs::bcachefs::bch_sb_handle> {
	let rhandle = read_offset_block(device, offset)?;
	match rhandle {
		Ok(handle) => {
			let sb = handle.sb();
			tracing::info!(
				magic=?uuid::Uuid::from_slice(&sb.layout.magic.b[..]),
				disk_fd=?handle.bdev().bd_fd,
				sb_sector=?sb.offset,
				byte_offset=?sb.offset*512
			);
			Ok(Ok(handle))
		},
		Err(err) => {
			tracing::warn!(?err, msg="SuperBlock Failed");
			Ok(Err(err))
		},
	}
}

type RResult<T> = std::io::Result<std::io::Result<T>>;
fn read_offset_block(device: &std::path::Path, offset: u64) -> RResult<rlibbcachefs::bcachefs::bch_sb_handle> {
	let mut opts = rlibbcachefs::bcachefs::bch_opts {
		nochanges: 1,
		noexcl: 1,
		sb: offset,
		..Default::default()
	};
	opts.set_nochanges_defined(1);
	opts.set_noexcl_defined(1);
	opts.set_sb_defined(1);

	rlibbcachefs::rs::read_super_opts(device, opts)
}