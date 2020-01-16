use std::{
    collections::{HashMap, HashSet},
    io::{BufReader, Read},
    marker::PhantomData,
    path::Path,
    sync::Arc,
};

use crate::{loader::Loader, plugin::prelude::*};
use anyhow::Result;
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
        start: i64,
        end: i64,
    ) -> Result<HashMap<&'static str, Vec<StatementResult>>> {
        let mut res = HashMap::new();
        let mut groups = HashMap::new();

        for plugin in &self.plugins {
            let set = self
                .load_and_flush_data(plugin.name(), start, end, plugin.as_ref())
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
                .fold(HashMap::new(), |mut acc, data| {
                    acc.entry(data.id()).or_insert_with(Vec::new).push(data);
                    acc
                })
                .par_iter()
                .map(|(_, data)| match data.len() {
                    1 => StatementResult::OneSide(data[0].into()),
                    2 => {
                        let mismatches = data[0].compare(data[1]);
                        StatementResult::DataMismatch(
                            data.iter().map(|v| (*v).into()).collect(),
                            mismatches,
                        )
                    }
                    _ => unreachable!(),
                })
                .collect::<Vec<StatementResult>>();

            res.insert(*group_name, diff);
        }
        Ok(res)
    }
}
