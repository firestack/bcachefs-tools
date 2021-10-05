use crate::bcachefs;

extern "C" {
	pub static stdout: *mut libc::FILE;
}

#[tracing_attributes::instrument]
pub fn read_super(path: &std::path::Path) -> std::io::Result<(bcachefs::bch_sb_handle, bcachefs::bch_opts)> {
	// let devp = camino::Utf8Path::from_path(devp).unwrap();
	
	use std::os::unix::ffi::OsStrExt;
	let path = std::ffi::CString::new(path.as_os_str().as_bytes())?;
	
	let mut opts = std::mem::MaybeUninit::zeroed();
	let mut sb = std::mem::MaybeUninit::zeroed();
	
	use gag::{BufferRedirect};
	// Stop libbcachefs from spamming the output
	let gag = BufferRedirect::stderr().unwrap();
	tracing::trace!("entering libbcachefs");
	let ret = unsafe { crate::bcachefs::bch2_read_super(
		path.as_ptr(),
		opts.as_mut_ptr(),
		sb.as_mut_ptr(),
	)};
	unsafe { libc::fflush(stdout); }
	// Flush stdout so buffered output don't get printed after we remove the gag
	let mut buf = String::new();

	use std::io::Read;
	let data = { gag.into_inner().read_to_string(&mut buf)? };
	
	tracing::trace!(result=%data, log=%buf);

	match -ret {
		0 => Ok( unsafe {(
			sb.assume_init(),
			opts.assume_init(),
		)}),
		libc::EACCES => Err(std::io::Error::new(
			std::io::ErrorKind::PermissionDenied,
			"Access Permission Denied",
		)),
		22 => Err(std::io::Error::new(
			std::io::ErrorKind::InvalidData,
			"Not a BCacheFS SuperBlock",
		)),
		code => {
			tracing::debug!(msg="BCacheFS return error code", ?code);
			Err(std::io::Error::new(
				std::io::ErrorKind::Other,
				"Failed to Read SuperBlock",
		))}
	}
}