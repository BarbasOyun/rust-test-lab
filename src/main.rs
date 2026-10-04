mod mtr_calc;
mod spread_sheets;
mod async_test;
mod multithread;

use std::{io, cmp::Ordering, time::Instant};

use rand::{Rng, RngExt};

// Ferris say Exercise
use ferris_says::say;
use std::io::{BufWriter, stdout};

fn main() {
    spread_sheets::create_names_workbook();
    spread_sheets::create_factions_workbook();
    spread_sheets::create_attributes_workbook();
    
    // spread_sheets::create_spread_sheet_test();

//     let string = String::from(
//         "-Hyreim
//          -Rakvar
//          -Sjaard
//          -Rekvam",
//     );
//     string_analyzer(string);

    // guessing_game();
    // mtr_calc::balance_calculation();

    // time_function(|| {
    //     let fibonacci_nbr = fibonacci_nbr(185);
    //     println!("Fibonacci nbr : {fibonacci_nbr}");
    // });

    // test_multithreading();
    // test_async();
}

fn ferris_say(message: String) {
    let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();
}

fn guessing_game() {
    // println!("Guess the number!");
    ferris_say(String::from("Guess the number!"));

    let mut rng = rand::rng();
    let secret_number: u8 = rng.random_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Your guess :");

        let mut guess = String::new();

        io::stdin() // Create a handle to read terminal input
            .read_line(&mut guess) // Read input
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn string_analyzer(string: String) {
    println!("---Analyzing String");
    println!("String Length = {}", string.capacity());

    println!("Characters :");
    let mut count = 0;
    for character in string.chars() {
        count += 1;
        println!("{} = {}", count, character);
    }
}

/// Measure the execution time of a function
fn time_function(function: fn()) {
    let start = Instant::now();

    function();

    let duration = start.elapsed();

    println!("Time elapsed is: {:?}", duration);
}

fn fibonacci_nbr(n: u64) -> u128 {
    // Fibonacci Sequence start with 0, 1, each number in the sequence is the sum of the last 2
    // Avoid Recusivity -> Iterative
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut is_alt = false;

    if n < 2 {
        return if n < 1 { a } else { b };
    }

    for _ in 2..n {
        if is_alt {
            b = a + b;
        }else {
            a = a + b;
        }

        is_alt = !is_alt;
    }

    return a + b
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
