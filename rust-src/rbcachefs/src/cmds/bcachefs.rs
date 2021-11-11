use structopt::StructOpt;

#[no_mangle]
pub extern fn bcachefs_start() -> i32 {
	let opts = dbg!(Options::from_args());

	match opts.subcommand {
		SubCommand::Format(_) => {

		},
		SubCommand::ShowSuper(opt) => crate::cmds::cmd_show_super::show_super(opt),
		SubCommand::Fsck => todo!(),
		SubCommand::Fs(_) => todo!(),
		SubCommand::Device(_) => todo!(),
		SubCommand::Subvolume(_) => todo!(),
		SubCommand::Data(_) => todo!(),
		SubCommand::Encryption(_) => todo!(),
		SubCommand::Migration(_) => todo!(),
		SubCommand::Setattr(_) => todo!(),
		SubCommand::Debug(_) => todo!(),
	}
	0
}

#[derive(StructOpt, Debug)]
#[structopt(name="bcachefs", version=option_env!("VERSION").or(option_env!("CARGO_PKG_VERSION")).unwrap_or("v0.1") )]
/// Bcachefs commands
/// 
/// Report bugs to <linux-bcachefs@vger.kernel.org>
struct Options {
	#[structopt(subcommand)]
	subcommand: SubCommand

}

#[derive(StructOpt, Debug)]
enum SubCommand {
	/// Format a new filesystem
	Format(crate::cmds::cmd_format::Options),

	/// Dump superblock information to stdout
	ShowSuper(crate::cmds::cmd_show_super::Options),

	/// Run Filesystem Check on Offline Device
	/// Check an existing filesystem for errors
	Fsck,
	
	// / Startup/shutdown, assembly of multi device filesystems
	// MultiDev(MultiDeviceCommands),

	/// Show disk usage
	Fs(FileSystemCommands),
	/// Commands for managing devices within a running filesystem
	Device(DeviceCommands),
	/// Commands for managing subvolumes and snapshots
	Subvolume(SubvolumeCommands),
	/// Commands for managing filesystem data
	Data(FileSystemDataCommands),
	/// Commands for managing Filesystem Encryption
	Encryption(EncryptionCommands),
	/// Migrate
	Migration(MigrationCommands),
	/// Commands for operating on files in a bcachefs filesystem
	Setattr(SetFAttrCommand),
	/// Commands that operate on offline, unmounted filesystems
	Debug(DebugCommands),

	// / Miscellaneous
	// Miscellaneous(MiscCommands),
}

#[derive(StructOpt, Debug)]
enum MultiDeviceCommands {
	Assemble,
	Incremental,
	Run,
	Stop,
}

#[derive(StructOpt, Debug)]
enum DeviceCommands {
	Add,
	Remove,
	Online,
	Offline,
	Evacuate,
	SetState,
	Resize,
	ResizeJournal,
}

#[derive(StructOpt, Debug)]
enum SubvolumeCommands {
	Create,
	Delete,
	Snapshot,
}

#[derive(StructOpt, Debug)]
enum FileSystemCommands {
	Usage {
		#[structopt(default_value="./.")]
		/// The path of the filesystem to inspect 
		fs: std::path::PathBuf,
		#[structopt(short, long="human")]
		/// Option to print units in human readable form
		human_readable: bool,
	},
}

#[derive(StructOpt, Debug)]
enum FileSystemDataCommands {
	Rereplicate,
	Job,
}

#[derive(StructOpt, Debug)]
enum EncryptionCommands {
	Unlock,
	SetPassphrase,
	RemovePassphrase,
}

#[derive(StructOpt, Debug)]
enum MigrationCommands {
	Migrate,
	MigrateSuperblock,
}

#[derive(StructOpt, Debug)]
struct SetFAttrCommand {
}

#[derive(StructOpt, Debug)]
enum DebugCommands {
	/// Dump filesystem metadata to a qcow2 image
	Dump {
		#[structopt(short, long)]
		/// Output qcow2 image(s)
		output: std::path::PathBuf,
		#[structopt(short, long)]
		/// Force; overwrite when needed
		force: bool,
	},
	/// List filesystem metadata in textual form
	List,
	/// List contents of journal
	ListJournal

}

#[derive(StructOpt, Debug)]
enum MiscCommands {
	Version
}

