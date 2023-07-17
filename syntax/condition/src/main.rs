fn main() {
    // [case-1]
    let number = 11;

    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    // [case-2]
    // if表达式
    let flag = true;
    let number = if flag { 5 } else { 6 };
    println!("number value: {}", number);

    // [case-3]
    // loop循环
    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter value: {}", counter);
        if counter == 10 {
            break;
        }
    }

    // [case-4]
    // loop + break 表达式
    let mut cnt = 0;
    let result = loop {
        cnt += 1;
        if cnt == 20 {
            break cnt;
        }
    };
    println!("result value: {}", result);

    // [case-5]
    // while循环
    let mut number = 3;
    while number != 0 {
        number -= 1
    }
    println!("number value: {}", number);

    // [case-6]
    // for循环遍历数组
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("array element is: {}", *element);
    }

    // [case-7]
    // 通过range数值序列遍历
    for num in 1..4 {
        println!("rannge element is: {}", num);
    }
}
