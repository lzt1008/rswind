use std::{sync::mpsc, time::Duration};

use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;
use rayon::prelude::*;
use rswind::{
    generator::Generator,
    glob::GlobFilter,
    io::{write_output, FileInput, OutputChannel},
    processor::ParGenerateWith,
};
use rswind_extractor::ParCollectExtracted;
use rustc_hash::FxHashSet;
use tracing::debug;

pub trait WatchApp {
    fn watch(&mut self, output: &OutputChannel);
}

impl WatchApp for Generator {
    fn watch(&mut self, output: &OutputChannel) {
        let (tx, rx) = mpsc::channel();

        let mut debouncer = new_debouncer(Duration::from_millis(10), None, tx).unwrap();

        debouncer.watcher().watch(self.glob.base(), RecursiveMode::Recursive).unwrap();

        let res = self.generate_contents();
        write_output(&res.css, output);

        for change in rx {
            let Ok(changes) = change else {
                continue;
            };

            let changes = changes
                .into_iter()
                .filter_map(|e| match e.kind {
                    EventKind::Create(_) | EventKind::Modify(_) => Some(e.event.paths),
                    _ => None,
                })
                .flatten()
                .glob_filter(&self.glob)
                .collect::<FxHashSet<_>>();

            if changes.is_empty() {
                continue;
            }

            debug!("Changes: {:?}", changes);

            let res = changes
                .into_par_iter()
                .map(FileInput::from_file)
                .collect::<Vec<_>>()
                .par_iter()
                .collect_extracted()
                .par_generate_with(&mut self.processor);

            write_output(&res.css, output);
        }
    }
}
