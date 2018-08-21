use std::io::{self, StdoutLock, Write};

use config::{Config};
use traces::{CoverageStat, TraceMap};


/// Reports coverage information to stdout. This acquires a lock on stdout to
/// ensure nothing else can write to the stream before coverage information
/// has been printed.
///
pub fn report(config: &Config, traces: &TraceMap) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if !traces.is_empty() {
        handle.write(b"Coverage Results\n\n")?;

        if config.verbose {
            report_verbose(&mut handle, config, traces)?;
        }

        handle.write(b"Tested / Total Lines:\n")?;

        for file in traces.files() {
            let path = config.strip_project_path(file);
            write!(&mut handle, "{}: {}/{}\n",
                path.display(),
                traces.covered_in_path(&file),
                traces.coverable_in_path(&file)
            )?;
        }

        let cov_percent = traces.coverage_percentage() * 100f64;

        // TODO: Put File Filtering Here
        //
        write!(&mut handle, "{:.2}% coverage, {}/{} lines covered\n",
            cov_percent,
            traces.total_covered(),
            traces.total_coverable()
        )?;
    }
    else {
        handle.write(b"No Coverage Results Collected\n")?;
    }

    handle.flush()
}


fn report_verbose(handle: &mut StdoutLock, config: &Config, traces: &TraceMap) -> io::Result<()> {
    handle.write(b"Uncovered Lines:\n")?;

    for (ref key, ref value) in traces.iter() {
        let path = config.strip_project_path(key);
        let mut uncovered = vec![];

        for v in value.iter() {
            match v.stats {
                CoverageStat::Line(x) if x == 0 =>
                    uncovered.push(v.line),

                _                               =>
                    (),
            }
        }

        uncovered.sort();

        let (groups, last_group) = uncovered.into_iter()
            .fold((vec![], vec![]), accumulate_lines);

        let (groups, _) = accumulate_lines((groups, last_group), u64::max_value());

        if !groups.is_empty() {
            write!(handle, "{}: {}\n", path.display(), groups.join(", "))?;
        }
    }

    handle.write(b"\n")?;
    Ok(())
}


fn accumulate_lines((mut acc, mut group): (Vec<String>, Vec<u64>), next: u64) -> (Vec<String>, Vec<u64>) {
    if let Some(last) = group.last().cloned() {
        if next == last + 1 {
            group.push(next);
            (acc, group)
        }
        else {
            match (group.first(), group.last()) {
                (Some(first), Some(last)) if first == last => {
                    acc.push(format!("{}", first));
                },
                (Some(first), Some(last)) => {
                    acc.push(format!("{}-{}", first, last));
                },
                (Some(_), None) |
                (None, _) => (),
            };

            (acc, vec![next])
        }
    }
    else {
        group.push(next);
        (acc, group)
    }
}

