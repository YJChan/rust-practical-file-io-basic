use std::io;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::prelude::*;
use chrono::{NaiveDate, Utc, DateTime};
use chrono::format::ParseError;
use std::process;

/**
 * TODO:
 * [y] list all books
 * [y] search book by name
 * [y] create new book entry
 * [y] issue book to borrower
 * [y] collect book from borrower
 * [y] check late payment is needed during book return (14 days consider late, fix rate $0.50)
 * [y] delete book
 * 
 * using file to store all the related data
 * display a management menu
 */
#[derive(Copy, Clone)]
struct Book<'b> {
    name: &'b str,
    author: &'b str,
    year_published: u32,
    borrowed: bool,
    issue_date: NaiveDate
}

impl<'b> Book<'b> {
    pub fn to_string(&self) -> String {
        format!("{}", format!("{},{},{},{},{}\n", 
        &self.name.trim(), 
        &self.author.trim(), 
        &self.year_published.to_string(), 
        &self.borrowed, 
        &self.issue_date.to_string()))
    }
}

// struct Borrower<'a> {
//     name: &'a str,
//     book_name: &'a str, 
//     borrow_date: &'a NaiveDate,
// }

// impl<'a> Borrower<'a> {
//     pub fn to_string(&'a self) -> String {
//         format!("{}", format!("{},{},{}\n",
//         &self.name,
//         &self.book_name,
//         &self.borrow_date))
//     }
// }

fn main() {
    loop {
        match menu() {
            Ok(num) => {
                match num {
                    0 => process::exit(0),
                    1 => list_all_book(),
                    2 => search_a_book(),
                    3 => create_a_book(),
                    4 => borrow_a_book(),
                    5 => return_a_book(),
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
        if line.trim() != "" {
            let slice_data: Vec<&str> = line.split(',').clone().collect();            
            let issue_date = NaiveDate::parse_from_str(slice_data[4].trim(), "%Y-%m-%d")?;
            books.push(Book {
                name: slice_data[0].trim(),
                author: slice_data[1].trim(),
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
                    for (i, book) in search_result.iter().enumerate() {
                        println!("[{}]", i);
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

// borrow a book
fn borrow_a_book() {
    println!("Please enter the book name you want to borrow:");
    let mut book_name = String::new();
    io::stdin()
        .read_line(&mut book_name)
        .expect("Please enter something to search..");

    if book_name.trim() == "" {
        println!("Please enter something...");
    } else {
        let data_path = Path::new("librarystore");
        let mut library_file = match File::open(&data_path) {
            Err(why) => panic!("No library data store, {}", why),
            Ok(file) => file
        };       
        
        let mut data = String::new();
        match library_file.read_to_string(&mut data) {
            Err(why) => println!("Error when reading file, {}", why),
            Ok(_) => {
                let books: Vec<Book> = match to_book_list(&data) {
                    Err(_) => vec![],
                    Ok(b) => b
                };
                let search_result: Vec<&Book> = books.iter()
                    .filter(|x| x.name.to_uppercase().contains(&book_name.trim().to_uppercase()) && x.borrowed == false)
                    .clone().collect();
                println!("\n#-------------------------------#");
                println!("#  Rusty Library Search Result  #"); 
                println!("#-------------------------------#");
                if search_result.len() > 0 {
                    println!("Result found: {}", search_result.len());
                    println!("--------------------------------------------");
                    for (i, book) in search_result.iter().enumerate() {
                        println!("[{}]", i);
                        println!("Book Name      : {}", book.name);
                        println!("Book Author    : {}", book.author);
                        println!("Published Year : {}", book.year_published);                        
                        println!("Issue on       : {}", book.issue_date);
                        println!("--------------------------------------------");
                    }
                } else {
                    println!("Book is borrowed others\n");
                    return;
                }
                println!();

                println!("Please enter the book number you want to borrow:");
                let mut book_number = String::new();
                io::stdin()
                    .read_line(&mut book_number)
                    .expect("Please enter a valid number");

                let book_number: isize = book_number.trim().parse().unwrap_or(-1);

                if book_number == -1 {
                    println!("Please enter a valid book number");
                } else {                    
                    let book = &search_result.get(book_number as usize).unwrap();
                    let path = Path::new("borrower");        
                    let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .create(true)
                                    .open(&path)
                                    .unwrap();                    

                    println!("Please enter the borrower name:");
                    let mut borrower_name = String::new();
                    io::stdin()
                        .read_line(&mut borrower_name)                    
                        .expect("Please enter the borrow name");                

                    println!("Please enter the borrow date [YYYY-MM-DD]:");
                    let mut borrow_date = String::new();
                    io::stdin()
                        .read_line(&mut borrow_date)                    
                        .expect("Please enter the borrow date");

                    if let Err(e) = write!(&mut file, "{}", format!("{}", 
                        format!("{},{},{}\n", borrower_name.trim(), book.name.trim(), borrow_date.trim()))) {
                            println!("Error when writing file, {}", e);
                        }
                    
                    let mut book_index = 0;
                    for (i, b) in books.iter().enumerate() {
                        if &book.name == &b.name {
                            println!("{}", &book.name);
                            book_index = i;
                        }
                    }
                    
                    let mut updated_file = match File::create(data_path) {
                        Err(why) => panic!("Cannot create new file, {}", why),
                        Ok(file) => file
                    };
                    let mut updated_data = String::new();
                    for (i, line) in data.split('\n').enumerate() {
                        let new_line: String;                     
                        if i == book_index {
                            let mut slice_data: Vec<&str> = line.split(',').clone().collect();
                            slice_data[3] = "true";
                            new_line = format!("{},{},{},{},{}\n", 
                                slice_data[0].trim(),
                                slice_data[1].trim(),
                                slice_data[2].trim(),
                                slice_data[3].trim(),
                                slice_data[4].trim());
                        } else {
                            new_line = format!("{}\n", line);
                        }
                        updated_data.push_str(&new_line);
                    }

                    match updated_file.write_all(updated_data.as_bytes()) {
                        Err(err) => println!("Error occured when update file, {}", err),
                        Ok(_) => println!("#[ Library store has been updated ]#")
                    }

                    println!("#[ {} has been borrowed by {} ]#", &book.name.trim(), &borrower_name.trim());
                }
            }
        }
    }    
}

fn return_a_book() {
    println!("Please enter the borrower name:");
    let mut borrower_name = String::new();
    io::stdin()
        .read_line(&mut borrower_name)
        .unwrap();
    
    println!("Please enter book that want to return:");
    let mut borrowed_book = String::new();
    io::stdin()
        .read_line(&mut borrowed_book)
        .unwrap();    
    let mut library_data = String::new();
    let library_path = Path::new("librarystore");
    let mut library_file = match File::open(library_path) {
        Err(err) => panic!("Failed to open library file, {}", err),
        Ok(file) => file
    };
        
    match library_file.read_to_string(&mut library_data) {
        Err(err) => println!("Failed to read library file, {}", err),
        Ok(_) => {
            let books: Vec<Book> = match to_book_list(&library_data) {
                Err(_) => vec![],
                Ok(b) => b
            };
            let search_result: Vec<&Book> = books.iter()
                .filter(|x| x.name.to_uppercase().contains(&borrowed_book.trim().to_uppercase()) && x.borrowed == true)
                .clone().collect();
            println!("\n#-------------------------------#");
            println!("#  Rusty Library Search Result  #"); 
            println!("#-------------------------------#");
            if search_result.len() > 0 {
                println!("Result found: {}", search_result.len());
                println!("--------------------------------------------");
                for (i, book) in search_result.iter().enumerate() {
                    println!("[{}]", i);
                    println!("Book Name      : {}", book.name);
                    println!("Book Author    : {}", book.author);
                    println!("Published Year : {}", book.year_published);                        
                    println!("Issue on       : {}", book.issue_date);
                    println!("--------------------------------------------");
                }
            } else {
                println!("No books found with this name\n");
                return;
            }
            println!("Please enter the book number you want to return:");
            let mut book_number = String::new();
            io::stdin()
                .read_line(&mut book_number)
                .expect("Please enter a valid number");

            let book_number: isize = book_number.trim().parse().unwrap_or(-1);

            if book_number == -1 {
                println!("Please enter a valid book number");
            } else {
                if book_number as usize >= search_result.len() {
                    println!("Please enter a valid option from the display list");
                    return;
                }
                let book = &search_result.get(book_number as usize).unwrap();
                let borrower_path = Path::new("borrower");
                let mut borrower_file = match File::open(borrower_path) {
                    Err(err) => panic!("Failed to open borrower file, {}", err),
                    Ok(file) => file
                };
            
                let mut borrower_data = String::new();
                match borrower_file.read_to_string(&mut borrower_data) {
                    Err(err) => println!("Failed to read borrower file, {}", err),
                    Ok(_) => {            
                        let mut updated_borrower_data = String::new();                        
                        for line in borrower_data.split('\n') {
                            if line.trim().len() > 0 {
                                let slice_data: Vec<&str> = line.split(',').clone().collect();
                                if slice_data[0].to_uppercase() == borrower_name.trim().to_uppercase() 
                                    && slice_data[1] == book.name {
                                    let borrower_date = DateTime::parse_from_str(format!("{}  00:00:0.000 +0000", slice_data[2].trim()).as_str(), "%Y-%m-%d %H:%M:%S%.3f %z").unwrap();
                                    let today: DateTime<Utc> = Utc::now();
                                    let diff = borrower_date.signed_duration_since(today).num_days();
                                    // println!("day: {}", diff);
                                    if diff < -14 {
                                        let late_payment: f64 = (diff * -1) as f64 * 0.5;
                                        println!("Borrower has late payment of {} days,", diff);
                                        println!("Borrower need to pay {} ", late_payment);

                                        let mut done_payment = String::new();
                                        println!("Type [done] after borrower make payment");
                                        io::stdin()
                                            .read_line(&mut done_payment)
                                            .unwrap();
                                        if done_payment.trim() != "done" {
                                            println!("Please type correct command, 'done' in order to finish the operation\nRetry again\n");
                                            return;
                                        }
                                    }                                    
                                } else {
                                    updated_borrower_data.push_str(line);
                                    updated_borrower_data.push_str("\n");
                                }
                            }
                        }
            
                        let mut updated_borrower_file = match File::create(borrower_path) {
                            Err(why) => panic!("Cannot create new file, {}", why),
                            Ok(file) => file
                        };
                        match updated_borrower_file.write_all(updated_borrower_data.as_bytes()) {
                            Err(err) => println!("Error occured when update file, {}", err),
                            Ok(_) => println!("#[ Borrower data has been updated ]#")
                        };
                    }
                }

                let mut book_index = 0;
                for (i, b) in books.iter().enumerate() {
                    if &book.name == &b.name {
                        book_index = i;
                    }
                }
                
                let mut updated_file = match File::create(library_path) {
                    Err(why) => panic!("Cannot create new file, {}", why),
                    Ok(file) => file
                };
                let mut updated_library_data = String::new();
                for (i, line) in library_data.split('\n').enumerate() {
                    let new_line: String;
                    if i == book_index {
                        let mut slice_data: Vec<&str> = line.split(',').clone().collect();
                        slice_data[3] = "false";
                        new_line = format!("{},{},{},{},{}\n", 
                            slice_data[0].trim(),
                            slice_data[1].trim(),
                            slice_data[2].trim(),
                            slice_data[3].trim(),
                            slice_data[4].trim());
                    } else {
                        new_line = format!("{}\n", line);
                    }
                    updated_library_data.push_str(&new_line);
                }

                match updated_file.write_all(updated_library_data.as_bytes()) {
                    Err(err) => println!("Error occured when update file, {}", err),
                    Ok(_) => println!("#[ Library store has been updated ]#")
                }

                println!("#[ {} has been return by {} ]#\n", &book.name.trim(), &borrower_name.trim());
            }
        }
    };    
    
}