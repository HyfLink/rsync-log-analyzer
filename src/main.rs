//!
//!
//!

use std::collections::HashMap;

use crate::rsynclog::*;

pub(crate) mod analyzer;
pub(crate) mod rsynclog;

const INPUT: &str = "data/local/rsyncdata-7-4-1.log";
// const INPUT: &str = "data/rsyncdata-7-4-1-simplified.log";
const OUTPUT: &str = "out/output.txt";

#[allow(unused)]
fn frequency() -> std::io::Result<()> {
    let analyzer = analyzer::RsyncLogAnalyzer::new(INPUT)?;
    let mut dfrequency = HashMap::with_capacity(1000);
    let mut ffrequency = HashMap::with_capacity(1000);
    for (line, duration) in analyzer {
        match line.kind {
            RsyncLogKind::Directory => {
                *dfrequency.entry(duration.num_seconds()).or_insert(0) += 1_u32
            }
            RsyncLogKind::Filepath => {
                *ffrequency.entry(duration.num_seconds()).or_insert(0) += 1_u32
            }
        }
    }

    dbg!(dfrequency);
    dbg!(ffrequency);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let analyzer = analyzer::RsyncLogAnalyzer::new(INPUT)?;
    let mut output = std::fs::File::create(OUTPUT)?;

    let mut map_dir = HashMap::with_capacity(10000);
    let mut map_file = HashMap::with_capacity(10000);

    for (line, duration) in analyzer {
        use std::io::Write;
        output.write_all(&line.path)?;
        output.write_fmt(format_args!(": {}\n", duration.num_seconds()))?;

        match line.kind {
            RsyncLogKind::Directory => *map_dir.entry(duration.num_seconds()).or_insert(0) += 1_u32,
            RsyncLogKind::Filepath => *map_file.entry(duration.num_seconds()).or_insert(0) += 1_u32,
        }
    }

    println!("{map_dir:?}");
    println!("{map_file:?}");

    Ok(())
}
