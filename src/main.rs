use macro_crate::{Create};
#[derive(Create)]
struct Student{
    #[field(pk)]
    id:i32,
    #[field(length=40,null=false)]
    name:String,
    #[field(null=false,default=60.0)]
    score: f32,
    #[field(null=false,default=true)]
    is_job:bool,
}
fn main() {
    let create=Student::create_table_sql();
    println!("{}",create);
}
