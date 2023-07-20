// 引入不同的路径，但路径下包含相同的函数或者结构体，如何解决冲突
/*
// std::fmt包和std::io都有同名的Result
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // to doing something
}

fn function2() -> io::Result<()> {
    // to doing something
}
*/

// 可以通过use + as 重命名引入解决冲突
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // to doing something
}

fn function2() -> IoResult<()> {
    // to doing something
}