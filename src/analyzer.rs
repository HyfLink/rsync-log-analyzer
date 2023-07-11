use std::io;
use std::path::Path;

use chrono::Duration;

use crate::rsynclog::{RsyncLogFile, RsyncLogLine};

/// An iterator over pairs of [`RsyncLogLine`] and [`Duration`].
///
/// # Examples
///
/// ```ignore
/// let analyzer = RsyncLogAnalyzer::new("data/rsyncdata-7-4-1-simplified.log").unwrap();
///
/// for (line, duration) in analyzer {
///     // ...
/// }
/// ```
#[derive(Debug)]
pub struct RsyncLogAnalyzer {
    file: RsyncLogFile,
    stack: Vec<RsyncLogLine>,
    /// buffers results.
    buffer: Vec<(RsyncLogLine, Duration)>,
}

impl RsyncLogAnalyzer {
    /// Opens the rsync log data and creates a `RsyncLogAnalyzer`.
    ///
    ///  # Params
    ///
    /// - `logpath` path to the log file.
    pub fn new<P: AsRef<Path>>(logpath: P) -> io::Result<Self> {
        Ok(Self {
            file: RsyncLogFile::new(logpath)?,
            // the capacity of `stack` is proportional to the path depth.
            stack: Vec::with_capacity(100),
            // the capacity of `buffer` is supposed to be the same as `stack`.
            buffer: Vec::with_capacity(100),
        })
    }

    fn refill(&mut self) {
        // refills `buffer` only if its empty.
        while self.buffer.is_empty() {
            // takes `this_line` for the newly read line from `file`.
            //   and `last_line` for the line on the top of `stack`.
            if let Some(this_line) = self.file.next() {
                // - if `file` is NOT empty:
                //
                //   if the `last_line` is not the root of `this_line`.
                //   pops `stack` and moves them to `buffer`.
                while matches!(self.stack.last(), Some(last) if !last.is_root_of(&this_line)) {
                    // SAFETY: the loop condition guarantees that `stack` is not empty.
                    let last_line = self.stack.pop().unwrap();
                    let duration = this_line.time - last_line.time;
                    self.buffer.push((last_line, duration));
                }
                //   now `stack` is empty or `last_line` is the root of `this_line`.
                //   pushes `this_line` to `stack`.
                self.stack.push(this_line);
            } else if let Some(line) = self.stack.last() {
                // - if `file` is empty and `stack` is NOT empty:
                //   moves lines in the `stack` to `buffer`.
                let this_time = line.time;
                while let Some(last_line) = self.stack.pop() {
                    let duration = this_time - last_line.time;
                    self.buffer.push((last_line, duration));
                }
            } else {
                // - if `file` is empty and `stack` is empty:
                //   the iterator is exausted.
                return;
            }
        }
    }
}

impl Iterator for RsyncLogAnalyzer {
    type Item = (RsyncLogLine, Duration);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.buffer.is_empty() {
            self.refill();
        }
        self.buffer.pop()
    }
}
