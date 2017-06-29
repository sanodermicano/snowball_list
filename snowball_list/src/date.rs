#[derive(Clone)] //Useful for borrow issues
pub struct Date
{
    date: String
}

#[allow(dead_code)]
impl Date
{
    pub fn new() -> Date
    {
        Date
        {
            date: String::new()
        }
    }
}
