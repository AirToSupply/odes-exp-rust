/*
   Rust面向对象设计模式-状态模式（state pattern）

   ❓ 状态模式（state pattern）
      定义一系列值的内含状态，这些状态体现为一系列的状态对象，同时值的行为随着其内部状态而改变。

   ❓ 状态模式（state pattern）原来
      程序的业务需求改变时，无需改变值持有状态或者使用值的代码。只需更新某个状态对象中的代码来改变其规则，或者是增加更多的状态对象。
 */

use blog::Post;

/*
   需求：博文发布
        博文从空白的草案开始。
        一旦草案完成，请求审核博文。
        一旦博文过审，它将被发表。
        只有被发表的博文的内容会被打印，这样就不会意外打印出没有被审核的博文的文本。
 */
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
