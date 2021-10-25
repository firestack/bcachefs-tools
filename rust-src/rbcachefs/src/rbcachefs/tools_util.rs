use std::ffi::CString;

type DeviceMounted = Option<DeviceMountState>;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum DeviceMountState {
	ReadWrite,
	ReadOnly,
}

pub fn dev_mounted<T: Into<Vec<u8>>>(path: T) -> DeviceMounted {
	let cstr = std::ffi::CString::new(path).expect("Bad CSTR");
	let ptr = cstr.into_raw();
	let ret_val = unsafe { crate::c::bcachefs::dev_mounted(ptr) };
	unsafe {
		drop(CString::from_raw(ptr));
	}
	match ret_val {
		0 => None,
		1 => Some(DeviceMountState::ReadOnly),
		2 => Some(DeviceMountState::ReadWrite),
		_ => unreachable!("BAD RET_VAL"),
	}
}
