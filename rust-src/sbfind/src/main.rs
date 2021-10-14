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
	#[structopt(short,long)]
	rewrite_superblocks: bool,
}

fn main() {
	tracing_subscriber::fmt::init();
	let r = inner();
	if let Err(e) = r {
		tracing::error!(fatal_error=?e);
	}
}
type ARResult<T> = anyhow::Result< anyhow::Result<T> >;

#[tracing::instrument]
fn inner() -> anyhow::Result<()> {
	// use std::io::{Error, ErrorKind};

	let args = Options::from_args();

	let _sb = find_working_superblock(&args.device, args.sb_offset)??;
	let sb = _sb.sb();

	let blocks: Vec<_> = sb.layout.sb_offset.iter().take(sb.layout.nr_superblocks as usize).copied().collect();

	check_layout_offsets(&args.device, &blocks[..])?.map(|_| ())?;
	Ok(())
}


#[tracing::instrument]
fn find_working_superblock(device: &std::path::Path, search: Option<u64>) -> ARResult<bch_bindgen::bcachefs::bch_sb_handle> {
	let offsets = vec![search.unwrap_or(8), 2056];
	tracing::debug!(msg="searching default offsets", ?offsets);

	let mut handle = None;
	for offset in offsets { // can't map because of outter error propogation

		match check_sector_super(device, offset)? {
			Ok(sbhandle) => { handle = Some(sbhandle); break; },
			Err(_) => { continue; }
		}
	}
	Ok(Ok(handle.expect("No SuperBlock Found")))
}

#[tracing::instrument(skip(device, blocks))]
fn check_layout_offsets(device: &std::path::Path, blocks: &[u64]) -> ARResult<Vec<bch_bindgen::bcachefs::bch_sb_handle>> {
	// tracing::info!("searching blocks");
	let mut v = vec![];
	
	for offset in blocks {
		let rhandle = check_sector_super(device, *offset)?;
		// tracing::debug!(?rhandle);
		if let Ok(handle) = rhandle {
			v.push(handle);
		}
	}
	Ok(Ok(v))
}

#[tracing::instrument]
fn check_sector_super(device: &std::path::Path, block: u64) -> ARResult<bch_bindgen::bcachefs::bch_sb_handle> {
	let rhandle = superblock_equality_check(device, block)?;
	match rhandle {
		Ok(handle) => {
			let sb = handle.sb();
			trace_superblock(sb);
			Ok(Ok(handle))
		},
		Err(err) => {
			tracing::warn!(?err, msg="SuperBlock Failed");
			Ok(Err(err))
		},
	}
}

fn trace_superblock(sb: &bch_bindgen::c::bch_sb) {
	tracing::info!(
		?sb
		// uuid=?sb.uuid(),//uuid::Uuid::from_slice(&sb.layout.uuid.b[..]),
		// sb_sector=?sb.offset,
		// byte_offset=?sb.offset*512,
		// ?sb.csum.lo,
		// ?sb.csum.hi,
	);
}

type RResult<T> = std::io::Result<std::io::Result<T>>;
mod c {
	pub fn read_superblock_from_sector(device: &std::path::Path, sector: u64) -> super::RResult<bch_bindgen::bcachefs::bch_sb_handle> {
		let mut opts = bch_bindgen::bcachefs::bch_opts {
			nochanges: 1,
			noexcl: 1,
			sb: sector,
			..Default::default()
		};
		opts.set_nochanges_defined(1);
		opts.set_noexcl_defined(1);
		opts.set_sb_defined(1);

		bch_bindgen::rs::read_super_opts(device, opts)
	}

	// fn _rebuild_superblocks(device: &std::path::Path, superblock: &mut bch_bindgen::c::bch_sb) -> std::io::Result<()> {
	// 	use std::os::unix::io::AsRawFd;
	// 	unsafe {
	// 		bch_bindgen::c::bch2_super_write_fd(
	// 			std::fs::OpenOptions::new().write(true).open(device)?.as_raw_fd()
	// 			, superblock
	// 		);
	// 	}
	// 	Ok(())
	// }
}

mod rs {
	const SECTOR_SIZE: usize = 512;
	// #[tracing::instrument(skip(device))]
	pub fn read_sector_as_superblock(device: &std::path::Path, sector: u64) -> anyhow::Result<bch_bindgen::c::bch_sb> {
		use std::os::unix::fs::FileExt;
		
		let mut sb = [0; std::mem::size_of::<bch_bindgen::c::bch_sb>()];
	
		std::fs::File::open(device)?
			.read_exact_at(&mut sb, sector*SECTOR_SIZE as u64)?;
		
		let sb = unsafe { std::mem::transmute::<_, bch_bindgen::c::bch_sb>(sb) };
		
		anyhow::ensure!(uuid::Uuid::from_bytes(sb.magic.b) == bch_bindgen::rs::SUPERBLOCK_MAGIC);
		anyhow::ensure!(sb.offset == sector);

		Ok(sb)
	}
}

fn superblock_equality_check(device: &std::path::Path, sector: u64) -> ARResult<bch_bindgen::c::bch_sb_handle> {
	let a = rs::read_sector_as_superblock(&device, sector)?;
	let b = c::read_superblock_from_sector(&device, sector)??;
	anyhow::ensure!(a == *(b.sb()));
	Ok(Ok(b))
}

#[cfg(test)]
mod test {
	#[test]
	fn test_uuid() {
		assert_eq!(uuid::Uuid::from_bytes(unsafe {bch_bindgen::c::BCH_FS_MAGIC.b}), bch_bindgen::rs::SUPERBLOCK_MAGIC);
	}
}