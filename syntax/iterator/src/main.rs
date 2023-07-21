fn main() {
    create_iterator();
    iterator_demonstration();
    iterator_sum();
    iterator_adaptor();
    select();
    count_iterator();
    count_iterator_chaininng();
}

// [case=0] 创建迭代器
fn create_iterator() {
    let v = vec![1, 2, 3];
    // it: Iter<i32>
    // 在Rust中迭代器是lazy的，当调用iter方法时并不会进行遍历 
    let it = v.iter();

    // 循环遍历
    for value in it { // value: &i32
        println!("Got: {}", value);
    }
}

// [case-1] 通过next方法进行遍历
// 注意：
//      <1> iter方法迭代的元素获取的是不可变引用
//      <2> iter_mut方法迭代元素获取的是可变引用
//      <2> into_iter方法迭代元素获取元素的所有权
fn iterator_demonstration() {
    let v = vec![1, 2, 3];
    let mut it = v.iter();

    // it_mut: IterMut<i32>
    // let it_mut = v.iter_mut();

    //into_iter: IntoIter<i32>
    // let into_iter = v.into_iter();

    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), None);
}

// [case-2] 迭代器缺省方法-消费者方法
fn iterator_sum() {
    let v = vec![1, 2, 3];
    let it = v.iter();
    let total: i32 = it.sum();
    assert_eq!(total, 6);
}

// [case-3] 迭代器缺省方法-适配器方法
fn iterator_adaptor() {
    let v: Vec<i32> = vec![1, 2, 3];
    let it = v.iter();
    let v2: Vec<i32> = it.map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4])
}

// [case-4] 使用捕获其环境的闭包 + 迭代器
// 定义一个鞋子的结构体
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// 定义策略函数
// 在鞋子集合中寻找指定鞋码的鞋子并返回成一个集合
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn select() {
    let shoes = vec![
        Shoe {size: 10, style: String::from("sneaker")},
        Shoe {size: 13, style: String::from("sandal")},
        Shoe {size: 10, style: String::from("boot")}];

    let in_my_size = shoes_in_size(shoes, 10);

    println!("in_my_size: {:?}", in_my_size);
}

// [case-5] 如何实现迭代器
// 定义一个计数器结构体
struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Counter实现迭代器特征
impl Iterator for Counter {
    // 确定一个关联类型
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn count_iterator() {
   let counter = Counter::new();
   for v in counter.into_iter() { // v: u32
       println!("[counter] value = {}", v);
   }
}

fn count_iterator_chaininng() {
    let c1 = Counter::new();
    let c2 = Counter::new().skip(1);
    let sum: u32 = c1
        .zip(c2)                // impl Iterator<Item = (u32, u32)>
        // (a: u32, b: u32)
        .map(|(a, b)| a * b) // impl Iterator<Item = u32>
        // x: &u32
        .filter(|x| *x % 3 == 0)  // impl Iterator<Item = u32>
        .sum();
    println!("sum = {}", sum) // 18
}