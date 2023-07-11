//! 
//! 
//! 

pub(crate) mod analyzer;
pub(crate) mod rsynclog;

// const INPUT: &str = "data/local/rsyncdata-7-4-1.log"
const INPUT: &str = "data/rsyncdata-7-4-1-simplified.log";
const OUTPUT: &str = "out/output.txt";

fn main() -> std::io::Result<()> {
    let analyzer = analyzer::RsyncLogAnalyzer::new(INPUT)?;
    let mut output = std::fs::File::create(OUTPUT)?;

    for (line, duration) in analyzer {
        use std::io::Write;
        output.write_all(&line.path)?;
        output.write_fmt(format_args!(": {}\n", duration.num_seconds()))?;
    }

    Ok(())
}
