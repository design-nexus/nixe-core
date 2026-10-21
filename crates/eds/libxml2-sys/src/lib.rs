//! The libxml2 types that libcamel and libedataserver mention in their headers.
//! They are only ever passed by pointer, so they are opaque here; no libxml2
//! function is bound.

#![allow(non_camel_case_types)]

#[repr(C)]
pub struct xmlNode {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct xmlDoc {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct xmlNs {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub type xmlNodePtr = *mut xmlNode;
pub type xmlDocPtr = *mut xmlDoc;
pub type xmlNsPtr = *mut xmlNs;
pub type xmlChar = u8;
