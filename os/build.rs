// generate link_app.s containing all the app's elf file

use std::fs::{read_dir, File};
use std::io::{Result, Write};

static SOURCE_PATH: &str = "../user/src/";
static TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release/";
static LINK_APP_FILE: &str = "./src/link_app.s";

fn main() {
    println!("cargo:rerun-if-changed={}", SOURCE_PATH);
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    build_app_data().unwrap();
}

fn build_app_data() -> Result<()> {
    let mut f = File::create(LINK_APP_FILE).unwrap();
    // get app's name without ext(.rs)
    let mut apps: Vec<_> = read_dir(SOURCE_PATH.to_string() + "bin/")
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let mut name_with_ext = dir_entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(
        f,
        r#"
    .align 3
    .section .data
    .global _num_app
_num_app:
    .quad {}"#,
        apps.len()
    )?;

    for i in 0..apps.len() {
        writeln!(f, r#"    .quad app_{}_start"#, i)?;
    }
    writeln!(f, r#"    .quad app_{}_end"#, apps.len() - 1)?;

    // add app names

    writeln!(
        f,
        r#"
    .global _app_names
_app_names:"#
    )?;
    for app in apps.iter() {
        writeln!(f, r#"    .string "{}""#, app)?;
    }

    // debug: use elf instead of bin to get memory layout

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            f,
            r#"
    .section .data
    .global app_{0}_start
    .global app_{0}_end
    .align 3
app_{0}_start:
    .incbin "{2}{1}"
app_{0}_end:"#,
            idx, app, TARGET_PATH
        )?;
    }
    Ok(())
}
