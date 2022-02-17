fn str1() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);

    println!("Words in reverse");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // 复制字符到vector，排序并移除重复值
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // 创建一个空的String
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(",");
    }

    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // 堆分配一个字符串
    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}

fn str2() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
             unicode_codepoint, character_name);

    let long_string = "String literals\
        can span multiple lines.\
        The linebreak and indentation here ->\
        <- can be escaped too!";
    println!("{}", long_string);
}

fn str3() {
    let raw_str = r"Escape don't work here: \x3F \u{211D}";
    println!("{}", raw_str);

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let longer_delimiter = r###"A string with "# in it. And event "##!"###;
    println!("{}", longer_delimiter);
}

use std::str;

fn str4() {
    let bytestring: &[u8; 20] = b"this is a bytestring";
    println!("A bytestring: {:?}", bytestring);

    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);

    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);
    }

    let quotes = br#"You can also use "fancier" formatting, \
        like with normal raw strings"#;

    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82";

    match str::from_utf8(shift_jis) {
        Ok(mys_str) => println!("Conversion successful: '{}'", mys_str),
        Err(e) => println!("Conversion failed: {:?}", e),
    }
}

fn main() {
    str1();
    str2();
    str3();
    str4();
}