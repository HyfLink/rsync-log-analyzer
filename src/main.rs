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

fn main() -> std::io::Result<()> {
    let analyzer = analyzer::RsyncLogAnalyzer::new(INPUT)?;
    let mut out = std::fs::File::create(OUTPUT)?;
    let mut freq = vec![HashMap::with_capacity(1000); 5];

    for (line, duration) in analyzer {
        use std::io::Write;

        match line.kind {
            RsyncLogKind::CD => *freq[0].entry(duration.num_seconds()).or_insert(0) += 1_u32,
            RsyncLogKind::FP => *freq[1].entry(duration.num_seconds()).or_insert(0) += 1_u32,
            RsyncLogKind::CL => *freq[2].entry(duration.num_seconds()).or_insert(0) += 1_u32,
            RsyncLogKind::CS => *freq[3].entry(duration.num_seconds()).or_insert(0) += 1_u32,
            RsyncLogKind::DD => *freq[4].entry(duration.num_seconds()).or_insert(0) += 1_u32,
        }

        out.write_all(&line.path)?;
        out.write_fmt(format_args!(": {}\n", duration.num_seconds()))?;
    }

    println!("cd => {:?}", freq[0]);
    println!(">f => {:?}", freq[1]);
    println!("cL => {:?}", freq[2]);
    println!("cS => {:?}", freq[3]);
    println!("cD => {:?}", freq[4]);
    Ok(())
}
