use std::{thread, time::Duration};

fn main() {
    test_workout();
    test_workout_for_closure();
    test_workout_for_closure_memory_model();
    catch_var();
}

// --------------------------------------------------------------------------------
// [case-0] 前置案例
// 模拟一个耗时操作的工具函数
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("缓慢计算中...");
    // 睡眠2s
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 定义一个实际工作的函数，这个函数中会运用到上面定义的工具函数
fn workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("今天做{}个俯卧撑！", simulated_expensive_calculation(intensity));
        println!("今天做{}个仰卧起坐！", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("今天休息下，注意保持水分！");
        } else {
            println!("今天跑步{}分钟！", simulated_expensive_calculation(random_number));
        }
    }
}

// 定义一个测试函数
fn test_workout() {
    println!("[workout_test]");
    let simulated_intensity = 10;
    let simulated_random_number = 7;
    workout(simulated_intensity, simulated_random_number);
}

/*
  📒 上面这个例子有如下缺点：
  <1> 工具函数是类似策略的函数，在逻辑函数中需要定义多次，不够优雅
  <2> 当工具函数的代码很少，能否简化定义来缩短整个逻辑代码？
*/

// --------------------------------------------------------------------------------
// [case-1] 使用闭包优化[case-0]
fn workout_for_closure(intensity: u32, random_number: u32) {
    // 定义一个闭包来优化工具函数
    // expensive_closure: impl Fn(u32) -> u32
    // 这里的Fn为函数类型
    let expensive_closure = |num: u32| -> u32 {
        println!("缓慢计算中...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("今天做{}个俯卧撑！", expensive_closure(intensity));
        println!("今天做{}个仰卧起坐！", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("今天休息下，注意保持水分！");
        } else {
            println!("今天跑步{}分钟！", expensive_closure(random_number));
        }
    }
}

// 定义一个测试函数
fn test_workout_for_closure() {
    println!("[workout_for_closure]");
    let simulated_intensity = 10;
    let simulated_random_number = 7;
    workout_for_closure(simulated_intensity, simulated_random_number);
}

/*
  📒 上面这个例子有如下缺点：
  <1> workout_for_closure函数中闭包需要重复计算多次，能否优化？
*/

// --------------------------------------------------------------------------------
// [case-2] 通过`闭包记忆模式`来解决[case-1]中闭关重复计算的问题
// 注意：
//      所谓`闭包记忆模式`是通过定义一个结构体来保存闭包和闭包计算的结果

// 定义记忆模式结构体
// 注意：
//       <1> 结构体中的泛型T的界定必须是Fn函数类型，且函数的入参和出参类型都是u32
//       <2> [重要] 闭包特征的函数类型表现形式通常有如下3种：
//           （1）Fn
//           （2）FnMut
//           （3）FnOnce
struct Cacher<T> 
where
    T: Fn(u32) -> u32,
{
    // 保存闭包
    calculation: T,
    // 保存结果
    value: Option<u32>,
}

// 定义记忆模式结构体实现块
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32, 
{
    // 定义关联函数用于创建记忆模式结构体
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation, 
            value: None,
        }
    }

    // 定义缓存方法
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                // 通过闭包进行计算
                let v = (self.calculation)(arg);
                // 将计算结果缓存
                self.value = Some(v);
                v
            },
        }
    }
}

// 定义逻辑函数
fn workout_for_closure_memory_model(intensity: u32, random_number: u32) {
    // 定义一个闭包
    let expensive_closure = |num: u32| -> u32 {
        println!("缓慢计算中...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // 生成闭包记忆结构
    // cache: Cacher<impl Fn(u32) -> u32>
    let mut cache = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!("今天做{}个俯卧撑！", cache.value(intensity));
        println!("今天做{}个仰卧起坐！", cache.value(intensity));
    } else {
        if random_number == 3 {
            println!("今天休息下，注意保持水分！");
        } else {
            println!("今天跑步{}分钟！", cache.value(random_number));
        }
    }
}

// 定义测试函数
fn test_workout_for_closure_memory_model() {
    println!("[workout_for_closure_memory_model]");
    let simulated_intensity = 10;
    let simulated_random_number = 7;
    workout_for_closure_memory_model(simulated_intensity, simulated_random_number);
}

// --------------------------------------------------------------------------------
// [case-3] 通过闭包将作用域的变量捕获到闭包中
// 注意：
//      <1> 函数不能将作用域的变量捕获到函数中
//      <2> 函数不需要额外的存储，但闭包需要有额外的存储开销
fn catch_var() {
    let x = 4;
    // equal_to_x: impl Fn(i32) -> bool
    let equal_to_x = |z: i32| x == z;
    let y = 4;
    assert!(equal_to_x(y));
}

/*
  📒 
  闭包可以映射到函数输入参数的三种形式
  <1> 捕获所有权
  <2> 以可变形式借用
  <3> 以不可变形式借用

  📒 函数类型类型的3种形式
  <1> FnOnce: 获取作用域中变量的所有权，
              Once表示闭包只能获取一个变量的所有权，并且只能获取一次，
              不能多次获取同一变量的所有权，也就是这种闭包只能调用一次。
  <2> FnMut:  以可变形式借用变量
  <3> Fn:     以不可变形式借用变量

  📒 在Rust中当创建一个闭包时，会根据在闭包中的对值的使用方式，自动推导出你的闭包使用了哪一种函数特征
*/

fn move_closure() {
    let x = vec![1, 2, 3];
    // equal_to_x: imple Fn(Vec<i32>) -> boole
    let equal_to_x = move |z: Vec<i32>| z == x;
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
    // 由于变量x被移入至闭包中，所以x的所有权发生的转移，后续将不能再使用变量x
    // [ERROR] borrow of moved value: `x` value borrowed here after move
    // println!("can not use x here: {:?}", x); // ❌
}