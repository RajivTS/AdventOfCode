fn main() {
    let results = include_str!("input.txt")
        .split("\n\n")
        .map(|input| PassportBuilder::parse(input).and_then(|b| b.build()));
    let num_valid = results.filter(Result::is_ok).count();
    println!("{} passport records were valid", num_valid);
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Year(u64);

#[derive(Debug, Clone, Copy, PartialEq)]
enum Length {
    Cm(u64),
    In(u64),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Color<'a>(&'a str);

#[derive(Debug, Clone, Copy, PartialEq)]
struct ID<'a>(&'a str);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Passport<'a> {
    birth_year: Year,
    issue_year: Year,
    expiration_year: Year,
    height: Length,
    hair_color: Color<'a>,
    eye_color: Color<'a>,
    passport_id: ID<'a>,
    country_id: Option<ID<'a>>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
struct PassportBuilder<'a> {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Length>,
    hair_color: Option<Color<'a>>,
    eye_color: Option<Color<'a>>,
    passport_id: Option<ID<'a>>,
    country_id: Option<ID<'a>>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("missing field: {0}")]
    MissingField(&'static str),

    #[error("could not parse {0}: {1}")]
    ParseError(String, String),
}

impl<'a> PassportBuilder<'a> {
    fn build(self) -> Result<Passport<'a>, Error> {
        Ok(Passport {
            birth_year: self.birth_year.ok_or(Error::MissingField("birth_year"))?,
            issue_year: self.issue_year.ok_or(Error::MissingField("issue year"))?,
            expiration_year: self
                .expiration_year
                .ok_or(Error::MissingField("expiration_year"))?,
            height: self.height.ok_or(Error::MissingField("height"))?,
            hair_color: self.hair_color.ok_or(Error::MissingField("hair color"))?,
            eye_color: self.eye_color.ok_or(Error::MissingField("eye_color"))?,
            passport_id: self.passport_id.ok_or(Error::MissingField("passport id"))?,
            country_id: self.country_id,
        })
    }

    fn parse(input: &'a str) -> Result<Self,Error> {
        let mut b: Self = Default::default();

        peg::parser! {
            grammar parser() for str {
                pub(crate) rule root(b: &mut PassportBuilder<'input>)
                    = (field(b) separator()*)* ![_]

                rule separator()
                    = ['\n' | ' ']

                rule field(b: &mut PassportBuilder<'input>)
                    // years
                    = byr(b) / iyr(b) / eyr(b)
                    // height
                    / hgt(b)
                    // colors
                    / hcl(b) / ecl(b)
                    // IDs
                    / pid(b) / cid(b)

                rule byr(b: &mut PassportBuilder<'input>)
                    = "byr:" year:year((1920..=2002)) { b.birth_year = Some(year) }

                rule iyr(b: &mut PassportBuilder<'input>)
                    = "iyr:" year:year((2010..=2020)) { b.issue_year = Some(year) }

                rule eyr(b: &mut PassportBuilder<'input>)
                    = "eyr:" year:year((2020..=2030)) { b.expiration_year = Some(year) }

                rule hgt(b: &mut PassportBuilder<'input>)
                    = "hgt:" height:length() {?
                        match &height {
                            Length::Cm(v) if !(150..=193).contains(v) => {
                                Err("bad height (cm)")
                            },
                            Length::In(v) if !(59..=76).contains(v) => {
                                Err("bad height (in)")
                            },
                            _ => {
                                b.height = Some(height);
                                Ok(())
                            },
                        }
                    }

                rule pid(b: &mut PassportBuilder<'input>)
                    = "pid:" id:$(['0'..='9']*<9,9>) { b.passport_id = Some(ID(id)) }

                rule cid(b: &mut PassportBuilder<'input>)
                    = "cid:" id:$((!separator()[_])+) { b.country_id = Some(ID(id)) }

                rule hcl(b: &mut PassportBuilder<'input>)
                    = "hcl:" color:$("#" ['0'..='9' | 'a'..='f']*<6,6>) { b.hair_color = Some(Color(color)) }

                rule ecl(b: &mut PassportBuilder<'input>)
                    = "ecl:" color:$("amb" / "blu" / "brn" / "gry" / "grn" / "hzl" / "oth") { b.eye_color = Some(Color(color)) }

                rule year(range: std::ops::RangeInclusive<u64>) -> Year
                    = num:num() {?
                        if range.contains(&num) {
                            Ok(Year(num))
                        } else {
                            Err("year out of range")
                        }
                    }

                rule color() -> Color<'input>
                    = s:$((!separator()[_])*) { Color(s) }

                rule length() -> Length
                    = num:num() "cm" { Length::Cm(num) }
                    / num:num() "in" { Length::In(num) }

                rule num() -> u64
                    = s:$(['0'..='9']+) { s.parse().unwrap() }

                rule id() -> ID<'input>
                    = s:$(['0'..='9' | 'a'..='z' | '#']+) { ID(s) }
            }
        }

        parser::root(input, &mut b).map_err(|e| Error::ParseError(input.into(), e.to_string()))?;
        Ok(b)
    }
}
