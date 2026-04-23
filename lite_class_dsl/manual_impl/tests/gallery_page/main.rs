#[macro_use]
extern crate manual_impl;

use manual_impl::{utils::BUF, widget::IWidget};
use oop_rs::rc::RcDefault;

use crate::{gallery_page::GalleryPage, root_widget::RootWidget};

mod gallery_page;
mod gallery_page_state;
mod my_element;
mod my_widget;
mod root_element;
mod root_widget;

#[test]
#[cfg_attr(miri, ignore = "known bug of `RcRef`")]
fn gallery_page() {
    BUF.take();
    let root = RootWidget::default().create_element();
    root.mount(None);
    GalleryPage::default().create_element().mount(Some(&*root));
    assert_eq!(BUF.take(), EXPECTED_OUTPUT);
}

#[cfg(not(miri))]
#[test]
fn run_dart() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    if Command::new("dart")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        println!("`dart` not found, skipping test");
        return;
    }
    let output = Command::new("dart")
        .stderr(Stdio::inherit())
        .arg("tests/gallery_page/gallery_page.dart")
        .output()
        .unwrap();
    if !output.status.success() {
        eprintln!("`dart gallery_page.dart` run failed");
        eprintln!("---- dart stdout ----");
        std::io::stderr().write_all(&output.stdout).unwrap();
        eprintln!("---- dart stderr ----");
        std::io::stderr().write_all(&output.stderr).unwrap();
        assert!(output.status.success());
    }
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.lines().collect::<Vec<_>>(), EXPECTED_OUTPUT);
}

const EXPECTED_OUTPUT: &[&str] = &[
    "Element",
    "RootElement",
    "RootElement::mount",
    "RootElementMixin::mount",
    "Element::mount",
    "RootElement::perform_rebuild",
    "Element::perform_rebuild",
    "GalleryPage::create_state",
    "Element",
    "ComponentElement",
    "StatefulElement",
    "StatefulElement::mount",
    "ComponentElement::mount",
    "Element::mount",
    "StatefulElement::first_build",
    "GalleryPageState::init_state",
    "State::init_state",
    "ComponentElement::first_build",
    "Element::rebuild",
    "StatefulElement::perform_rebuild",
    "ComponentElement::perform_rebuild",
    "GalleryPageState::build",
    "GalleryPage::on_create",
    "Element::perform_rebuild",
];
