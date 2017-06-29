use date::Date;
#[allow(dead_code)]
pub struct Task
{
    sorting: u16,
    description: String,
    duration: u32,
    priority: String, //high, trying_to_care, low
    repeat_every: u32,
    starting_from: Date,
    is_checked: bool,
}

#[allow(dead_code)]
impl Task
{
    pub fn new(& mut self)
    {
        self.sorting=0;
        self.description=String::new();
        self.duration=0;
        self.priority=String::new();
        self.repeat_every=0;
        self.starting_from=Date::new();
        self.is_checked=false;
    }

    pub fn set_sorting(& mut self, sorting: u16)
    {
        self.sorting=sorting;
    }
    pub fn get_sorting(& mut self)->u16
    {
        self.sorting
    }

    pub fn set_description(& mut self, description: String)
    {
        self.description=description;
    }
    pub fn get_description(& mut self)->String
    {
        self.description.clone()
    }

    pub fn set_duration(& mut self, duration: u32)
    {
        self.duration=duration;
    }
    pub fn get_duration(& mut self)->u32
    {
        self.duration
    }

    pub fn set_priority(& mut self, priority: String)
    {
        self.priority=priority;
    }
    pub fn get_priority(& mut self)->String
    {
        self.priority.clone()
    }

    pub fn set_repeat_every(& mut self, repeat_every: u32)
    {
        self.repeat_every=repeat_every;
    }
    pub fn get_repeat_every(& mut self)->u32
    {
        self.repeat_every
    }

    // starting_from: Date,
    // is_checked: bool,
    pub fn set_starting_from(& mut self, starting_from: Date)
    {
        self.starting_from=starting_from;
    }
    pub fn get_starting_from(& mut self)->Date
    {
        //self.starting_from.new()
        Date::new()
    }

    pub fn set_is_checked(& mut self, is_checked: bool)
    {
        self.is_checked=is_checked;
    }
    pub fn get_is_checked(& mut self)->bool
    {
        self.is_checked
    }
}
