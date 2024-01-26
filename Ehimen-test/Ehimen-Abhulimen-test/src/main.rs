use std::io;
use std::io:write;

struct Leverage {
    shares:f64, liabilities:f64
}

impl Leverage {
    fn lev(&self) -> f64 {
        ((self.shares - self.liabilities) / self.shares) * 100.0
    }    
}

fn main(){

    let company_names = vec!["Nigerian breweries Plc", "Honeywell Nigeria Plc", "Unilever Nigeria Plc", "Nestle Nigeria Plc", "Flour Mills Nigeria Plc", "Dangote sugar Refinery Plc", "Champion Breweries Plc", "Cadbury Nigeria Plc"];
    let company_shares = vec![30_000_000, 34_000_000, 37_000_000, 8_000_000, 32_000_000, 18_000_000, 25_000_000, 15_000_000];
    let company_liabilities = vec![12_000_000, 9_000_000, 11_000_000, 1_500_000, 4_000_000, 10_000_000, 8_000_000, 5_500_000];

    //different company history databases
    println!("CONTINOUS ASSESSMENT PROJECT");
    println!("-------------------");
    println!("a list of company databases with their usernames");
    println!("Nigerian Breweries Plc - NIGE");
    println!("Honeywell Nigeria Plc - HONE");
    println!("Unilever Nigeria Plc - UNIL");
    println!("Nestle Nigeria Plc - NEST");
    println!("Flour Mills Nigeria - FLOU");
    println!("Dangote sugar refinery - DANG");
    println!("Champion Breweries Plc - CHAMP");
    println!("Cadbury Nigeria Plc - CADB");
    println!("");

    let mut input1 = String::new();
    println!("enter the username");
    io::stdin().read_line(&mut input1).expect("invalid");
    let username = input1.trim().parse().expect("invalid");

    let mut input2 = String::new();
    println!("enter the password");
    io::stdin().read_line(&mut input2).expect("invalid");
    let password = input2.trim().parse().expect("invalid");

    let usernames = vec!["NIGE", "HONE", "UNIL", "NEST", "FLOU", "DANG", "CHAMP", "CADB"];
    
    file.write_all("NIGERIAN BREWERIES PLC\n".as_bytes()).expect("invalid");
    file.write_all("Established date: 1946\n".as_bytes()).expect("invalid");
    file.write_all("Total Assets: 30,000,000\n".as_bytes()).expect("invalid");
    file.write_all("Total liabilities: 12,000,000\n".as_bytes()).expect("invalid");
    file.write_all("-----------------\n".as_bytes()).expect("invalid");
    //NIGERIAN BREWERIES PLC
    let percentage = Leverage {shares:shares[8],liabilities:liabilities[8]};
    file.write_all("percent of liability: ".as_bytes()).expect("invalid");
    file.write_all(percentage.lev().to string().as_bytes().expect("invalid");

     //HONEYWELL NIGERIA PLC
     file.write_all("HONEYWELL NIGERIA PLC\n".as_bytes()).expect("invalid");
     file.write_all("Established date: 1906\n".as_bytes()).expect("invalid");
     file.write_all("Total assets: 34_000_000\n".as_bytes()).expect("invalid");
     file.write_all("Total liabilities: 9_000_000\n".as_bytes()).expect("invalid");
     file.write_all("------------\n".as_bytes()).expect("invalid");
     let percentage = Leverage {shares:shares[7],liabilities:liabilities[7]};
     file.write_all("percent of liability: ".as_bytes()).expect("invalid");
     file.write_all(percentage.lev().to String().as_bytes().expect("invalid");

     //UNILEVER NIGERIA PLC
     file.write_all("UNILEVER NIGERIA PLC\n".as_bytes()).expect("invalid");
     file.write_all("Established date: 1923\n".as_bytes()).expect("invalid");
     file.write_all("Total assets: 37_000_000\n".as_bytes()).expect("invalid");
     file.write_all("Total liabilities: 11_000_000\n".as_bytes()).expect("invalid");
     file.write_all("------------\n".as_bytes()).expect("invalid");
     let percentage = Leverage {shares:shares[6],liabilities:liabilities[6]};
     file.write_all("percent of liability: ".as_bytes()).expect("invalid");
     file.write_all(percentage.lev().to string().as_bytes()).expect("invalid");

     //NESTLE NIGERIA PLC
     file.write_all("NESTLE NIGERIA PLC\n".as_bytes()).expect("invalid");
     file.write_all("Established date: 1961\n".as_bytes()).expect("invalid");
     file.write_all("Total assets: 8_000_000\n".as_bytes()).expect("invalid");
     file.write_all("Total liabilities: 1_500_000\n".as_bytes()).expect("invalid");
     file.write_all("----------\n".as_bytes()).expect("invalid");
     let percentage = Leverage {shares:shares[5],liabilities[5]};   
     file.write_all("percent of liability: ".as_bytes()).expect("invalid");
     file.write_all(percentage.lev().to string().as_bytes()).expect("invalid");

     //FLOUR MILLS NIGERIA PLC
     file.write_all("FLOUR MILLS NIGERIA PLC\n".as_bytes()).expect("invalid");
     file.write_all("Established date: 1960\n".as_bytes()).expect("invalid");
     file.write_all("Total assets: 32_000_000\n".as_bytes()).expect("invalid");
     file.write_all("Total liabilities: 4_000_000\n".as_bytes()).expect("invalid");
     file.write_all("---------\n".as_bytes()).expect("invalid");
     let percentage = Leverage {shares:shares[4],liabilities[4]};
     file.write_all("percent of liability: ".as_bytes()).expect("invalid");
     file.write_all(percentage.lev().to string().as_bytes()).expect("invalid");

     file.write_all("DANGOTE SUGAR REFINERY PLC\n".as_bytes()).expect("invalid");
     file.write_all("Established date: 1970\n".as_bytes()).expect("invalid");
     file.write_all("Total assets: 18_000_000\n".as_bytes()).expect("invalid");
     file.write_all("Total liabilities: 10_000_000\n".as_bytes()).expect("invalid");
     file.write_all("---------\n".as_bytes()).expect("invalid");
     let percentage = Leverage {shares:shares[3],liabilities[3]};
     file.write_all("percent of liability: ".as_bytes()).expect("invalid");
     file.write_all(percentage.lev().to string().as_bytes()).expect("invalid");
     //DANGOTE SUGAR REFINERY PLC
    
     file.write_all("CHAMPION BREWERIES PLC\n".as_bytes()).expect("invalid");
     file.write_all("Established date: 1974\n".as_bytes()).expect("invalid");
     file.write_all("Total assets: 25_000_000\n".as_bytes()).expect("invalid");
     file.write_all("Total liabilities: 8_000_000\n".as_bytes()).expect("invalid");
     file.write_all("---------\n".as_bytes()).expect("invalid");
     let percentage = Leverage {shares:shares[2],liabilities[2]};
     file.write_all("percent of liability: ".as_bytes()).expect("invalid");
     file.write_all(percentage.lev().to string().as_bytes()).expect("invalid");
     //CHAMPION BREWERIES PLC

     file.write_all("CADBURY NIGERIA PLC\n".as_bytes()).expect("invalid");
     file.write_all("Established date: 1965\n".as_bytes()).expect("invalid");
     file.write_all("Total assets: 15_000_000\n".as_bytes()).expect("invalid");
     file.write_all("Total liabilities: 5_500_000\n".as_bytes()).expect("invalid");
     file.write_all("---------\n".as_bytes()).expect("invalid");
     let percentage = Leverage {shares:shares[1],liabilities[1]};
     file.write_all("percent of liability: ".as_bytes()).expect("invalid");
     file.write_all(percentage.lev().to string().as_bytes()).expect("invalid");
     //CADBURY NIGERIA PLC
}






     
 