// could be usize, no idea;
// not important in this case, it's a toy project
pub type Id = i32;

#[derive(PartialEq, Eq, Debug)]
pub enum FieldVal<'a> {
    Num(i32),
    Str(&'a str),
}
#[derive(PartialEq, Eq, Debug)]
pub struct Field<'a> {
    pub id: Id,
    pub value: FieldVal<'a>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Record<'a> {
    pub id: Id,
    pub fields: Vec<Field<'a>>,
}
