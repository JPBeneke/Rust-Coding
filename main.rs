fn main() {
    let numbers = 123456789;
    let letters1: String = numbers.to_string();  /* Convert numbers to letters (string). */
    let letters2 = String::from("Zero");  /* Define string letters2 as "Zero". */

    println!("The Number is: {}\n", letters1);  /* This is only letters/text. */
    println!("The Number is: {}{}\n", letters2, letters1);  /* This is only letters/text. */
    println!("{}", numbers);  /* This is a number. */
}
