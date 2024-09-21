use std::io::{self, Write};

fn count_letters(text: &str ) -> usize 
{
    // 
    text.chars().filter(|c| c.is_alphabetic()).count()
}

fn count_words(text: &str) -> usize 
{
    text.split_whitespace().count()
}

fn count_sentence(text: &str) -> usize 
{
    text.chars().filter(|&c| c == '.' || c == '!' || c == '?').count()
}

fn calculate_grade(text: &str) -> f64 
{
    let letters = count_letters(text);
    let words = count_words(text);
    let sentences = count_sentence(text);

    if words == 0
    {
        return 0.0;
    }

    let l = (letters as f64 / words as f64) * 100.0;
    let s = (sentences as f64 / words as f64) * 100.0;

    0.0588 * l - 0.296 * s - 15.8
}

fn main() 
{

    // initialize input variable
    let mut input = String::new();
    print!("Enter the text: ");
    // push any buffered output to the terminal
    // ensures the text enter the text shown to the user before the input
    io::stdout().flush().unwrap();
    // ask for input from user
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // initialize grade variable and assign into the grade that return float
    let grade = calculate_grade(&input);
    // round the grade into int because its from float
    let grade_rounded = grade.round();

    if grade_rounded >= 16.0 
    {
        println!("Grade 16+");
    }
    else if grade_rounded < 1.0
    {
        println!("Before Grade 1");
    }
    else
    {
        println!("Grade {}", grade_rounded);
    }
}
