// could be usize, no idea;
// not important in this case, it's a toy project
pub type Id = i32;

#[derive(PartialEq, Eq, Debug)]
pub enum FieldVal {
    Num(i32),
    Str(String),
}
#[derive(PartialEq, Eq, Debug)]
pub struct Field {
    pub id: Id,
    pub value: FieldVal,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Record {
    pub id: Id,
    pub fields: Vec<Field>,
}
