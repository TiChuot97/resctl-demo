// Copyright (c) Facebook, Inc. and its affiliates.
use super::bench::{iocost_on_off, IOCOST_QOS_PATH};
use super::{prepare_bin_file, Config};
use anyhow::{bail, Result};
use std::process::Command;
use util::*;

const MISC_BINS: [(&str, &[u8]); 4] = [
    (
        "iocost_coef_gen.py",
        include_bytes!("misc/iocost_coef_gen.py"),
    ),
    ("sideloader.py", include_bytes!("misc/sideloader.py")),
    ("io_latencies.py", include_bytes!("misc/io_latencies.py")),
    (
        "io_latencies_wrapper.sh",
        include_bytes!("misc/io_latencies_wrapper.sh"),
    ),
];

pub fn prepare_misc_bins(cfg: &Config) -> Result<()> {
    for (name, body) in &MISC_BINS {
        prepare_bin_file(&format!("{}/{}", &cfg.misc_bin_path, name), body)?;
    }

    if cfg.io_latencies_bin.is_some() {
        run_command(
            Command::new(cfg.io_latencies_bin.as_ref().unwrap())
                .arg(format!("{}:{}", cfg.scr_devnr.0, cfg.scr_devnr.1))
                .args(&["-i", "0"]),
            "is bcc working? https://github.com/iovisor/bcc",
        )?;
    }

    if let Err(e) = iocost_on_off(true, cfg) {
        bail!(
            "failed to enable iocost by writing to {:?} ({:?})",
            IOCOST_QOS_PATH,
            &e
        );
    }

    Ok(())
}
