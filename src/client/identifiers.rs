use std::collections::HashMap;

use crate::{protocol::Uid, Request};
pub struct Identifiers {
    id_map: HashMap<i32, Vec<Uid>>,
}

impl Identifiers {

    pub fn new(request: &Request) -> Self {
        let eids = &request.context.user.eids;
        let mut id_map = HashMap::<i32, Vec<Uid>>::new();

        for eid in eids {
            let uids = &eid.uids;
            for uid in uids {
                if !id_map.contains_key(&uid.atype) {
                    id_map.insert(uid.atype, Vec::<Uid>::new());
                }
                id_map.get_mut(&uid.atype).unwrap().push(uid.clone());
            }
        }

        for (_, value) in id_map.iter_mut() {
            value.sort_by(|a, b| b.ver.cmp(&a.ver));
        }

        Self {
            id_map,
        }
    }

    pub fn get_ids(&self, atype: i32) -> Option<&Vec<Uid>> {
        self.id_map.get(&atype)
    }

    pub fn get_id(&self, atype: i32, seq: i32) -> Option<&Uid> {
        if let Some(uids) = self.id_map.get(&atype) {
            if seq < uids.len() as i32 {
                return uids.get(seq as usize);
            }
        }
        None
    }

    pub fn get_flatten_ids(&self) -> Vec<&Uid> {
        self.id_map.values().flatten().collect()
    }

}
