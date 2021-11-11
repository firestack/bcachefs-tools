fn cmd_format(opts: Options) {

}

use structopt::StructOpt;
#[derive(StructOpt, Debug)]
pub struct Options {
	// 	--block_size=size
	#[structopt(long)]
	block_size: usize,
	// 	--btree_node_size=size  
	/// Btree node size
	#[structopt(long, default_value = "262144")]
	btree_node_size: usize,
	// 	--errors=(continue|ro|panic)
	#[structopt(long, possible_values(&["continue", "ro", "panic"]))]
	/// Action to take on filesystem error
	errors: String,
	// 	--metadata_replicas=#   Number of metadata replicas
	#[structopt(long)]
	metadata_replicas: usize,
	// 	--data_replicas=#       Number of data replicas
	#[structopt(long)]
	data_replicas: usize,
	// 	--metadata_replicas_required=#
	#[structopt(long)]
	metadata_replicas_required: usize,
	// 	--data_replicas_required=#
	#[structopt(long)]
	data_replicas_required: usize,
	// 	--replicas=#            Sets both data and metadata replicas
	#[structopt(long)]
	replicas: usize,
	// 	--metadata_checksum=(none|crc32c|crc64|xxhash)
	#[structopt(long,default_value="crc32c", possible_values(&[ "none", "crc32c", "crc64", "xxhash" ]))]
	metadata_checksum: String,
	// 	--data_checksum=(none|crc32c|crc64|xxhash)
	#[structopt(long,default_value="crc32c", possible_values(&[ "none", "crc32c", "crc64", "xxhash" ]))]
	data_checksum: String,
	// 	--compression=(none|lz4|gzip|zstd)
	// 	--background_compression=(none|lz4|gzip|zstd)
	// 	--str_hash=(crc32c|crc64|siphash)
	// 									Hash function for directory entries and xattrs
	// 	--metadata_target=(target)
	// 									Device or disk group for metadata writes
	// 	--foreground_target=(target)
	// 									Device or disk group for foreground writes
	// 	--background_target=(target)
	// 									Device or disk group to move data to in the background
	// 	--promote_target=(target)
	// 									Device or disk group to promote data to on read
	// 	--erasure_code          Enable erasure coding (DO NOT USE YET)
	// 	--inodes_32bit          Constrain inode numbers to 32 bits
	// 	--shard_inode_numbers   Shard new inode numbers by CPU id
	// 	--inodes_use_key_cache  Use the btree key cache for the inodes btree
	// 	--gc_reserve_percent=%  Percentage of disk space to reserve for copygc
	// 	--gc_reserve_bytes=%    Amount of disk space to reserve for copygc
	// 									Takes precedence over gc_reserve_percent if set
	// 	--root_reserve_percent=%
	// 									Percentage of disk space to reserve for superuser
	// 	--wide_macs             Store full 128 bits of cryptographic MACs, instead of 80
	// 	--acl                   Enable POSIX acls
	// 	--encrypted             Enable whole filesystem encryption (chacha20/poly1305)
	// 	--no_passphrase         Don't encrypt master encryption key
	// -L, --label=label
	// -U, --uuid=uuid
	// 	--superblock_size=size
}