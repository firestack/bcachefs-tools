use crate::{call_err_fn_for_c, rbcachefs::tools_util::DeviceMountState};

#[no_mangle]
pub extern "C" fn RS_cmd_fsck_main() -> i32 {
	call_err_fn_for_c(|| fsck())
}

fn mk_opts(args: &Options) -> crate::c::bcachefs::bch_opts {
	use crate::c::bcachefs::fsck_err_opts;
	// opt_set(opts, degraded, true);
	// opt_set(opts, fsck, true);
	// opt_set(opts, fix_errors, FSCK_OPT_ASK);
	// opt_set(opts, fix_errors, FSCK_OPT_YES);
	// opt_set(opts, fix_errors, FSCK_OPT_YES);
	// opt_set(opts, nochanges, true);
	// opt_set(opts, fix_errors, FSCK_OPT_NO);
	// opt_set(opts, reconstruct_alloc, true);
	// opt_set(opts, verbose, true);
	let mut opts = crate::c::bcachefs::bch_opts {
		fsck: true as u8,
		degraded: true as u8,
		reconstruct_alloc: args.reconstruct_alloc as u8,
		verbose: args.verbose as u8,
		nochanges: args.dry_run as u8,
		fix_errors: match (args.dry_run, args.auto_repair.or(args.assume_yes)) {
			(true, _) => fsck_err_opts::FSCK_OPT_NO,
			(_, Some(true)) => fsck_err_opts::FSCK_OPT_YES,
			_ => fsck_err_opts::FSCK_OPT_ASK,
		} as u8,
		..Default::default()
	};
	opts.set_fsck_defined(1);
	opts.set_degraded_defined(1);
	opts.set_fix_errors_defined(1);
	opts.set_fsck_defined(1);
	if args.reconstruct_alloc {
		opts.set_reconstruct_alloc_defined(1);
	}
	opts.set_verbose_defined(1);
	opts.set_nochanges_defined(1);

	args.options.as_ref().map(|i| parse_mount_options(&i, &mut opts));

	opts
}

fn parse_mount_options(options: &str, opts: &mut crate::c::bcachefs::bch_opts) {
	let v = CString::new(options.clone())
		.expect("Failed to convert options to cstr")
		.into_raw();

	let ret = unsafe {
		let ret = crate::c::bcachefs::bch2_parse_mount_opts(std::ptr::null_mut(), opts, v);
		// CString::from_raw(v); // cleanup: Drop string
		ret
	};

	if ret != 0 {
		panic!("oh no");
	}
}

use std::{ffi::CString, ops::Deref, os::unix::prelude::OsStrExt, path::PathBuf};
fn check_devices_mounted(devices: &Vec<PathBuf>) -> anyhow::Result<()> {
	for dev in devices {
		let buffer = dev.as_os_str().as_bytes();
		let path = std::ffi::CString::new(buffer)?;
		let mounted = crate::rbcachefs::tools_util::dev_mounted(path);

		if mounted == Some(DeviceMountState::ReadWrite) {
			Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				"Devices Mounted ReadWrite, Exiting",
			))?;
		}
	}
	Ok(())
}

fn fsck() -> anyhow::Result<()> {
	// FSCK Expects a return of bit 16 if this fails
	// 16: Usage or syntax error
	let args = Options::from_args();
	tracing::trace!(?args);

	check_devices_mounted(&args.devices)?;

	let devs: Vec<Vec<u8>> = args
		.devices
		.iter()
		.map(|i| i.deref().as_os_str().as_bytes().into())
		.collect();
	// 	.map(Deref::deref)
	// 	.map(Path::as_os_str)
	// 	.map(std::os::unix::prelude::OsStrExt::as_bytes)
	// 	.collect();

	let opts = mk_opts(&args);
	let rc_fs = crate::c::bcachefs::bch_fs::try_new(devs.as_slice(), opts);
	let c_fs = match rc_fs {
		Ok(t) => t,
		Err(i) => anyhow::bail!(i),
	};

	// if (test_bit(BCH_FS_ERRORS_FIXED, &c->flags)) {
	// 	fprintf(stderr, "%s: errors fixed\n", c->name);
	// 	ret |= 1; // Filesystem Errors Corrected
	// }
	// if (test_bit(BCH_FS_ERROR, &c->flags)) {
	// 	fprintf(stderr, "%s: still has errors\n", c->name);
	// 	ret |= 4; // Filesystem Errors Left Uncorrected
	// }

	unsafe {
		crate::c::bcachefs::bch2_fs_stop(c_fs);
	}
	// return ret;

	Ok(())
}

// fn parse_mount_options(src: &str) -> (String, Option<String>) {
// 	match src.split_once("=") {
// 		Some((astr, bstr)) => (astr.to_owned(), Some(bstr.to_owned())),
// 		None => (src.to_owned(), None),
// 	}
// }

use structopt::StructOpt;
/// bcachefs fsck - filesystem check and repair
#[derive(StructOpt, Debug)]
pub struct Options {
	/// Automatic Repair (No Questions)
	#[structopt(short = "p", long)]
	pub auto_repair: Option<bool>,

	/// Don't repair, only check for errors
	#[structopt(short = "n", long)]
	pub dry_run: bool,

	/// Assume "yes" to all questions
	#[structopt(short = "y", long)]
	pub assume_yes: Option<bool>,

	/// Force checking even if filesystem is marked clean
	#[structopt(short = "f", long)]
	pub force_fs_check: bool,

	/// Reconstruct the alloc btree
	#[structopt(short = "R", long)]
	pub reconstruct_alloc: bool,

	/// Be verbose
	#[structopt(short, long)]
	pub verbose: bool,

	/// Devices
	#[structopt(required(true))]
	pub devices: Vec<PathBuf>,

	// /// Options List
	// #[structopt(short="o", long, parse(from_str = parse_mount_options))]
	// pub options: Vec<(String, Option<String>)>,
	/// Options List
	#[structopt(short = "o", long)]
	pub options: Option<String>,
}

// pub struct OptionsFstab {
// 	/// Allowed
// 	/// UUID=
// 	/// LABEL=
// 	/// device
// 	pub fs_spec: String
// 	pub fs_file
// 	pub fs_vfstype
// 	pub fs_mnt_opts
// 	pub fs_freq: Option<>
// 	pub fs_passno:

// }
