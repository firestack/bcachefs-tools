const MAX_ERROR_NO: isize = 4095;
// #define MAX_ERRNO	4095

// #define IS_ERR_VALUE(x) unlikely((x) >= (unsigned long)-MAX_ERRNO)

#[tracing::instrument]
fn is_error_value(val: isize) -> bool {
   tracing::info!("ptr");
   val <= MAX_ERROR_NO
}

pub type Error = isize;
pub type Result<T> = std::result::Result<T, Error>;

#[tracing::instrument]
pub fn ptr_result<'ptr, T>(ptr: *mut T ) -> Result<&'ptr mut T> {
   match is_error_value(ptr as isize) {
      true => Err(ptr as isize),
      false => Ok(unsafe { &mut *ptr })
   }
}