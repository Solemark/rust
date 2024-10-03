pub struct FizzBuzz {
    fizz: u32,
    buzz: u32,
    max: u32,
}

#[allow(dead_code)]
impl FizzBuzz {
    pub fn new(fizz: u32, buzz: u32, max: u32) -> FizzBuzz {
        FizzBuzz { fizz, buzz, max }
    }

    pub fn play(&self) -> String {
        let mut output = String::new();
        for i in 1..=self.max {
            output += &self.if_then_else(i % self.fizz == 0, "fizz".to_string());
            output += &self.if_then_else(i % self.buzz == 0, "buzz".to_string());
            output += &self.if_then_else(!output.ends_with('z'), i.to_string());
            output += &self.if_then_else(i != self.max, ", ".to_string());
        }
        output
    }

    fn if_then_else(&self, condition: bool, success: String) -> String {
        if condition {
            success
        } else {
            String::new()
        }
    }
}
