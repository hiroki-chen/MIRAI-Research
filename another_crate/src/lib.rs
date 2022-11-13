#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

extern crate mirai_annotations;

use mirai_annotations::*;
pub struct SecretTaintKind<const MASK: TagPropagationSet> {}

pub const SECRET_TAINT: TagPropagationSet = TAG_PROPAGATION_ALL;

pub type SecretTaint = SecretTaintKind<SECRET_TAINT>;
pub struct Wrapper {
    pub content: u32,
}
