fn main() {
    let svar = String::from("Hi");
    let len = refrence_borrowing(svar);
    println!("Len => {} ", len);
    println!("Strs => {} ", svar);
}

fn refrence_borrowing(s: String) -> usize{
    s.len()
}
