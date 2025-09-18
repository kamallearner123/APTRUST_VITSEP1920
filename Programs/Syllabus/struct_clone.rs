#[derive(Debug,Clone, Copy)]
struct book <'a> {
    name:&'a str,
    date:&'a str,
    quantity:i32,
//    authors: Vec<String>
}

fn main() {
    let b1 = book{name:"Rust programmong",
                  date:"19-Sep-2025",
                  quantity:10,
                  //authors:vec!["richard".to_string(),"stevens".to_string()]
                  };

    let cb1 = b1;//.clone();

    let v1: Vec<book> = vec![b1];
    for book in v1 {
        println!("name {}",book.name);
        println!("full details = {:?}", book);
    }

    println!("cb1 = {:?}", cb1);
}
