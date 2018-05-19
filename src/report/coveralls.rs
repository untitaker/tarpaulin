use std::collections::HashMap;

use coveralls_api::*;

use config::Config;
use report::types::{self, Error, Report};
use traces::{TraceMap, CoverageStat};


fn mk_report(config: &Config) -> types::Result<CoverallsReport> {
    match config.coveralls {
        Some(ref key)   => {
            let id = match config.ci_tool {
                Some(ref service)   => Identity::ServiceToken(Service {
                    service_name:   service.clone(),
                    service_job_id: key.clone()
                }),

                None                => Identity::RepoToken(key.clone())
            };

            Ok(CoverallsReport::new(id))
        },

        None            => {
            Err(Error::CoverallsAuth)
        }
    }
}


impl Report for CoverallsReport {

    fn render(config: &Config, traces: &TraceMap) -> types::Result<Self> {
        let mut report = mk_report(config)?;

        for file in traces.files() {
            let rel_path  = config.strip_project_path(file);
            let file_cov  = traces.get_child_traces(file);
            let mut lines = HashMap::new();

            for cov in file_cov {
                match cov.stats {
                    CoverageStat::Line(hits)    => {
                        lines.insert(cov.line as usize, hits as usize);
                    },
                    _                           => {
                        // TODO: This should be more useful / descriptive
                        println!("Unsupported coverage statistic")
                    }
                }

                if let Ok(source) = Source::new(&rel_path, file, &lines, &None, false) {
                    report.add_source(source);
                }
            }
        }

        Ok(report)
    }

    // TODO: This can fail, the trait should return a Result too
    fn export(&self, config: &Config) {
        let result = match config.report_uri {
            Some(ref uri)   => {
                println!("Sending report to {}", uri);
                self.send_to_endpoint(uri)
            },
            None            => {
                println!("Sending report to coveralls.io");
                self.send_to_coveralls()
            }
        };

        if config.verbose {
            if let Err(e) = result {
                println!("Failed to export: {}", e);
            }
        }
    }
}

