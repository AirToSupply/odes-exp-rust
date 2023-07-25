// 定义博文结构体
pub struct Post {
    // 博文当前状态
    state: Option<Box<dyn State>>,
    // 博文内容 
    content: String,
}

// 定义博文结构体实现块
impl Post {
    pub fn new() -> Post {
        Post {
            // 初始化状态为草稿状态
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }

    // 添加博文
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    // 请求评审
    pub fn request_review(&mut self) {
        // 通过take方法从option中取值
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    // 评审通过
    pub fn approve(&mut self) {
        // 通过take方法从option中取值
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }

    // 获取博文内容
    pub fn content(&self) -> &str {
        // 注意：
        //       这里的state必须要调用as_ref去获取引用
        //       因为需要 Option 中值的引用而不是获取其所有权，此时返回的类型是：Option<&Box<dyn State>>
        //       如果不调用as_ref，将会得到一个错误，因为不能将 state 移动出借用的 &self 函数参数
        self.state.as_ref().unwrap().content(self)
    }
}

// 定义状态特征
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

// 定义草稿状态
struct Draft {

}

impl State for Draft {
    // 请求博文评审：草稿状态 -> 待评审状态
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

}

// 定义待评审状态
struct PendingReview {

}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // 博文评审：待评审状态 -> 发布状态
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 定义发布状态
struct Published {

}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}