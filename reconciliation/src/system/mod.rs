use std::{
    collections::{HashMap, HashSet},
    io::{BufReader, Read},
    marker::PhantomData,
    ops::Range,
    path::Path,
    sync::Arc,
};

use crate::{loader::Loader, plugin::prelude::*};
use anyhow::Result;
use chrono::{DateTime, FixedOffset, Utc};
use rayon::prelude::*;

pub struct System<R: Read, L: Loader<R> + Sync> {
    _phantom: PhantomData<R>,
    loader: L,
    plugins: Vec<Box<dyn Flush>>,
}

impl<R, L> System<R, L>
where
    R: Read,
    L: Loader<R> + Sync,
{
    fn load_and_flush_data(
        &self,
        name: &str,
        start: i64,
        end: i64,
        f: &dyn Flush,
    ) -> Result<HashSet<FlushData>> {
        self.loader.get(name, start, end).map(|reader| {
            serde_json::Deserializer::from_reader(BufReader::new(reader))
                .into_iter::<Value>()
                .flat_map(|res| -> Result<_> { Ok(res?) })
                .flat_map(|v| f.flush(v))
                .flatten()
                .map(|mut flush_data| {
                    flush_data.belongs = f.name().to_string();
                    flush_data
                })
                .collect::<HashSet<FlushData>>()
        })
    }

    pub fn init<P: AsRef<Path>>(loader: L, plugin_path: P) -> Result<Self> {
        load_plugins(plugin_path).map(|plugins| System {
            _phantom: PhantomData,
            loader,
            plugins,
        })
    }

    pub fn process(
        &self,
        with_buffer: Range<DateTime<Utc>>,
        without_buffer: Range<DateTime<Utc>>,
        offset: FixedOffset,
    ) -> Result<HashMap<&'static str, Vec<StatementResult>>> {
        let mut res = HashMap::new();
        let mut groups = HashMap::new();

        for plugin in &self.plugins {
            let set = self
                .load_and_flush_data(
                    plugin.name(),
                    with_buffer.start.timestamp_millis(),
                    with_buffer.end.timestamp_millis(),
                    plugin.as_ref(),
                )
                .map(Arc::new)?;

            for group in plugin.groups() {
                groups
                    .entry(group)
                    .or_insert_with(Vec::new)
                    .push(set.clone());
            }
        }

        for (group_name, group) in &groups {
            assert_eq!(group.len(), 2);
            let diff = group[0]
                .symmetric_difference(&group[1])
                .fold(HashMap::new(), |mut acc, flush_data| {
                    acc.entry(flush_data.id())
                        .or_insert_with(Vec::new)
                        .push(flush_data);
                    acc
                })
                .par_iter()
                .filter(|(_, flush_data)| flush_data.len() <= 2)
                .flat_map(|(_, flush_data)| match flush_data.len() {
                    1 => {
                        if without_buffer.contains(flush_data[0].datetime.as_ref()) {
                            Some(StatementResult::OneSide(flush_data[0].into()))
                        } else {
                            None
                        }
                    }
                    2 => {
                        let mismatches = flush_data[0].compare(flush_data[1], offset);
                        Some(StatementResult::DataMismatch(
                            flush_data.iter().map(|v| (*v).into()).collect(),
                            mismatches,
                        ))
                    }
                    _ => None,
                })
                .collect::<Vec<StatementResult>>();

            res.insert(*group_name, diff);
        }
        Ok(res)
    }
}
