use crate::convert::entry_location::Key;
use crate::io::write_region::{RegionWriteError, WriteRegion};
use crate::EntryLocation3d;
use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

pub struct CachingRegionWriter<K: Key<R>, R: Eq + Hash> {
    sector_size: usize,
    max_cache_size: usize,

    path: PathBuf,
    region_cache: HashMap<R, WriteRegion<K, R>>,
    _phantom: PhantomData<R>,
}

impl<K: Key<R> + Copy, R: Eq + Hash> CachingRegionWriter<K, R> {
    pub fn new(path: &Path, sector_size: usize, max_cache_size: usize) -> Result<Self, std::io::Error> {
        fs::create_dir(path)?;
        Ok(Self {
            sector_size,
            max_cache_size,
            path: path.to_path_buf(),
            region_cache: HashMap::new(),
            _phantom: PhantomData::default(),
        })
    }

    pub fn write(&mut self, entry_location: K, data: &[u8]) -> Result<(), RegionWriteError> {
        if self.region_cache.len() > self.max_cache_size {
            self.flush()?
        }

        let region = self.region_cache.entry(entry_location.to_region_pos()).or_insert_with(|| {
            WriteRegion::new(
                &self.path.join(entry_location.region_key()),
                self.sector_size,
                EntryLocation3d::ENTRIES_PER_REGION,
            )
        });

        region.write(entry_location, data)?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), RegionWriteError> {
        for (_, mut region) in self.region_cache.drain() {
            region.flush()?;
        }
        Ok(())
    }
}
