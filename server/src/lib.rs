use std::{sync::{mpsc::{self, Receiver}, Arc, Mutex}, thread};

pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}


impl ThreadPool {
    ///Create a new thread pool.
    /// 
    /// Size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The 'new' function will panic if the size is non positive.
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let mut workers=Vec::with_capacity(size);

        let (sender,reciver) = mpsc::channel();
        let reciver = 
            Arc::new(Mutex::new(reciver));

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&reciver)));
        }
        
        ThreadPool {workers, sender}
    }

    pub fn execute<F>(&self, f: F)
    where
    F: FnOnce() + Send + 'static{
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("Terminating all workers.");

        for _ in &self.workers{
            self.sender.send(Message::Terminate).unwrap();
        }
        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}


struct Worker{
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, reciver: Arc<Mutex<Receiver<Message>>>) -> Worker{
        let thread = thread::spawn(move || loop {
            let message = reciver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job!", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} terminating.", id);
                    break;
                }
            }
            
            });
        Worker {id, thread: Some(thread)}
    }
    
}