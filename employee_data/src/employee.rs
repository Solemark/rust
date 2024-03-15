#[derive(Debug)]
pub struct Employee {
    pub(crate) name: String,
    pub(crate) phone: String,
    pub(crate) email: String,
    pub(crate) rate: f32,
}
pub trait EmpFunctions {
    fn get_name(&self) -> String;
    fn set_name(&mut self, name: String);
    fn get_phone(&self) -> String;
    fn set_phone(&mut self, phone: String);
    fn get_email(&self) -> String;
    fn set_email(&mut self, email: String);
    fn get_rate(&self) -> f32;
    fn set_rate(&mut self, rate: f32);
}
impl EmpFunctions for Employee {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn get_phone(&self) -> String {
        self.phone.clone()
    }

    fn set_phone(&mut self, phone: String) {
        self.phone = phone;
    }

    fn get_email(&self) -> String {
        self.email.clone()
    }

    fn set_email(&mut self, email: String) {
        self.email = email;
    }

    fn get_rate(&self) -> f32 {
        self.rate
    }

    fn set_rate(&mut self, rate: f32) {
        self.rate = rate;
    }
}
