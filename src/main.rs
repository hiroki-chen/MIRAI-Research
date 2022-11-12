#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![allow(dead_code)]

extern crate mirai_annotations;

use mirai_annotations::*;

struct SecretTaintKind<const MASK: TagPropagationSet> {}

const SECRET_TAINT: TagPropagationSet = TAG_PROPAGATION_ALL;

type SecretTaint = SecretTaintKind<SECRET_TAINT>;

struct Wrapper {
    content: u32,
}

struct VecWrapper {
    inner: Vec<u8>,
}

/// Tests whether adding tags to vector itself works.
fn test_vec_1() {
    let vec = Vec::<u8>::new();
    add_tag!(&vec, SecretTaint);
    // Works well.
    verify!(has_tag!(&vec, SecretTaint));
    // Also works well: warning: provably false verification condition.
    verify!(does_not_have_tag!(&vec, SecretTaint));
}

/// Tests whether adding tags to vector after a `push` works.
fn test_vec_2() {
    let mut vec = vec![];
    vec.push(0u8);
    add_tag!(&vec, SecretTaint);
    // Works well.
    verify!(has_tag!(&vec, SecretTaint));
    // Also works well: warning: provably false verification condition
    verify!(does_not_have_tag!(&vec, SecretTaint));
}

/// Tests whether adding tags to vector component (non-primitive type) works.
fn test_vec_3() {
    let mut vec = vec![];
    vec.push(Wrapper { content: 123 });
    add_tag!(&vec[0], SecretTaint);
    // Works well.
    verify!(has_tag!(&vec[0], SecretTaint));
    // Works well: warning: provably false verification condition
    verify!(does_not_have_tag!(&vec[0], SecretTaint));
}

/// Tests whether adding tags to vector component (non-primitive type) works.
/// Not working for basic types lile u8, u16, u32...
fn test_vec_4() {
    let mut vec = vec![];
    vec.push(123);
    add_tag!(&vec[0], SecretTaint);

    // None of the following is true.
    verify!(does_not_have_tag!(&vec[0], SecretTaint));
    verify!(has_tag!(&vec[0], SecretTaint));
}

/// Test whether secret tag will propagate from child to parent.
/// Not working.
fn test_vec_5(wrapper: Wrapper) {
    add_tag!(&wrapper, SecretTaint);
    let mut vec = vec![];
    vec.push(wrapper);
    verify!(has_tag!(&vec[0], SecretTaint));
}

/// Test whether secret tag will propagate from parent to child.
/// Not working.
fn test_vec_6() {
    let mut vec = vec![];
    vec.push(Wrapper { content: 123 });
    add_tag!(&vec, SecretTaint);
    verify!(has_tag!(&vec[0], SecretTaint));
}

/// Test whether secret tag can be added to some elements of the vector.
/// Not working.
fn test_vec_7() {
    let mut vec = vec![];
    vec.push(0u8);
    add_tag!(&vec, SecretTaint);
    // warning: possible false verification condition
    verify!(has_tag!(&vec[0], SecretTaint));
}

/// May relate to test_vec_4.
fn test_vec_8() {
    let vec = vec![0];
    add_tag!(&vec[0], SecretTaint);
    // Becomes unreachable.
    verify!(has_tag!(&vec[0], SecretTaint));
}

/// May relate to test_vec_4.
fn test_vec_9() {
    let vec = vec![Wrapper { content: 123 }];
    add_tag!(&vec[0], SecretTaint);
    // Becomes unreachable.
    verify!(has_tag!(&vec[0], SecretTaint));
}

/// Works well.
fn test_struct_1() {
    let wrapper = Wrapper { content: 123 };
    add_tag!(&wrapper.content, SecretTaint);
    verify!(has_tag!(&wrapper, SecretTaint));
}

/// Works well.
fn test_struct_2() {
    let wrapper = Wrapper { content: 123 };
    add_tag!(&wrapper, SecretTaint);
    verify!(has_tag!(&wrapper.content, SecretTaint));
}

/// Works well.
fn test_struct_3() {
    let mut wrapper = Wrapper { content: 123 };
    add_tag!(&wrapper, SecretTaint);
    wrapper.content += 1;
    verify!(has_tag!(&wrapper.content, SecretTaint));
}

/// Works well.
fn test_struct_4() {
    let mut wrapper = Wrapper { content: 123 };
    wrapper.content += 1;
    // warning: provably false verification condition
    verify!(has_tag!(&wrapper, SecretTaint));
}

/// Works well.
fn test_struct_5() {
    let mut wrapper = Wrapper { content: 123 };
    let tainted = 123;
    add_tag!(&tainted, SecretTaint);
    wrapper.content += 1 + tainted;
    verify!(has_tag!(&wrapper, SecretTaint));
    // warning: provably false verification condition.
    verify!(does_not_have_tag!(&wrapper, SecretTaint));
}

/// Works well.
fn test_vecwrapper_1() {
    let vec_wrapper = VecWrapper { inner: Vec::new() };
    add_tag!(&vec_wrapper, SecretTaint);
    verify!(has_tag!(&vec_wrapper.inner, SecretTaint));
}

fn test_vecwrapper_2() {
    let vec_wrapper = VecWrapper {
        inner: vec![1, 2, 3],
    };
    add_tag!(&vec_wrapper, SecretTaint);
    verify!(has_tag!(&vec_wrapper.inner[0], SecretTaint));
}

fn main() {
    // test_vec_1();
    // test_vec_2();
    // test_vec_3();
    // test_vec_5(Wrapper { content: 123 });
    // test_vec_6();
    test_vecwrapper_2();
}
