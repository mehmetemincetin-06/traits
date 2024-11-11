fn main() {
    let mut database: Vec<Account>=Vec::new();


    let acount:Account=Account{
        tc:32452525,
        iban:3425252452,
        name:"Mehmet".to_string(),
        hesap_no:"müşteri".to_string(),
        amaount:1000.0


    };
    database.push(acount.clone());
    database[0].show_amaount();
    database[0].deposit(1000.0);
    database[0].withdraw(500.0);
    
}
#[derive(Debug,Clone)]
struct Account{
    tc:u32,
    iban:u32,
    name:String,
    hesap_no:String,
    amaount:f32
}
enum  Donate{

    military
}
trait  Account_functions{
    fn deposit(&mut self,amaount:f32);
    fn withdraw(&mut self,amaount:f32);
    fn show_amaount(&mut self);
    
    
}
impl Account_functions for Account {
    fn deposit(&mut self,amaount:f32) {
        self.amaount =self.amaount+amaount;
        println!("Başarıyla yükleme yapıldı yeni bakiyeniz{}",self.amaount)
    }
    fn withdraw(&mut self,amaount:f32) {
        if amaount>=self.amaount{
            println!("Hesap bakiyeniz yetersiz hesabınızdaki para{}",self.amaount)
        }
        else {
            self.amaount=self.amaount-amaount;
            println!("Para başarılı bir şekilde çekildi yeni bakiyeniz{}",self.amaount)
        }
    }
    fn show_amaount(&mut self) {
        println!("Hesap bakiyeniz{}",self.amaount);
    }
    
}
