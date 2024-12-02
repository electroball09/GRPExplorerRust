use std::cmp::min;
use std::collections::HashSet;

use queues::{IsQueue, Queue};
use strum::IntoEnumIterator;

use super::*;

pub struct AmortizedLoad {
    loaded: HashSet<u32>,
    type_map: HashMap<ObjectType, Vec<u32>>,
    to_load: Vec<u32>,
}

pub trait LoadSet {
    fn loaded_by_type(&self, obj_type: crate::bigfile::ObjectType) -> Option<&Vec<u32>>;
    fn is_loaded(&self) -> bool;
}

impl LoadSet for AmortizedLoad {
    fn loaded_by_type(&self, obj_type: crate::bigfile::ObjectType) -> Option<&Vec<u32>> {
        self.type_map.get(&obj_type)
    }

    fn is_loaded(&self) -> bool {
        self.to_load.len() == 0
    }
}

impl Default for AmortizedLoad {
    fn default() -> Self {
        Self::new(0xFFFFFFFF)
    }
}

impl AmortizedLoad {
    pub fn new(initial_key: u32) -> Self {
        Self {
            loaded: HashSet::new(),
            to_load: vec![initial_key],
            type_map: ObjectType::iter().map(|t| (t, Vec::new())).collect(),
        }
    }

    pub fn get_load_status(&self) -> String {
        format!("{} / {}", self.loaded.len(), self.loaded.len() + self.to_load.len())
    }

    pub fn load_num(&mut self, bf: &mut Bigfile, num: u32) -> bool {
        if self.is_loaded() {
            warn!("attemtping to load a completely loaded load set!");
            return true;
        }

        let mut tmp_to_load: Queue<u32> = Queue::new();

        for key in self.to_load.drain(0..min(num as usize, self.to_load.len())) {
            let _ = tmp_to_load.add(key);
        }

        let mut counter = 0;
        while counter < num {            
            if let Ok(key) = tmp_to_load.remove() {
                if self.loaded.contains(&key) {
                    continue;
                }
                
                if bf.is_key_valid(key) {
                    match bf.load_file(key) {
                        Ok(_) => {
                            for subkey in &bf.object_table[&key].references {
                                let _ = tmp_to_load.add(*subkey);
                            };
                            self.type_map.get_mut(&bf.file_table.get(&key).unwrap().object_type).unwrap().push(key);
                        },
                        Err(error) => {
                            error!("error loading key {:#010X}: {}", key, error);
                        }
                    }
                }

                self.loaded.insert(key);

            } else {
                if self.to_load.len() == 0 {
                    return true;
                }
            }

            counter += 1;
        };

        while let Ok(key) = tmp_to_load.remove() {
            if !self.loaded.contains(&key) && bf.is_key_valid(key) {
                self.to_load.push(key);
            }
        }

        return false;
    }

    pub fn unload_all(&mut self, bf: &mut Bigfile) {
        for key in self.loaded.iter() {
            let load = bf.is_key_valid(*key);
            if load {
                if let Err(error) = bf.unload_file(*key) {
                    error!("{}", error);
                }
            }
        }
    }
}