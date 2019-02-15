use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use crate::context::Context;
use crate::rls_data::{Analysis, Def, GlobalCrateId, Id};

#[derive(Debug, Clone)]
pub struct KrateDb {
    krates: HashMap<GlobalCrateId, Krate>,
}

impl KrateDb {
    pub fn new() -> Self {
        KrateDb {
            krates: HashMap::new(),
        }
    }

    pub fn ingest_krate(&mut self, analysis: Analysis) {
        let krate = Krate::new(analysis.prelude.crate_id.clone());

        self.krates.insert(krate.id.clone(), krate);
    }

    pub fn lookup_krate(&self, id: &GlobalCrateId) -> Option<Context<Krate, Self>> {
        self.krates.get(id).map(|k| Context::new(k, self))
    }
}

#[derive(Debug, Clone)]
pub struct Krate {
    id: GlobalCrateId,
    externals: HashMap<u32, (PathBuf, GlobalCrateId)>,
    defs: HashMap<u32, Arc<Def>>,
}

impl Krate {
    fn new(id: GlobalCrateId) -> Self {
        Krate {
            id,
            externals: HashMap::new(),
            defs: HashMap::new(),
        }
    }

    fn map_id(self: Context<Self, KrateDb>, id: &Id) -> Option<(GlobalCrateId, u32)> {
        if id.krate == 0 {
            Some((self.id.clone(), id.index))
        } else {
            self.externals
                .get(&id.krate)
                .map(|(_, gid)| (gid.clone(), id.index))
        }
    }

    pub fn lookup_def(self: Context<Self, KrateDb>, id: Id) -> Option<Arc<Def>> {
        unimplemented!()
    }
}
