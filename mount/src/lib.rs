use anyhow::anyhow;
use structopt::StructOpt;

#[macro_export]
macro_rules! c_str {
	($lit:expr) => {
		unsafe {
			std::ffi::CStr::from_ptr(concat!($lit, "\0").as_ptr() as *const std::os::raw::c_char)
				.to_bytes_with_nul()
				.as_ptr() as *const std::os::raw::c_char
		}
	};
}

#[derive(Debug)]
struct ErrnoError(errno::Errno);
impl std::fmt::Display for ErrnoError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		self.0.fmt(f)
	}
}
impl std::error::Error for ErrnoError {}

#[derive(Debug)]
pub(crate) enum KeyLocation {
	Fail,
	Wait,
	Ask,
}

#[derive(Debug)]
struct KeyLoc(Option<KeyLocation>);
impl std::ops::Deref for KeyLoc {
	type Target = Option<KeyLocation>;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
impl std::str::FromStr for KeyLoc {
	type Err = anyhow::Error;
	fn from_str(s: &str) -> anyhow::Result<Self> {
		// use anyhow::anyhow;
		match s {
			"" => Ok(KeyLoc(None)),
			"fail" => Ok(KeyLoc(Some(KeyLocation::Fail))),
			"wait" => Ok(KeyLoc(Some(KeyLocation::Wait))),
			"ask" => Ok(KeyLoc(Some(KeyLocation::Ask))),
			_ => Err(anyhow!("invalid password option")),
		}
	}
}

#[derive(StructOpt, Debug)]
/// Mount a bcachefs filesystem by its UUID.
struct Options {
	/// Where the password would be loaded from.
	///
	/// Possible values are:
	/// "fail" - don't ask for password, fail if filesystem is encrypted;
	/// "wait" - wait for password to become available before mounting;
	/// "ask" -  prompt the user for password;
	#[structopt(short, long, default_value = "")]
	key_location: KeyLoc,

	/// External UUID of the bcachefs filesystem
	uuid: uuid::Uuid,

	/// Where the filesystem should be mounted. If not set, then the filesystem
	/// won't actually be mounted. But all steps preceeding mounting the
	/// filesystem (e.g. asking for passphrase) will still be performed.
	mountpoint: Option<std::path::PathBuf>,

	/// Mount options
	#[structopt(short, default_value = "")]
	options: String,
}

mod filesystem;
mod key;

#[tracing_attributes::instrument("main")]
pub fn main_inner() -> anyhow::Result<()> {
	unsafe {
		libc::setvbuf(
			crate::filesystem::stdout,
			std::ptr::null_mut(),
			libc::_IONBF,
			0,
		);
		// libc::fflush(crate::filesystem::stdout);
	}
	let opt = Options::from_args_safe()?;
	tracing::trace!(?opt);

	let fss = filesystem::probe_filesystems()?;
	let fs = fss
		.get(&opt.uuid)
		.ok_or_else(|| anyhow::anyhow!("filesystem was not found"))?;

	tracing::info!(msg="found filesystem", %fs);
	if fs.encrypted() {
		let key = opt
			.key_location
			.0
			.ok_or_else(|| anyhow::anyhow!("no keyoption specified for locked filesystem"))?;

		key::prepare_key(&fs, key)?;
	}

	let mountpoint = opt
		.mountpoint
		.ok_or_else(|| anyhow::anyhow!("mountpoint option was not specified"))?;

	fs.mount(&mountpoint, &opt.options)?;

	Ok(())
}

// pub fn mnt_in_use()
