use nom::character::complete::{alphanumeric1, alpha1, multispace1, one_of,line_ending,digit1};
use nom::character::{is_space,is_alphanumeric};
use nom::{branch::alt,combinator::opt,bytes::complete::{tag,take_while_m_n,is_not}, sequence::tuple, IResult, multi::many1};
use std::error::Error;
use std::io;
use std::fs;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

fn is_hex_digit(c: char) -> bool {
    return c.is_digit(16);
}

fn is_num(c: char) -> bool {
    return c.is_digit(10);
}


impl Passport {
    fn validate(&self) -> Result<(),Box<dyn Error + '_>> {

        let (_, byr) = digit1::<_,(_,_)>(self.byr.as_str())?;
        let byr = byr.parse::<usize>()?;
        println!("{}", byr);
        if !(byr >= 1920 && byr <= 2002) {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"invalid birth year")));
        }

        let (_, iyr) = digit1::<_,(_,_)>(self.iyr.as_str())?;
        let iyr = iyr.parse::<usize>()?;
        println!("{}", iyr);
        if !(iyr >= 2010 && iyr <= 2020) {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"invalid issue year")));
        }

        let (_, eyr) = digit1::<_,(_,_)>(self.eyr.as_str())?;
        let eyr = eyr.parse::<usize>()?;
        println!("{}", eyr);
        if !(eyr >= 2020 && eyr <= 2030) {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"invalid expiration")));
        }

        let (_, (hgt,unit)) = tuple::<_,(_,_),(_,_),(_,_)>
            ((digit1,alpha1))(self.hgt.as_str())?;
        let hgt = hgt.parse::<usize>()?;
        println!("{}:{}", hgt,unit);
        match unit {
            "in" if hgt >= 59 && hgt <= 76 => {},
            "cm" if hgt >= 150 && hgt <= 193 => {},
           _ =>  return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"invalid height"))),
        }

        let (rem, (_,hcl)) = tuple::<_,(_,_),(_,_),(_,_)>
            ((tag("#"),take_while_m_n(6,6, is_hex_digit)))(self.hcl.as_str())?;
        println!("{}", hcl);
        if rem != "" {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"invalid expiration")));
        }

        println!("{}", self.ecl);
        match self.ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {},
           _ =>  return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"invalid eye color"))),
        }

        let (rem, pid) = take_while_m_n::<_,_,(_,_)>(9,9, is_num)(self.pid.as_str())?;
        println!("{}", pid);
        if rem != "" {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"invalid expiration")));
        }
        return Ok(());
    }
}

fn kv(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, (k, _,v,_)) = tuple((alphanumeric1, tag(":"), is_not(" \n\r"),opt(one_of(" \n\r\0"))))(input)?;
    return Ok((input, (k, v)));
}

fn passport(input: &str) -> IResult<&str, Passport> {
    let (mut byr, mut iyr, mut eyr, mut hgt, mut hcl, mut ecl, mut pid, mut cid) = (String::from(""),String::from(""),String::from(""),String::from(""),String::from(""),String::from(""),String::from(""),String::from(""));
    let (input, (kv,_)) = tuple((many1(kv),opt(one_of("\r\n"))))(input)?;
    for (k,v) in kv {
        match k {
            "byr" => byr = String::from(v),
            "iyr" => iyr = String::from(v),
            "eyr" => eyr = String::from(v),
            "hgt" => hgt = String::from(v),
            "hcl" => hcl = String::from(v),
            "ecl" => ecl = String::from(v),
            "pid" => pid = String::from(v),
            "cid" => cid = String::from(v),
            _ => panic!("Unexpected input!"),
        }
    }
    return Ok((input, Passport{byr,iyr,eyr,hgt,hcl,ecl,pid,cid}))
}

fn passports(input: &str) -> IResult<&str, Vec<Passport>> {
    let (input, passports) = many1(passport)(input)?;
    return Ok((input,passports))
}

fn main() -> Result<(), Box<dyn Error>> {
    let raw = fs::read("./input.txt")?;
    let input = String::from_utf8(raw)?;
    //println!("{:#?}",passport("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm")?);
    //println!("{:#?}", kv("pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"));
    // AWW YEAH MEMORY LEAK TIME
    let s: &'static str = Box::leak(input.into_boxed_str());
    let (_, passports) = passports(s)?;
    let mut valid = 0;
    for passport in passports {
        println!("{:#?}",passport);
        if passport.validate().is_ok() {
            valid = valid + 1;
            println!("VALID");
        } else {
        println!("INVALID");
        }
    }
    println!("{}",valid);
    return Ok(());
}
