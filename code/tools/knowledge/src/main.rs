struct Thing {
    name: &'static str,
    abbrev: Option<&'static str>,
    desc: Vec<&'static str>,
}

impl Thing {
    pub const fn new(name: &'static str) -> Self {
        Thing { name, abbrev: None, desc: vec![] }
    }

    pub const fn abbrev(mut self, abbrev: &'static str) -> Self {
        self.abbrev = Some(abbrev);
        self
    }

    pub fn desc(mut self, desc: &'static str) -> Self {
        self.desc.push(desc);
        self
    }
}

const fn t(name: &'static str) -> Thing {
    Thing::new(name)
}

//     Thing::new("EMVCo").desc(&["\
// EMV is a payment method based on a technical standard for smart payment cards
// and for payment terminals and automated teller machines which can accept them.",
// "EMV stands for \"Europay, Mastercard, and Visa\", the three companies that created the standard."
// ,"EMV is managed by EMVCo, a financial technology consortium owned by card scheme companies."]),
//     Thing::new("Radar")];

const THINGS: &[Thing] = &[Thing::new("X-PAY-TOKEN")];

struct Wrap {
    width: usize,
    buffer: Vec<String>,
}

impl Wrap {
    pub fn new(width: usize) -> Self {
        Self { width, buffer: vec![] }
    }

    pub fn wrap(&mut self, text: &str) -> &[String] {
        let mut j = 0;
        if self.buffer.is_empty() {
            self.buffer.push(String::new())
        }
        let mut line = self.buffer.get_mut(0).unwrap();
        for word in text.split_ascii_whitespace() {
            if line.len() + word.len() + 1 > self.width {
                j += 1;
                line = match self.buffer.get_mut(j) {
                    Some(v) => {
                        v.clear();
                        v.push_str(word);
                        v
                    }
                    None => {
                        self.buffer.push(word.to_string());
                        self.buffer.last_mut().unwrap()
                    }
                };
            } else {
                if !line.is_empty() {
                    line.push(' ');
                }
                line.push_str(word)
            }
        }
        &self.buffer[..=j]
    }
}

fn wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = vec![];
    for word in text.split_ascii_whitespace() {
        let Some(line) = lines.last_mut() else {
            lines.push(word.to_string());
            continue;
        };
        if line.len() + word.len() + 1 > width {
            lines.push(word.to_string());
        } else {
            line.push(' ');
            line.push_str(word)
        }
    }
    lines
}

fn all_things() -> Vec<Thing> {
    vec![Thing::new("EMVCo").desc(
        "\
EMV is a payment method based on a technical standard for smart payment cards
and for payment terminals and automated teller machines which can accept them.",
    )]
}

fn main() {
    let mut wrap = Wrap::new(60);
    for thing in all_things() {
        println!();
        print!("\x1b[32m{}\x1b[m", thing.name);
        if let Some(abbrev) = thing.abbrev {
            print!(" \x1b[37m({})\x1b[m", abbrev);
        }
        println!();
        println!();

        // for line in wrap.wrap(thing.desc)
        for desc_line in thing.desc {
            for (idx, subline) in wrap.wrap(desc_line).iter().enumerate() {
                if idx == 0 { println!("* {subline}") } else { println!("  {subline}") }
            }
        }
    }
    // thing("X-PAY-TOKEN").abbrev("hey");

    println!("Hello, world!");
}
