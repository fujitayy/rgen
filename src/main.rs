use std::iter::FromIterator;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Args {
    #[structopt(long)]
    no_marks: bool,
    #[structopt(short, long, default_value = "32")]
    length: usize,
}

/// 候補文字は素数(89)個
const CS: &str = r##"abcdefghijklmnopqlstuvwxyzABCDEFGHIJKLMNOPQLSTUVWXYZ0123456789!#$%&()=-^|[{]}@:*;+/?_.>,<"##;

fn main() {
    let args = Args::from_args();
    println!("{}", gen_password(!args.no_marks, args.length));
}

fn gen_password(include_marks: bool, length: usize) -> String {
    let parts = (0..length).map(|_| {
        let cs = select_char_table(include_marks);
        let i = rand::random::<usize>() % cs.len();
        cs.get(i..i + 1).unwrap_or("a")
    });
    String::from_iter(parts)
}

fn select_char_table(include_marks: bool) -> &'static str {
    if include_marks {
        &CS[..]
    } else {
        &CS[0..62]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_table_length_is_prime_number() {
        assert_eq!(CS.len(), 89);
    }

    #[test]
    fn select_char_table_include_marks() {
        assert_eq!(select_char_table(true), CS);
    }

    #[test]
    fn select_char_table_no_include_marks() {
        assert_eq!(
            select_char_table(false),
            "abcdefghijklmnopqlstuvwxyzABCDEFGHIJKLMNOPQLSTUVWXYZ0123456789"
        );
    }

    #[test]
    fn password_length_is_exact_set_one() {
        assert_eq!(gen_password(true, 0).len(), 0);
        assert_eq!(gen_password(true, 1).len(), 1);
        assert_eq!(
            gen_password(true, std::u16::MAX as usize).len(),
            std::u16::MAX as usize
        );
        assert_eq!(gen_password(false, 0).len(), 0);
        assert_eq!(gen_password(false, 1).len(), 1);
        assert_eq!(
            gen_password(false, std::u16::MAX as usize).len(),
            std::u16::MAX as usize
        );
    }
}
