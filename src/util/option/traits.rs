//! NOTE: this will be much better once specialization comes

use std::mem;
use std::ffi::CString;

use ffi::*;
use libc::{c_void, c_int, int64_t};
use ::{Error, Rational, ChannelLayout};
use util::format;

macro_rules! check {
	($expr:expr) => (
		match $expr {
			0 => Ok(()),
			e => Err(Error::from(e)),
		}
	)
}

pub unsafe trait Target {
	fn as_ptr(&self) -> *const c_void;
	fn as_mut_ptr(&mut self) -> *mut c_void;
}

pub trait Settable: Target {
	fn set<T: 'static>(&mut self, name: &str, value: &T) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set_bin(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				value as *const _ as *const _, mem::size_of::<T>() as c_int,
				AV_OPT_SEARCH_CHILDREN))
		}
	}

	fn set_str(&mut self, name: &str, value: &str) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				CString::new(value).unwrap().as_ptr(),
				AV_OPT_SEARCH_CHILDREN))
		}
	}

	fn set_int(&mut self, name: &str, value: i64) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set_int(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				value as int64_t,
				AV_OPT_SEARCH_CHILDREN))
		}
	}

	fn set_double(&mut self, name: &str, value: f64) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set_double(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				value,
				AV_OPT_SEARCH_CHILDREN))
		}
	}

	fn set_rational<T: Into<Rational>>(&mut self, name: &str, value: T) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set_q(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				value.into().into(),
				AV_OPT_SEARCH_CHILDREN))
		}
	}

	fn set_image_size(&mut self, name: &str, w: u32, h: u32) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set_image_size(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				w as c_int, h as c_int,
				AV_OPT_SEARCH_CHILDREN))
		}
	}

	fn set_pixel_format(&mut self, name: &str, format: format::Pixel) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set_pixel_fmt(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				format.into(),
				AV_OPT_SEARCH_CHILDREN))
		}
	}

	fn set_sample_format(&mut self, name: &str, format: format::Sample) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set_sample_fmt(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				format.into(),
				AV_OPT_SEARCH_CHILDREN))
		}
	}

	fn set_channel_layout(&mut self, name: &str, layout: ChannelLayout) -> Result<(), Error> {
		unsafe {
			check!(av_opt_set_channel_layout(self.as_mut_ptr(),
				CString::new(name).unwrap().as_ptr(),
				layout.bits() as int64_t,
				AV_OPT_SEARCH_CHILDREN))
		}
	}
}

pub trait Gettable: Target {

}

pub trait Iterable: Target {

}
