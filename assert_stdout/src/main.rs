use std::io::{self, BufRead, Write};

fn prompt<R, W>(mut reader: R, mut writer: W, question: &str) -> String
where
    R: BufRead,
    W: Write,
{
    write!(&mut writer, "{}", question).expect("Unable to write");
    let mut s = String::new();
    reader.read_line(&mut s).expect("Unable to read");
    s
}

#[test]
fn test_with_in_memory() {
    let input = b"I'm George";
    let mut output = Vec::new();

    let answer = prompt(&input[..], &mut output, "Who goes there?");

    let output = String::from_utf8(output).expect("Not UTF-8");

    assert_eq!("Who goes there?", output);
    assert_eq!("I'm George", answer);
}

fn main() {
    let stdio = io::stdin();
    let input = stdio.lock();

    let output = io::stdout();

    let answer = prompt(input, output, "Who goes there?");
    println!("was: {}", answer);
}
