// 嵌套路径来消除大量的 use 行

// [case-1] 
use std::cmp::Ordering;
use std::io;
// ...

// ...
use std::{cmp::Ordering, io};
// ...

// [case-2] self用法
// ...
use std::io;
use std::io::Write;
// ...

// ...
// 这里的self表示std::io包自身 
use std::io::{self, Write};
// ...

// [case-3] 导入包下所有内容
// 注意：
//      不推荐使用这样方式，因为有时候可能只是使用一个包下的部分内容，按需进行导入为了保持代码纯净
use std::io::*;