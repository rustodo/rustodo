extern crate chrono;

mod task;

#[cfg(test)]
mod tests {
    use task::Task;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_create_task() {
        let task = Task::new("I have something important to do!!");
        println!("{:?}", task);
    }
}

