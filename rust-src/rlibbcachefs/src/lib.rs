mod rs {
	const SECTOR_SIZE: usize = 512;
	// #[tracing::instrument(skip(device))]
	pub fn read_sector_as_superblock(device: &std::path::Path, sector: u64) -> anyhow::Result<bch_bindgen::c::bch_sb> {
		use std::os::unix::fs::FileExt;
		
		let mut sb = [0; std::mem::size_of::<bch_bindgen::c::bch_sb>()];
	
		std::fs::File::open(device)?
			.read_exact_at(&mut sb, sector*SECTOR_SIZE as u64)?;
		
		let sb = unsafe { std::mem::transmute::<_, bch_bindgen::c::bch_sb>(sb) };
		
		anyhow::ensure!(sb.magic.b == unsafe{ bch_bindgen::c::BCH_FS_MAGIC.b });
		anyhow::ensure!(sb.offset == sector);

		Ok(sb)
	}
}



#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let a = Point(1,1);
		assert_eq!(2 + 2, 4);
	}
}
