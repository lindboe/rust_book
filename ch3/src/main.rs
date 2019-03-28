fn f_to_c(fahr: i32) -> i32 {
    (fahr - 32) * 5 / 9
}

fn fibonacci(n: i32) -> i32 {
    let mut last_two: (i32, i32) = (0, 1);
    if n == 1 {
        last_two.0
    } else if n == 2 {
        last_two.1
    } else {
        // start at 2 because we've already figured out the first two
        for _i in 2..n {
            let next = last_two.0 + last_two.1;
            last_two = (last_two.1, next);
        }
        last_two.1
    }
}

fn twelve_days_lyrics() {
    // it doesn't print the 'and' before the partridge. I cheated.
    let gifts = ["a partridge in a pear tree",
                 "two turtle doves",
                 "three french hens",
                 "four calling birds",
                 "five golden rings",
                 "six geese a-laying",
                 "seven swans a-swimming",
                 "eight maids a-milking",
                 "nine ladies dancing",
                 "ten lords a-leaping",
                 "eleven pipers piping",
                 "twelve drummers drumming"
    ];

    let numbers = ["first",
                   "second",
                   "third",
                   "fourth",
                   "fifth",
                   "sixth",
                   "seventh",
                   "eighth",
                   "ninth",
                   "tenth",
                   "eleventh",
                   "twelfth"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas, my true love gave to me:", numbers[i]);
        for n in (0..i+1).rev() {
            println!("{}", gifts[n])
        }
        println!("");
    }
}

fn main() {
    let celsius = f_to_c(32);
    println!("32F is {}C", celsius);

    let mut n = 6;
    println!("The {} fibonacci number is {}.", n, fibonacci(n));
    n = 1;
    println!("The {} fibonacci number is {}.", n, fibonacci(n));
    n = 2;
    println!("The {} fibonacci number is {}.", n, fibonacci(n));
    n = 3;
    println!("The {} fibonacci number is {}.", n, fibonacci(n));

    twelve_days_lyrics();
}
