fn main() {
    // Create an array of 10 integer numbers of your choice
    let numbers: [i32; 10] = [25, 26, 27, 28, 29, 30, 31, 32, 33, 34];

    // Function to check if a number is even
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    // Iterate through the array and print whether each number is even or odd,
    // and also print Fizz, Buzz, or FizzBuzz if applicable
    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else {
            let even_or_odd = if is_even(num) { "even" } else { "odd" };
            println!("{}: {}", num, even_or_odd);
        }
    }

    // Use a while loop to find and print the sum of all numbers in the array
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Use a loop to find and print the largest number in the array
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}