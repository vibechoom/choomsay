//! choomsay — a cyberpunk choom that speaks your text in a terminal bubble.
//!
//! Zero dependencies, pure std. Give it a message as arguments or pipe text in
//! on stdin, and it prints that message inside a speech (or `--think` thought)
//! bubble beside a little ASCII choom.

use std::io::{self, IsTerminal, Read};

const DEFAULT_WIDTH: usize = 40;

const HELP: &str = "\
choomsay — a cyberpunk choom that speaks your text

USAGE:
    choomsay [OPTIONS] [MESSAGE]...
    echo \"piped text\" | choomsay [OPTIONS]

OPTIONS:
    -w, --width <N>   wrap the bubble at N columns (default: 40)
    -t, --think       use a thought bubble instead of a speech bubble
    -h, --help        print this help
    -V, --version     print version

EXAMPLES:
    choomsay hello, choom
    choomsay -t \"i think, therefore i lint\"
    git log --oneline -1 | choomsay
";

/// The choom mascot — a friendly little terminal-dwelling bot.
const CHOOM: &str = r"   ___
  [⊙_⊙]
  <|=|>
   d b";

struct Options {
    width: usize,
    think: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            width: DEFAULT_WIDTH,
            think: false,
        }
    }
}

enum Parsed {
    Help,
    Version,
    Say {
        message: Option<String>,
        opts: Options,
    },
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let parsed = match parse(&args) {
        Ok(p) => p,
        Err(msg) => {
            eprintln!("choomsay: {msg}");
            eprintln!("try 'choomsay --help' for usage");
            std::process::exit(2);
        }
    };

    let (message, opts) = match parsed {
        Parsed::Help => {
            print!("{HELP}");
            return;
        }
        Parsed::Version => {
            println!("choomsay {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        Parsed::Say { message, opts } => (message, opts),
    };

    let message = match message.or_else(read_stdin) {
        Some(m) => m,
        None => {
            // nothing to say: be friendly and show usage
            print!("{HELP}");
            return;
        }
    };

    print!("{}", render(message.trim(), &opts));
}

fn parse(args: &[String]) -> Result<Parsed, String> {
    let mut opts = Options::default();
    let mut words: Vec<String> = Vec::new();
    let mut positional_only = false;
    let mut i = 0;
    while i < args.len() {
        let arg = args[i].as_str();
        if positional_only {
            words.push(arg.to_string());
            i += 1;
            continue;
        }
        match arg {
            "-h" | "--help" => return Ok(Parsed::Help),
            "-V" | "--version" => return Ok(Parsed::Version),
            "-t" | "--think" => opts.think = true,
            "--" => positional_only = true,
            "-w" | "--width" => {
                i += 1;
                let val = args.get(i).ok_or("--width needs a value")?;
                opts.width = parse_width(val)?;
            }
            other if other.starts_with("--width=") => {
                opts.width = parse_width(&other["--width=".len()..])?;
            }
            other if other.starts_with('-') && other.len() > 1 => {
                return Err(format!("unknown option '{other}'"));
            }
            other => words.push(other.to_string()),
        }
        i += 1;
    }

    let message = if words.is_empty() {
        None
    } else {
        Some(words.join(" "))
    };
    Ok(Parsed::Say { message, opts })
}

fn parse_width(s: &str) -> Result<usize, String> {
    let n: usize = s.parse().map_err(|_| format!("invalid width '{s}'"))?;
    if n == 0 {
        return Err("width must be at least 1".to_string());
    }
    Ok(n)
}

/// Read all of stdin, but only when it's piped (not an interactive terminal).
fn read_stdin() -> Option<String> {
    let stdin = io::stdin();
    if stdin.is_terminal() {
        return None;
    }
    let mut buf = String::new();
    match stdin.lock().read_to_string(&mut buf) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(buf),
    }
}

/// Word-wrap `text` to at most `width` columns (counted in characters).
/// Words longer than `width` are hard-broken so a line never overflows.
fn wrap(text: &str, width: usize) -> Vec<String> {
    let width = width.max(1);
    let mut lines: Vec<String> = Vec::new();
    let mut current = String::new();

    for raw_word in text.split_whitespace() {
        let mut word = raw_word;
        while count(word) > width {
            if !current.is_empty() {
                lines.push(std::mem::take(&mut current));
            }
            let (head, tail) = split_at_chars(word, width);
            lines.push(head.to_string());
            word = tail;
        }
        if word.is_empty() {
            continue;
        }
        if current.is_empty() {
            current.push_str(word);
        } else if count(&current) + 1 + count(word) <= width {
            current.push(' ');
            current.push_str(word);
        } else {
            lines.push(std::mem::take(&mut current));
            current.push_str(word);
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

/// Draw the wrapped `lines` inside a rounded bubble.
fn bubble(lines: &[String]) -> String {
    let inner = lines.iter().map(|l| count(l)).max().unwrap_or(0);
    let bar = "─".repeat(inner + 2);
    let mut out = format!("╭{bar}╮\n");
    for line in lines {
        let pad = " ".repeat(inner - count(line));
        out.push_str(&format!("│ {line}{pad} │\n"));
    }
    out.push_str(&format!("╰{bar}╯\n"));
    out
}

/// Render the full scene: bubble + connector + choom.
fn render(message: &str, opts: &Options) -> String {
    let lines = wrap(message, opts.width);
    let mut out = bubble(&lines);
    out.push_str(if opts.think {
        "      o\n     o\n"
    } else {
        "      \\\n       \\\n"
    });
    for line in CHOOM.lines() {
        out.push_str("        ");
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn count(s: &str) -> usize {
    s.chars().count()
}

fn split_at_chars(s: &str, n: usize) -> (&str, &str) {
    match s.char_indices().nth(n) {
        Some((idx, _)) => s.split_at(idx),
        None => (s, ""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_on_word_boundaries() {
        assert_eq!(
            wrap("the quick brown fox", 9),
            vec!["the quick", "brown fox"]
        );
    }

    #[test]
    fn keeps_short_text_on_one_line() {
        assert_eq!(wrap("hi choom", 40), vec!["hi choom"]);
    }

    #[test]
    fn hard_breaks_overlong_words() {
        assert_eq!(
            wrap("supercalifragilistic", 5),
            vec!["super", "calif", "ragil", "istic"]
        );
    }

    #[test]
    fn blank_text_yields_one_empty_line() {
        assert_eq!(wrap("   ", 10), vec![String::new()]);
    }

    #[test]
    fn never_exceeds_width() {
        let text = "choom vibes in the terminal, writing and shipping code all day";
        for w in 1..=20 {
            for line in wrap(text, w) {
                assert!(count(&line) <= w, "line {line:?} exceeds width {w}");
            }
        }
    }

    #[test]
    fn bubble_is_rectangular() {
        let out = bubble(&wrap("ab cde fghij", 4));
        let widths: Vec<usize> = out.lines().map(count).collect();
        assert!(
            widths.windows(2).all(|w| w[0] == w[1]),
            "uneven bubble: {widths:?}"
        );
    }

    #[test]
    fn bubble_frames_the_text() {
        assert_eq!(bubble(&["hi".to_string()]), "╭────╮\n│ hi │\n╰────╯\n");
    }

    #[test]
    fn render_includes_the_choom() {
        let out = render("yo", &Options::default());
        assert!(out.contains("[⊙_⊙]"), "missing mascot:\n{out}");
        assert!(out.contains('╭'), "missing bubble:\n{out}");
    }

    #[test]
    fn think_mode_has_no_speech_tail() {
        let out = render(
            "hmm",
            &Options {
                think: true,
                ..Options::default()
            },
        );
        assert!(
            out.contains('o'),
            "think bubble should have circles:\n{out}"
        );
        assert!(
            !out.contains('\\'),
            "think bubble should not have a tail:\n{out}"
        );
    }
}
