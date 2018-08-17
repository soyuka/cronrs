#[macro_use]
extern crate nom;

#[derive(Debug,PartialEq,Eq)]
enum SpecialString {
  Daily,
  Reboot,
  Yearly,
  Weekly,
  Hourly,
  Monthly,
  Annually,
  Midnight,
  Unknown(usize)
}

fn is_not_space(c: u8) -> bool { ! nom::is_space(c)  }
// fn is_not_eol(c: u8) -> bool { c != b'\n' }

named!(special_string<SpecialString>,
    alt!(
        tag!("@daily")    => { |_| SpecialString::Daily }    |
        tag!("@reboot")   => { |_| SpecialString::Reboot }   |
        tag!("@yearly")   => { |_| SpecialString::Yearly }   |
        tag!("@weekly")   => { |_| SpecialString::Weekly }   |
        tag!("@hourly")   => { |_| SpecialString::Hourly }   |
        tag!("@monthly")  => { |_| SpecialString::Monthly }  |
        tag!("@annually") => { |_| SpecialString::Annually } |
        tag!("@midnight") => { |_| SpecialString::Midnight } |
        take_while!(is_not_space)      => { |r: &[u8]| SpecialString::Unknown(r.len()) }
    )
);

named!(comment<&[u8], (&[u8], &[u8])>, pair!(tag!("#"), take_until!("\n")));

// named!(full_greeting<&str, (&str, &str)>,
//   pair!(get_greeting, complete!(nom::alpha))
// );

fn main() {
    let res = special_string(b"@reboot command");
    println!("{:?}", res);
    let res = special_string(b"1-3 ");
    println!("{:?}", res);
    let comment = comment(b"# lala some comment");
    println!("{:?}", comment);
    // println!("result {:?}", full_greeting(" hi bob"));
}
