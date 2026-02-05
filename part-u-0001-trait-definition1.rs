fn main() {

}

trait Company {
    
    fn get_company_info(&self) -> String;

    fn set_department(&mut self, name: &'static str, num_of_emp: u8);
}
