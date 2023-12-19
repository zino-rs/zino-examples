use crate::extension::util;
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::{Arc, LazyLock};
use zino::{prelude::*, Cluster};
use zino_core::extension::TomlValueExt;

pub static ZINORANDOMDEQUE: LazyLock<Arc<Mutex<RandomDeque>>> =
    LazyLock::new(|| Arc::new(Mutex::new(RandomDeque::new())));

pub struct RandomDeque {
    deque: VecDeque<u32>,
    base_plus: u32,
    init: bool,
}

impl RandomDeque {
    pub fn new() -> Self {
        let mut temp: Vec<u32> = Vec::new();
        for i in 1..1000000 {
            temp.push(i);
        }
        let mut rng = rand::thread_rng();
        temp.shuffle(&mut rng);
        let code_deque = VecDeque::from(temp);
        let application_config = Cluster::config();
        let base_plus = application_config["uploads"]["short_code_base"]
            .as_u32()
            .unwrap_or(1000000);
        Self {
            deque: code_deque,
            base_plus,
            init: false,
        }
    }
    pub fn is_init(&mut self) -> bool {
        self.init
    }
    pub fn init_once(&mut self, remove_vec: Vec<u32>) {
        if !self.init {
            println!("initial the random deque... delete {:?}",remove_vec);
            self.deque.retain(|&x| {
                let temp = x + self.base_plus;
                !remove_vec.contains(&temp)
            });
            self.init = !self.init;
        }
    }
    pub fn get_random_num(&mut self) -> String {
        let random_num = self.deque.pop_front().unwrap() + self.base_plus;
        util::gen_code(random_num)
    }
    pub fn push_random_num(&mut self, num_str: &str) {
        self.deque
            .push_back(util::de_code(num_str) - self.base_plus);
    }
}

#[cfg(test)]
mod tests {
    use super::ZINORANDDEQUE;
    use std::{
        env::Args,
        rc::Rc,
        sync::{Arc, LazyLock},
        vec,
    };
    #[test]
    fn test_gen_code() {
        let m = Arc::clone(&ZINORANDDEQUE);
        let mut kk = m.lock().unwrap();
        let temp_num = kk.pop_front().unwrap();
        println!("{:?}", temp_num);
    }
}
