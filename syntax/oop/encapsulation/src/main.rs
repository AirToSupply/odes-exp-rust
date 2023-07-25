// Rust面向对象-封装
// [case] 实现计算平均数集合体

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {

    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32>{
        let result = self.list.pop();
        match result {
            None => None,
            Some(value) => {
                self.update_average();
                Some(value)
            },
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }
}

fn main() {
    let mut ac = AveragedCollection::new();
    println!("[average]: {}", ac.average());

    ac.add(1);
    println!("[average]: {}", ac.average());

    ac.add(2);
    println!("[average]: {}", ac.average());

    ac.add(3);
    println!("[average]: {}", ac.average());

    ac.remove();
    println!("[average]: {}", ac.average());
}
