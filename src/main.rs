use std::io;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use chrono::NaiveDate;
use chrono::format::ParseError;
use std::process;

/**
 * TODO:
 * [] list all books
 * [] search book by name
 * [] create new book entry
 * [] issue book to borrower
 * [] collect book from borrower
 * [] check late payment is needed during book return
 * [] delete book
 * 
 * using file to store all the related data
 * display a management menu
 */
struct Book {
    name: String,
    author: String,
    year_published: u32,
    borrowed: bool,
    last_borrow_date: NaiveDate
}

fn main() {
    loop {
        match menu() {
            Ok(num) => {
                match num {
                    0 => process::exit(0),
                    1 => list_all_book(),
                    2 => println!("Your input is {}", 2),
                    3 => println!("Your input is {}", 3),
                    4 => println!("Your input is {}", 4),
                    5 => println!("Your input is {}", 5),
                    6 => println!("Your input is {}", 6),
                    _ => println!("Please enter from the option provided")
                }
            },            
            Err(_) => {
                println!("You have entered an invalid input!!");
                continue
            }
        };        
    }
}

fn menu() -> Result<u8, i8> {
    println!("#----------------------------#");
    println!("#  Welcome to Rusty Library  #"); 
    println!("#----------------------------#");
    println!("#  Please input your option  #");
    println!("#  [1] list all book         #");
    println!("#  [2] search a book         #");
    println!("#  [3] create a book         #");
    println!("#  [4] borrow a book         #");
    println!("#  [5] return a book         #");
    println!("#  [6] delete a book         #");
    println!("#  [0] exit program          #");
    println!("#----------------------------#");
    
    // read user input
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read user input");
    match inp.trim().parse() {
        Ok (num) => Ok(num),
        Err(_) => Err(-1)
    }    
}

fn list_all_book() {
    let data_path = Path::new("librarystore");
    let mut file = match File::open(&data_path) {
        Err(why) => panic!("No library data store, {}", why),
        Ok(file) => file
    };

    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Err(why) => println!("Error when reading file, {}", why),
        Ok(_) => {
            let books: Vec<Book> = match to_book_list(&data) {
                Err(_) => vec![],
                Ok(b) => b
            };
            println!("\n#----------------------------#");
            println!("#  Rusty Library Book List   #"); 
            println!("#----------------------------#");            
            for book in books {
                println!("Book Name      : {}", book.name);
                println!("Book Author    : {}", book.author);
                println!("Published Year : {}", book.year_published);
                println!("Borrow Status  : {}", book.borrowed);
                println!("Last Borrow on : {}", book.last_borrow_date);
                println!("--------------------------------------------");
            }            
            println!("");
        }
    }
}

fn to_book_list(data: &String) -> Result<Vec<Book>, ParseError>{
    let mut books = vec![];

    for line in data.split("\n") {
        println!("{:?}", line);
        if line.trim() != "" {
            let slice_data: Vec<&str> = line.split(",").collect();
            println!("{:?}", slice_data);
            let last_borrow_date = NaiveDate::parse_from_str(slice_data[4].trim(), "%Y-%m-%d")?;
            books.push(Book {
                name: slice_data[0].trim().to_string(),
                author: slice_data[1].trim().to_string(),
                year_published: slice_data[2].trim().parse().unwrap_or_default(),            
                borrowed: slice_data[3].trim().to_string().parse().unwrap_or_default(),
                last_borrow_date: last_borrow_date
            });
        }
    }
    Ok(books)
}