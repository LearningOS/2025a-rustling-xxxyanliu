// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    // 结构体内保存的是引用，不拥有数据，借用其他的，只要Book<'a>还活着，author和title在这个生命周期内就是有效的
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
