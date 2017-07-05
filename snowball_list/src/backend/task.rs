use backend::date::Date;
#[allow(dead_code)]
pub struct Task
{
    sorting: u16,
    description: String,
    duration: u32,
    priority: Priority,
    repeat_every: u32,
    starting_from: Date,
    is_checked: bool,
    do_until: Date,
}

#[allow(dead_code)]
#[derive(Clone)] //Useful for borrow issues
pub enum Priority
{
    High,
    TryingToCare,
    Low,
    None
}


#[allow(dead_code)]
impl Task
{
    pub fn new(& mut self)
    {
        self.sorting=0;
        self.description=String::new();
        self.duration=0;
        self.priority=Priority::None;
        self.repeat_every=0;
        self.starting_from=Date::new();
        self.is_checked=false;
        self.do_until=Date::new();
    }

    pub fn set_sorting(& mut self, sorting: u16)
    {
        self.sorting=sorting;
    }
    pub fn get_sorting(&self)->u16
    {
        self.sorting
    }

    pub fn set_description(& mut self, description: String)
    {
        self.description=description;
    }
    pub fn get_description(&self)->String
    {
        self.description.clone()
    }

    pub fn set_duration(& mut self, duration: u32)
    {
        self.duration=duration;
    }
    pub fn get_duration(&self)->u32
    {
        self.duration
    }

    pub fn set_priority(& mut self, priority: Priority)
    {
        self.priority=priority;
    }
    pub fn get_priority(&self)->Priority
    {
        self.priority.clone()
    }

    pub fn set_repeat_every(& mut self, repeat_every: u32)
    {
        self.repeat_every=repeat_every;
    }
    pub fn get_repeat_every(&self)->u32
    {
        self.repeat_every
    }

    pub fn set_starting_from(& mut self, starting_from: Date)
    {
        self.starting_from=starting_from;
    }
    pub fn get_starting_from(&self)->Date
    {
        self.starting_from.clone()
    }

    pub fn set_is_checked(& mut self, is_checked: bool)
    {
        self.is_checked=is_checked;
    }
    pub fn get_is_checked(&self)->bool
    {
        self.is_checked
    }
}
