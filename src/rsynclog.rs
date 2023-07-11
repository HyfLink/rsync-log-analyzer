use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

type DateTime = chrono::NaiveDateTime;

/// An iterator over lines of a log file.
///
/// # Examples
///
/// ```ignore
/// let file = RsyncFile::new("data/rsyncdata-7-4-1-simplified.log").unwrap();
///
/// for line in file {
///     println!("{line:?}");
/// }
/// ```
#[derive(Debug)]
pub struct RsyncLogFile {
    /// The log file.
    file: BufReader<File>,
    /// Reads lines to the `buffer` to avoid allocating space every iteration.
    buffer: Vec<u8>,
}

impl RsyncLogFile {
    /// Opens the rsync log data and creates a `RsyncLogFile`.
    ///
    /// # Params
    ///
    /// - `logpath` path to the log file.
    pub fn new<P: AsRef<Path>>(logpath: P) -> io::Result<Self> {
        let file = File::open(logpath)?;
        Ok(Self {
            file: BufReader::new(file),
            // Actually no line exceeds 200 bytes in `<workspace>/data/rsyncdata-7-4-1-simplified.log`.
            buffer: Vec::with_capacity(1000),
        })
    }
}

impl Iterator for RsyncLogFile {
    type Item = RsyncLogLine;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.buffer.clear();
            if matches!(self.file.read_until(b'\n', &mut self.buffer), Ok(bytes) if bytes > 0) {
                // Reads a line and parse to `RsyncLine`.
                match RsyncLogLine::new(&self.buffer) {
                    Some(line) => return Some(line),
                    // If the `line` is ill-formated, continue parsing the next line.
                    // intends to skip lines like '2023/07/04 03:40:27 [14417] building file list'.
                    None => continue,
                }
            } else {
                // Stops if the file is exausted.
                return None;
            }
        }
    }
}

/// A single line of the log file.
///
/// # Examples
///
/// ```ignore
/// let line = "2023/07/04 03:50:55 [14417] >f+++++++++ projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/signals2/detail/auto_buffer.hpp";
/// let line = RsyncLine::new(line).unwrap();
/// let date = DateTime::parse_from_str("2023/07/04 03:50:55", "%Y/%m/%d %H:%M:%S").unwrap();
/// assert_eq!(line.date(), date);
/// let path = PathBuf::from("projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/signals2/detail/auto_buffer.hpp");
/// assert_eq!(line.path(), path);
/// ```
#[derive(Clone, Debug)]
pub struct RsyncLogLine {
    // '2023/07/04 03:50:55 [14417] cd+++++++++ projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/serialization/'
    //  ^~~~~~~~~~~~~~~~~~~                     ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //  time                                    path
    pub time: DateTime,
    pub path: Rc<[u8]>,
}

impl RsyncLogLine {
    /// Parses the line and returns the [`RsyncLine`].
    ///
    /// Returns [`None`] if the `line` is ill-formated.
    pub fn new(line: &[u8]) -> Option<Self> {
        // '2023/07/04 03:50:55 [14417] cd+++++++++ projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/serialization/\n'
        //  ^~~~~~~~~~~~~~~~~~~ |     |^~~~~~~~~~~~|^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        //  time                |     |kind        | path
        //                      ptr1  ptr2         ptr3
        const DATETIME_FORMAT: &str = "%Y/%m/%d %H:%M:%S";
        const DIRECTORY_SPEC: &[u8] = b" cd+++++++++";
        const FILEPATH_SPEC: &[u8] = b" >f+++++++++";

        // splits the `line` into parts (time, kind, path).
        let (time, ptr1) = line.split_at(line.iter().position(|x| b'[' == *x)?);
        let (_num, ptr2) = ptr1.split_at(ptr1.iter().position(|x| b']' == *x)? + 1);
        let (kind, ptr3) = ptr2.split_at(FILEPATH_SPEC.len());
        let path = ptr3.get(1..ptr3.len() - 1)?;

        // parses `time`.
        let time = match std::str::from_utf8(time) {
            Ok(time) => match DateTime::parse_from_str(time.trim(), DATETIME_FORMAT) {
                Ok(time) => time,
                Err(_) => return None,
            },
            Err(_) => return None,
        };

        matches!(kind, FILEPATH_SPEC | DIRECTORY_SPEC).then(|| Self {
            time,
            // SAFETY: transmute `&[u8]` to `&OsStr`.
            path: Rc::from(path),
        })
    }

    /// Determines whether the path of `self` is the root of `other`'s.
    ///
    /// For example, `"projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/"` is root of
    /// - `"projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/signals2/detail/"`
    /// - `"projects/临时/xvd0605/20191/tps/boost_1_64_0/boost/signals2/detail/auto_buffer.hpp"`
    pub fn is_root_of(&self, other: &Self) -> bool {
        other.path.starts_with(&self.path)
    }
}
