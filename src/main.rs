use std::io;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::prelude::*;
use chrono::NaiveDate;
use chrono::format::ParseError;
use std::process;

/**
 * TODO:
 * [y] list all books
 * [y] search book by name
 * [y] create new book entry
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
    issue_date: NaiveDate
}

impl Book {
    pub fn to_string(&self) -> String {
        format!("{}", format!("{},{},{},{},{}\n", 
        &self.name.trim(), 
        &self.author.trim(), 
        &self.year_published.to_string(), 
        &self.borrowed, 
        &self.issue_date.to_string()))
    }
}

fn main() {
    loop {
        match menu() {
            Ok(num) => {
                match num {
                    0 => process::exit(0),
                    1 => list_all_book(),
                    2 => search_a_book(),
                    3 => create_a_book(),
                    4 => println!("Your input is 4"),
                    5 => println!("Your input is 5"),
                    6 => delete_a_book(),
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

// list all book
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
                println!("Issue on       : {}", book.issue_date);
                println!("--------------------------------------------");
            }            
            println!();            
        }
    }
}

// list all book
fn to_book_list(data: &str) -> Result<Vec<Book>, ParseError>{
    let mut books = vec![];

    for line in data.split('\n') {
        // println!("{:?}", line);
        if line.trim() != "" {
            let slice_data: Vec<&str> = line.split(',').collect();
            // println!("{:?}", slice_data);
            let issue_date = NaiveDate::parse_from_str(slice_data[4].trim(), "%Y-%m-%d")?;
            books.push(Book {
                name: slice_data[0].trim().to_string(),
                author: slice_data[1].trim().to_string(),
                year_published: slice_data[2].trim().parse().unwrap_or_default(),            
                borrowed: slice_data[3].trim().to_string().parse().unwrap_or_default(),
                issue_date
            });
        }
    }
    Ok(books)
}

// search a book
fn search_a_book() {
    println!("Please enter your search:");
    let mut book_name = String::new();
    io::stdin()
        .read_line(&mut book_name)
        .expect("Please enter something to search..");

    if book_name.trim() == "" {
        println!("Please enter something...");
    } else {
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
                let search_result: Vec<Book> = books.into_iter()
                    .filter(|x| x.name.to_uppercase().contains(&book_name.trim().to_uppercase()))
                    .collect();                
                println!("\n#-------------------------------#");
                println!("#  Rusty Library Search Result  #"); 
                println!("#-------------------------------#");
                if search_result.len() > 0 {
                    println!("Result found: {}", search_result.len());
                    println!("--------------------------------------------");
                    for book in search_result {
                        println!("Book Name      : {}", book.name);
                        println!("Book Author    : {}", book.author);
                        println!("Published Year : {}", book.year_published);
                        println!("Borrow Status  : {}", book.borrowed);
                        println!("Issue on       : {}", book.issue_date);
                        println!("--------------------------------------------");
                    }
                } else {
                    println!("Nothing is found");
                }
                println!();       
            }
        }
    }
}

// create a book
fn create_a_book() {    
    println!("Insert new book to library");

    println!("Book Name :");
    let mut book_name = String::new();
    io::stdin()
        .read_line(&mut book_name)
        .expect("Please enter something...");

    println!("Book Author :");
    let mut author = String::new();
    io::stdin()
        .read_line(&mut author)
        .expect("Please enter something...");

    println!("Book Published Year [YYYY] :");
    let mut published_year = String::new();
    io::stdin()
        .read_line(&mut published_year)
        .expect("Please enter something...");

    println!("Issue Date [YYYY-MM-DD] :");
    let mut issue_date = String::new();
    io::stdin()
        .read_line(&mut issue_date)
        .expect("Please enter something...");
    
    let data_path = Path::new("librarystore");
    let mut file = OpenOptions::new()        
        .write(true)    
        .append(true)
        .open(&data_path)
        .unwrap();          
            
    if let Err(e) = write!(&mut file, "{}", format!("{}", format!("{},{},{},{},{}", 
        book_name.trim(), author.trim(), published_year.trim(), 0.to_string(), issue_date.to_string()))) {
        println!("{}", e);
    } else {
        println!();
        println!("#[ New book has been added to library ]#");        
        println!();
    }
}

// delete a book
fn delete_a_book() {
    println!("Please select a book from below:");
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
            for (i, book) in books.iter().enumerate() {
                println!("[{}]", i);
                println!("Book Name      : {}", book.name);
                println!("Book Author    : {}", book.author);
                println!("Published Year : {}", book.year_published);
                println!("Borrow Status  : {}", book.borrowed);
                println!("Issue on       : {}", book.issue_date);
                println!("--------------------------------------------");
            }        
            println!();            

            let mut inp = String::new();
            io::stdin()
                .read_line(&mut inp).unwrap();
            let delete_option: usize = inp.trim().parse().unwrap_or_default();
            let mut updated_file = match File::create(data_path) {
                Err(why) => panic!("Cannot create new file, {}", why),
                Ok(file) => file
            };
            let mut updated_data = String::new();

            for (i, book) in books.iter().enumerate() {
                if i != delete_option {
                    updated_data.push_str(&book.to_string());
                }
            }

            match updated_file.write(updated_data.as_bytes()) {
                Err(err) => println!("Error occured when update file, {}", err),
                Ok(_) => println!("#[ Library store has been updated ]#")
            }
        }
    }

    
}
