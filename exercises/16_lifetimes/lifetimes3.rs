// Lifetimes are also needed when structs hold references.

// TODO: Fix the compiler errors about the struct.
//'a 表示一泛型生命周，它要求结构体 Book 实例的存活时间不能超过其字段 author 和 title 所引用的数据的存活时间
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
