use std::{
    collections::{HashMap, HashSet},
    io::{BufReader, Read},
    marker::PhantomData,
    path::Path,
    sync::Arc,
};

use crate::{loader::Loader, plugin::prelude::*};
use rayon::prelude::*;

pub struct Core<R: Read, L: Loader<R>> {
    _phantom: PhantomData<R>,
    loader: L,
    plugins: Vec<Box<dyn Flush>>,
    groups: HashMap<&'static str, Vec<Arc<HashSet<FlushData>>>>,
}

impl<R, L> Core<R, L>
where
    R: Read,
    L: Loader<R>,
{
    //TODO: load json as stream ?
    fn load_raw_data(&self, name: &str, start: i64, end: i64) -> Result<Vec<Value>> {
        self.loader
            .get(name, start, end)
            .map(BufReader::new)
            .and_then(|buf_reader| serde_json::from_reader(buf_reader).map_err(Into::into))
    }

    fn flush_data<D>(d: D, f: &dyn Flush) -> HashSet<FlushData>
    where
        D: IntoParallelIterator<Item = Value>,
    {
        d.into_par_iter()
            .flat_map(|v| f.flush(v))
            .flat_map(|array| array)
            .map(|mut flush_data| {
                flush_data.name = f.name().to_string();
                flush_data
            })
            .collect::<HashSet<FlushData>>()
    }

    pub fn new<P: AsRef<Path>>(loader: L, plugin_path: P) -> Self {
        let plugins = load_plugins(plugin_path).expect("failed to load plugins.");

        Core {
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
        }
    }

    pub fn run(
        &mut self,
        start: i64,
        end: i64,
    ) -> Result<HashMap<&'static str, Vec<StatementResult>>> {
        let mut res = HashMap::new();
        for plugin in &self.plugins {
            let set = self
                .load_raw_data(plugin.name(), start, end)
                .map(|data| Self::flush_data(data, plugin.as_ref()))
                .map(Arc::new)?;

            for group in plugin.groups() {
                self.groups
                    .entry(group)
                    .or_insert_with(Vec::new)
                    .push(set.clone());
            }
        }

        //TODO: remove to_owned
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
