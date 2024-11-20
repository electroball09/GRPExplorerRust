use std::cmp::min;
use std::collections::HashSet;

use queues::{IsQueue, Queue};

use super::*;

pub struct BigfileLoad {
    initial_key: u32,
    loaded: HashSet<u32>,
    to_load: Vec<u32>,
}

impl BigfileLoad {
    pub fn new(initial_key: u32) -> Self {
        Self {
            initial_key,
            loaded: HashSet::new(),
            to_load: vec![initial_key],
        }
    }

    pub fn get_initial_key(&self) -> u32 {
        self.initial_key
    }

    pub fn get_load_status(&self) -> String {
        format!("{} / {}", self.loaded.len(), self.loaded.len() + self.to_load.len())
    }

    pub fn load_num(&mut self, bf: &mut Bigfile, num: u32) -> bool {
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

                match bf.load_file(key) {
                    Ok(_) => {
                        for subkey in &bf.object_table[&key].references {
                            let _ = tmp_to_load.add(*subkey);
                        }
                    },
                    Err(error) => {
                        error!("error loading key {:#010X}: {}", key, error);
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
            if bf.is_key_valid_to_load(key) {
                self.to_load.push(key);
            }
        }

        return false;
    }

    pub fn unload_all(&mut self, bf: &mut Bigfile) {
        for key in self.loaded.iter() {
            let load = bf.is_key_valid_to_load(*key);
            if load {
                if let Err(error) = bf.unload_file(*key) {
                    error!("{}", error);
                }
            }
        }
    }
}