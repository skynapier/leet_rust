
fn main() {
    let s = String::from("cbbd");
    let len = s.len();
    let mut palindromic = &s[0..1];

    let mut c = len - 1;

    for i in 0..len{
        let start_char = s.as_bytes()[i];
        while c > i{
            let end_char = s.as_bytes()[i];
            if start_char == end_char {
                let sub_str = &s[i..=c];
                println!("{}", sub_str);
                if is_palindromic(&s[i..=c]) &&  sub_str.len() > palindromic.len(){
                    palindromic = sub_str;
                }
            }
            c -= 1;
        }
        c = len - 1;
    }

    println!("{:?}", palindromic);

    fn is_palindromic(str:&str)->bool{
        let len = str.len();
        for i in 0..len{
            if str.as_bytes()[i] != str.as_bytes()[len - i - 1]{
                return false;
            }
        }
        true
    }

}
