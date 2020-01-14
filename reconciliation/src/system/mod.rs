use std::{
    collections::{HashMap, HashSet},
    io::{BufReader, Read},
    marker::PhantomData,
    path::Path,
    sync::Arc,
};

use crate::{loader::Loader, plugin::prelude::*};
use rayon::prelude::*;

pub struct System<R: Read, L: Loader<R>> {
    _phantom: PhantomData<R>,
    loader: L,
    plugins: Vec<Box<dyn Flush>>,
    groups: HashMap<&'static str, Vec<Arc<HashSet<FlushData>>>>,
}

impl<R, L> System<R, L>
where
    R: Read,
    L: Loader<R>,
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
                    flush_data.name = f.name().to_string();
                    flush_data
                })
                .collect::<HashSet<FlushData>>()
        })
    }

    pub fn init<P: AsRef<Path>>(loader: L, plugin_path: P) -> Result<Self> {
        load_plugins(plugin_path).map(|plugins| System {
            _phantom: PhantomData,
            loader,
            groups: plugins.iter().fold(
                HashMap::with_capacity(plugins.len()),
                |mut acc, plugin| {
                    for group in plugin.groups() {
                        acc.insert(group, Vec::with_capacity(2));
                    }
                    acc
                },
            ),
            plugins,
        })
    }

    pub fn run(
        &mut self,
        start: i64,
        end: i64,
    ) -> Result<HashMap<&'static str, Vec<StatementResult>>> {
        let mut res = HashMap::new();
        for plugin in &self.plugins {
            let set = self
                .load_and_flush_data(plugin.name(), start, end, plugin.as_ref())
                .map(Arc::new)?;

            for group in plugin.groups() {
                self.groups
                    .entry(group)
                    .or_insert_with(Vec::new)
                    .push(set.clone());
            }
        }

        for (group_name, group) in &self.groups {
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
        self.groups.clear();
        Ok(res)
    }
}
