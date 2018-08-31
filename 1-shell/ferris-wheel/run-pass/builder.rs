// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn string(mut self, s: &str) -> Builder {
        self.string = Some(s.clone().to_string());
        return self
    }

    fn number(mut self, n: usize) -> Builder {
        self.number = Some(n);
        return self
    }
}

impl Default for Builder {
    fn default() -> Builder {
        Builder {
            string: None,
            number: None,
        }
    }
}

impl ToString for Builder {
    fn to_string(&self) -> String {
        let mut result = String::new();
        if let Some(ref s) = self.string {
            result.push_str(s);
            if self.number.is_some(){
                result.push_str(&" ");
            }
        }
        if let Some(i) = self.number {
            result.push_str(&i.to_string());
        }
        result
    }
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string(&"heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");
}
