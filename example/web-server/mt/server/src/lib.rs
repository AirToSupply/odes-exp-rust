use std::{thread::{JoinHandle, self}, sync::{mpsc, Arc, Mutex}};

/*
    线程池结构体
 */
pub struct ThreadPool {
    // 定义存放线程列表，元素类型可以thread::spawn方法的返回类型
    workers: Vec<Worker>,
    // 定义channel通道，所以这里必须要持有一个mpsc的发送端，它需要将执行请求发送给Worker线程进行执行
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// 创建线程池。
    ///
    /// 线程池中线程的数量。
    ///
    /// # Panics
    ///
    /// `new` 函数在 size 为 0 时会 panic。
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // 根据指定大小创建线程列表
        let mut workers = Vec::with_capacity(size);

        // 创建channel管道用于Job和Worker的任务收发
        let (sender, receiver) = mpsc::channel();

        // 创建线程安全并且支持多所有权共享的智能指针来解决下面循环初始化线程时receiver move移入问题
        // receiver: Arc<Mutex<Receiver<Job>>>
        let receiver  = Arc::new(Mutex::new(receiver));

        // 初始化线程，即Worker对象
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // 该方法签名可以参考thread::spawn方法签名
    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce(),
        F: Send + 'static,
    {
        let job = Box::new(f);
        // 将执行请求放入channel通道中于后续执行
        self.sender.send(job).unwrap();
    }
}

/*
    线程元数据结构体
    
    ❓ 为什么需要这个结构体
    因为处理逻辑被封装为闭包一旦被传入则闭包函数就会直接执行
    所以需要一个类似描述线程元数据结构体记录请求的元数据，这里包括被封装闭包，以及等地执行任何的线程
 */
struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        println!("[Worker(id = {})] startup", id);
        let thread = thread::spawn(move || {
            // 为了让每个Worker能够不断的执行请求，这里需要将整个逻辑让入死循环中
            // 如果从channel中能够recv到任务就是执行，否则就阻塞等待
            loop {
                // 从通道中获取执行请求进行执行
                // result: Receiver<Box<..>>
                let func = receiver.lock().unwrap().recv().unwrap();

                println!("[Worker(id = {})] execute job", id);

                // 这里是封装的代码，然后进行真正的执行
                func();
            }
        });
        Worker { id, thread }
    }
}

// 定义请求提交任务类型别名，用于代表存储执行请求的闭包类型
type Job = Box<dyn FnOnce() + Send + 'static>;