use core::ops::Index;
use qrcode::{Color, QrCode};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;

struct BlockMosaics {
    qr: Vec<Color>,
    d: usize,
}

impl BlockMosaics {
    pub fn new(qr: &QrCode) -> Self {
        let qr = qr.to_colors();
        let d = qr.len().isqrt() as usize;
        assert_eq!(d * d, qr.len());
        Self { qr, d }
    }

    pub fn to_section(&self, x: usize, y: usize) -> u8 {
        assert_eq!(x % 2, 0);
        assert_eq!(y % 3, 0);
        let mut m = 0;
        let mut k = 6;
        for j in y..y + 3 {
            for i in x..x + 2 {
                let mask = match self[(i, j)] {
                    Color::Dark => 1,
                    Color::Light => 0,
                };
                k -= 1;
                m |= mask << k;
            }
        }
        assert_eq!(k, 0);
        m
    }

    pub fn to_sections(&self) -> Vec<u8> {
        let mut sections = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                sections.push(self.to_section(x * 2, y * 3));
            }
        }
        return sections;
    }

    pub const fn width(&self) -> usize {
        self.d / 2 + 1
    }

    pub const fn height(&self) -> usize {
        self.d / 3 + 1
    }

    pub fn render<W: Write>(&self, mut f: W) {
        println!("Rendering for {d} x {d}", d = self.d);
        let sections = self.to_sections();
        let mut i = 0;
        for _ in 0..self.height() {
            for _ in 0..self.width() {
                write!(f, "{}", Self::from_section(sections[i])).unwrap();
                i += 1;
            }
            writeln!(f).unwrap()
        }
    }

    pub fn from_section(sections: u8) -> char {
        use Color::{Dark as i, Light as X};
        // 0	1	2	3	4	5	6	7	8	9	A	B	C	D	E	F
        match sections {
            0b000000 => ' ',
            0b100000 => '🬀',
            0b010000 => '🬁',
            0b110000 => '🬂',
            0b001000 => '🬃',
            0b101000 => '🬄',
            0b011000 => '🬅',
            0b111000 => '🬆',
            0b000100 => '🬇',
            //
            0b100100 => '🬈',
            0b010100 => '🬉',
            0b110100 => '🬊',
            0b001100 => '🬋',
            0b101100 => '🬌',
            0b011100 => '🬍',
            0b111100 => '🬎',
            //
            0b000010 => '🬏',
            0b100010 => '🬐',
            0b010010 => '🬑',
            0b110010 => '🬒',
            0b001010 => '🬓',
            0b101010 => '▌',
            0b011010 => '🬔',
            0b111010 => '🬕',
            0b000110 => '🬖',
            0b100110 => '🬗',
            0b010110 => '🬘',
            0b110110 => '🬙',
            0b001110 => '🬚',
            0b101110 => '🬛',
            0b011110 => '🬜',
            0b111110 => '🬝',

            0b000001 => '🬞',
            0b100001 => '🬟',
            0b010001 => '🬠',
            0b110001 => '🬡',
            0b001001 => '🬢',
            0b101001 => '🬣',
            0b011001 => '🬤',
            0b111001 => '🬥',
            0b000101 => '🬦',
            0b100101 => '🬧',
            0b010101 => '▐',
            0b110101 => '🬨',
            0b001101 => '🬩',
            0b101101 => '🬪',
            0b011101 => '🬫',
            0b111101 => '🬬',
            0b000011 => '🬭',
            0b100011 => '🬮',
            0b010011 => '🬯',
            0b110011 => '🬰',
            0b001011 => '🬱',
            0b101011 => '🬲',
            0b011011 => '🬳',
            0b111011 => '🬴',
            0b000111 => '🬵',
            0b100111 => '🬶',
            0b010111 => '🬷',
            0b110111 => '🬸',
            0b001111 => '🬹',
            0b101111 => '🬺',
            0b011111 => '🬻',
            0b111111 => '█',
            _ => ' ',
        }
    }
}

impl Index<(usize, usize)> for BlockMosaics {
    type Output = Color;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let d = self.d;
        if x < d && y < d { &self.qr[x + y * self.d] } else { &Color::Light }
    }
}

fn main() {
    let Some(target_file) = std::env::args().skip(1).next() else {
        println!("Please specify a target file");
        return;
    };
    let f = File::open(target_file).unwrap();
    let mut br = BufReader::new(f);

    const BUFSIZE: usize = 1600;
    let mut buffer = [0u8; BUFSIZE];
    for idx in 1.. {
        buffer.fill(0);
        let n = match br.read(&mut buffer) {
            Ok(0) | Err(_) => {
                break;
            }
            Ok(n) => n,
        };
        let qr = QrCode::new(&buffer[..n]).unwrap();
        let name = format!("{:0>3}.txt", idx);
        let mut file = File::create(name).unwrap();
        BlockMosaics::new(&qr).render(&mut file);
    }
}
