#![warn(warnings)]
#![cfg_attr(feature = "yew", recursion_limit = "1024")]

#[cfg(feature = "askama")]
pub mod askama;
pub mod html;
#[cfg(feature = "tera")]
pub mod tera;
#[cfg(feature = "yew")]
pub mod yew;

mod pager;
mod pagination;

pub use pagination::Pagination;

pub use pager::Pager;
