use std::collections::HashMap;

use crate::{protocol::Uid, Request};
pub struct Identifiers {
    id_map: HashMap<i32, Vec<Uid>>,
}

impl Identifiers {

    pub fn new(request: &Request) -> Self {
        let eids = &request.context.user.eids;
        let mut id_map = HashMap::<i32, Vec<Uid>>::new();

        let mut default_caid_ver = None;
        let mut default_caid_vendor = None;

        for eid in eids {
            let uids = &eid.uids;
            for uid in uids {
                if uid.atype == 601 {
                    default_caid_ver = Some(uid.id.clone());
                    continue;
                }
                if uid.atype == 602 {
                    default_caid_vendor = Some(uid.id.clone());
                    continue;
                }

                if !id_map.contains_key(&uid.atype) {
                    id_map.insert(uid.atype, Vec::<Uid>::new());
                }
                id_map.get_mut(&uid.atype).unwrap().push(uid.clone());
            }
        }

        match id_map.get_mut(&513) {
            Some(caids) => {
                for caid in caids {
                    if caid.ver.is_none() {
                        caid.ver = default_caid_ver.clone();
                    }
                    if caid.vendor.is_none() {
                        caid.vendor = default_caid_vendor.clone();
                    }
                }
            }
            None => {}
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

}
