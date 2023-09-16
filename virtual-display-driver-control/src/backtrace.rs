use std::fmt::{self, Display};

use backtrace::{Backtrace, BacktraceFmt, BytesOrWideString, PrintFmt};

/// Force capture a short style backtrace
pub struct CaptureBacktrace;

impl Display for CaptureBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bt = Backtrace::new();

        let mut eidx = 0usize;
        let mut bidx = 0usize;
        let mut chop = 0usize;
        bt.frames().iter().enumerate().for_each(|(idx, frame)| {
            frame.symbols().iter().for_each(|symbol| {
                symbol.name().map(|n| {
                    n.as_str().map(|n| {
                        if n.contains("__rust_begin_short_backtrace") && bidx == 0 {
                            bidx = idx;
                        } else if n.contains("__rust_end_short_backtrace") {
                            eidx = idx + 1;
                        } else if n.starts_with("backtrace::") {
                            chop = idx + 1;
                        }
                    })
                });
            })
        });

        let start = if chop > eidx { chop } else { eidx };

        let frames = &bt.frames()[start..bidx];

        let cwd = std::env::current_dir();
        let mut print_path = move |fmt: &mut fmt::Formatter<'_>, path: BytesOrWideString<'_>| {
            let path = path.into_path_buf();

            if let Ok(cwd) = &cwd {
                if let Ok(suffix) = path.strip_prefix(cwd) {
                    return fmt::Display::fmt(&suffix.display(), fmt);
                }
            }

            fmt::Display::fmt(&path.display(), fmt)
        };

        let mut backtrace_fmt = BacktraceFmt::new(f, PrintFmt::Short, &mut print_path);

        for frame in frames.iter() {
            let mut single_frame = backtrace_fmt.frame();
            single_frame.backtrace_frame(frame)?;
        }

        backtrace_fmt.finish()?;

        Ok(())
    }
}
