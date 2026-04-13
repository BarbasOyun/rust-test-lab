mod multithread;
mod async_test;

fn main() {
    test_multithreading();
    test_async();
}

fn test_multithreading() {
        // multithread::test_thread();
    // multithread::data_from_main_to_spawned();
    // multithread::test_channel();
    // multithread::multiple_data_channel();
    // multithread::multiple_producer_single_consumer();
    // multithread::test_mutex();
    // multithread::threads_mutex();
    multithread::dead_lock(5, 2, 3);
}

fn test_async() {
        let urls = vec![
        String::from("https://www.rust-lang.org"),
        String::from("https://en.wikipedia.org/wiki/Wiki"),
    ];

    // async_test::test_runtime();
    // async_test::get_urls(urls[0].clone(), urls[1].clone());
    // async_test::url_race(&urls[0], &urls[1]);
    // async_test::async_count();
    // async_test::async_count2();
    // async_test::async_channel();
    // async_test::multiple_senders();
    // async_test::future_starving();
    // async_test::future_feeding();
    // async_test::timeout_test();
    // async_test::stream_test();
    async_test::mix_thread_async();
}