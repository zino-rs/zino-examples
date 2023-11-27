use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::{Arc, LazyLock};

pub static ZINORANDDEQUE: LazyLock<Arc<Mutex<VecDeque<u32>>>> = LazyLock::new(|| {
    let mut temp: Vec<u32> = Vec::new();
    for i in 1..1000000 {
        temp.push(i);
    }
    let mut rng = rand::thread_rng();
    temp.shuffle(&mut rng);
    let code_deque = VecDeque::from(temp);
    Arc::new(Mutex::new(code_deque))
});

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
