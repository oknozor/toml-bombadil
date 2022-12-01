pub mod links {
    use crate::dots::LinkResult;
    use anyhow::Error;
    use colored::Colorize;
    use std::fmt::Formatter;
    use std::io::Write;
    use std::{fmt, io};

    impl fmt::Display for LinkResult {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                LinkResult::Updated { copy, target, .. } => {
                    writeln!(f, "\t{copy:?} => {target:?}")?
                }
                LinkResult::Created { copy, target, .. } => {
                    writeln!(f, "\t{copy:?} => {target:?}")?
                }
                LinkResult::Ignored { source } => writeln!(f, "\t{source:?}")?,
                LinkResult::Unchanged { target } => writeln!(f, "\t{target:?}")?,
            }

            Ok(())
        }
    }

    pub fn write(results: Vec<LinkResult>, out: &mut impl Write, title: &str) -> io::Result<()> {
        if !results.is_empty() {
            writeln!(out, "{}", format!("[{title}]").bold().yellow())?;
            for result in results {
                write!(out, "{}", result)?;
            }
            writeln!(out)?;
        }
        Ok(())
    }

    pub fn write_errors(errored: Vec<Error>, out: &mut impl Write) -> io::Result<()> {
        if !errored.is_empty() {
            writeln!(out, "{}", "[Errored]".bold().red())?;
            for error in errored {
                writeln!(out, "\t{error:?}")?;
            }
            writeln!(out)?;
        }
        Ok(())
    }

    pub fn write_deletion(deleted: Vec<String>, out: &mut impl Write) -> io::Result<()> {
        if !deleted.is_empty() {
            writeln!(out, "{}", "[Deleted]".bold().red())?;
            for deleted in deleted {
                writeln!(out, "\t{deleted:?}")?;
            }
            writeln!(out)?;
        }
        Ok(())
    }
}
