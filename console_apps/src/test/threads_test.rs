#[cfg(test)]
mod tests {
    use crate::threads::{sender_reciever, thread_result_handling, threads_and_sync, wait_a_bit};

    #[test]
    fn test_sender_reciever() {
        let exp = String::from("message: Hello world!");
        let res = sender_reciever(String::from("Hello world!"));
        assert_eq!(exp, res);
    }

    #[test]
    fn test_thread_result_handling() {
        let exp = String::from("message: Hello world!");
        let res = thread_result_handling(String::from("Hello world!"));
        assert_eq!(exp, res);
    }

    #[test]
    fn test_threads_and_sync() {
        let exp = String::from("abcde12345678910");
        let res = threads_and_sync();
        assert_eq!(exp, res);
    }

    #[test]
    fn test_wait_a_bit() {
        let exp = String::from("waited 10 seconds!");
        let res = wait_a_bit(10);
        assert_eq!(exp, res);
    }
}
