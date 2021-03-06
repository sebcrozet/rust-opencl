/*! # rust-opencl

OpenCL bindings and high-level interface for Rust.

## Installation

Add the following to your `Cargo.toml` file:

```.ignore
[dependencies] 
rust-opencl = "0.5.0"
```
*/


#![allow(improper_ctypes)]
#![allow(missing_copy_implementations)]

#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unused_imports)]
#![warn(missing_docs)]

#![feature(static_mutex)]


extern crate libc;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;

#[link(name = "OpenCL", kind = "framework")]
#[cfg(target_os = "macos")]
extern { }

#[link(name = "OpenCL")]
#[cfg(target_os = "linux")]
extern { }

pub use platform::{Platform, platforms};
pub use device::{Device, DeviceType};
pub use context::Context;
pub use command_queue::CommandQueue;
pub use program::{Program, Kernel, KernelArg, KernelIndex};
pub use event::{Event, EventList};

pub use buffer::{BufferData, Buffer};

pub use image::{Image2D, Image3D};
pub use hl::{PreferedType, create_compute_context, create_compute_context_prefer};

pub use cl::{ChannelOrder, MemoryAccess};

pub mod cl;
pub mod ext;
pub mod error;
pub mod image_channel_type;
mod hl;
mod buffer;
mod platform;
mod device;
mod context;
mod command_queue;
mod program;
mod event;
mod image;
