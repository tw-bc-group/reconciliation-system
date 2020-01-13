use std::{
    collections::{HashMap, HashSet},
    io::{BufReader, Read},
    marker::PhantomData,
    path::Path,
};

use crate::{loader::Loader, plugin::prelude::*};
use rayon::prelude::*;

pub struct Core<R: Read, L: Loader<R>> {
    _phantom: PhantomData<R>,
    loader: L,
    plugins: Vec<Box<dyn Flush>>,
    groups: HashMap<&'static str, Vec<HashSet<FlushData>>>,
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
            .flat_map(|v| v)
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
                    acc.insert(plugin.group(), Vec::with_capacity(2));
                    acc
                },
            ),
            plugins,
        }
    }

    pub fn run(&mut self, start: i64, end: i64) -> Result<HashMap<&'static str, Vec<FlushData>>> {
        let mut res = HashMap::new();
        for plugin in &self.plugins {
            let set = self
                .load_raw_data(plugin.name(), start, end)
                .map(|data| Self::flush_data(data, plugin.as_ref()))?;

            self.groups
                .entry(plugin.group())
                .or_insert_with(Vec::new)
                .push(set);
        }

        //TODO: remove to_owned
        for (group_name, group) in &self.groups {
            assert_eq!(group.len(), 2);
            let diff = group[0]
                .symmetric_difference(&group[1])
                .map(ToOwned::to_owned)
                .collect::<Vec<FlushData>>();
            res.insert(*group_name, diff);
        }
        self.groups.clear();
        Ok(res)
    }
}
