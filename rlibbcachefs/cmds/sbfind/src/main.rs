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

	let _sb = find_working_superblock(&args.device, args.sb_offset)??;
	let sb = _sb.sb();

	let blocks: Vec<_> = sb.layout.sb_offset.iter().take(sb.layout.nr_superblocks as usize).copied().collect();

	check_block_locations(&args.device, &blocks[..])?.map(|_| ())
}

fn rebuild_superblock(device: &std::path::Path, superblock: &mut rlibbcachefs::c::bch_sb) -> std::io::Result<()> {
	use std::os::unix::io::AsRawFd;
	unsafe {
		rlibbcachefs::c::bch2_super_write_fd(
			std::fs::OpenOptions::new().write(true).open(device)?.as_raw_fd()
			, superblock
		);
	}
	Ok(())
}

#[tracing::instrument]
fn find_working_superblock(device: &std::path::Path, search: Option<u64>) -> RResult<rlibbcachefs::bcachefs::bch_sb_handle> {
	let offsets = vec![search.unwrap_or(8), 2056];
	tracing::debug!(msg="searching default offsets", ?offsets);

	let mut handle = None;
	for offset in offsets { // can't map because of outter error propogation

		match check_super_offset(device, offset)? {
			Ok(sbhandle) => { handle = Some(sbhandle); break; },
			Err(_) => { continue; }
		}
	}
	Ok(Ok(handle.expect("No SuperBlock Found")))
}

#[tracing::instrument(skip(device, blocks))]
fn check_block_locations(device: &std::path::Path, blocks: &[u64]) -> RResult<Vec<rlibbcachefs::bcachefs::bch_sb_handle>> {
	// tracing::info!("searching blocks");
	let mut v = vec![];
	
	for offset in blocks {
		let rhandle = check_super_offset(device, *offset)?;
		// tracing::debug!(?rhandle);
		if let Ok(handle) = rhandle {
			v.push(handle);
		}
	}
	Ok(Ok(v))
}

unsafe fn read_super(device: &std::path::Path, offset: u64) -> std::io::Result<rlibbcachefs::c::bch_sb> {
	use std::os::unix::{fs::FileExt};
	
	let mut sb = [0; std::mem::size_of::<rlibbcachefs::c::bch_sb>()];

	std::fs::File::open(device)?.read_exact_at(&mut sb, offset*512 as u64)?;
	let sb = std::mem::transmute::<_, rlibbcachefs::c::bch_sb>(sb);
	
	assert_eq!(sb.magic.b, rlibbcachefs::c::BCH_FS_MAGIC.b);
	assert_eq!(sb.offset, offset);
	trace_superblock(&sb);
	
	Ok(sb)
}

#[tracing::instrument]
fn check_super_offset(device: &std::path::Path, offset: u64) -> RResult<rlibbcachefs::bcachefs::bch_sb_handle> {
	let rhandle = read_offset_block(device, offset)?;
	match rhandle {
		Ok(handle) => {
			let sb = handle.sb();
			// tracing::info!(
			// 	uuid=?sb.uuid(),//uuid::Uuid::from_slice(&sb.layout.uuid.b[..]),
			// 	sb_sector=?sb.offset,
			// 	byte_offset=?sb.offset*512
			// );
			trace_superblock(sb);
			Ok(Ok(handle))
		},
		Err(err) => {
			tracing::warn!(?err, msg="SuperBlock Failed");
			Ok(Err(err))
		},
	}
}

fn trace_superblock(sb: &rlibbcachefs::c::bch_sb) {
	tracing::info!(
		uuid=?sb.uuid(),//uuid::Uuid::from_slice(&sb.layout.uuid.b[..]),
		sb_sector=?sb.offset,
		byte_offset=?sb.offset*512,
		?sb.csum.lo,
		?sb.csum.hi,
	);
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