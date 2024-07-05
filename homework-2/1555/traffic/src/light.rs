pub trait Light {
    fn time(&self) -> i32;
}

pub struct Red {
    //pub name: String,
}

pub struct Green {
    //pub time: i32,
}

pub struct Yellow {
    //pub time: i32,
}

impl Light for Red {
    fn time(&self) -> i32 {
        let at=60;
        at
    }
}

impl Light for Green {
    fn time(&self) -> i32 {
        let at=50;
        at
    }
}

impl Light for Yellow {
    fn time(&self) -> i32 {
        let at=3;
        at
    }
}