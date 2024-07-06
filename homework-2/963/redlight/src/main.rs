// enum LightTime {
//     R = 6,
//     Y = 1,
//     G = 3,
// }

// trait Process {
//     fn as_isize(self) -> isize;
//     fn process(v: isize) -> String {
//         format!("time:{}", v)
//     }
// }

// impl Process for LightTime {
//     fn as_isize(self) -> isize {
//         self as isize
//     }

//     fn process(v: isize) -> String {
//         format!("t:{}", v)
//     }
// }

#[allow(dead_code)]
enum TrLight {
    RED,
    YELLOW,
    GREEN,
}

trait WaitTime {
    fn wait(&self) -> u8;
}
impl WaitTime for TrLight {
    fn wait(&self) -> u8 {
        match self {
            Self::RED => 6,
            Self::GREEN => 4,
            Self::YELLOW => 1,
        }
    }
}

fn main() {
    let red = TrLight::RED;
    assert_eq!(6, red.wait());
    println!("wait GREEN time {}", TrLight::GREEN.wait())
}

#[test]
fn test_light() {
    let red = TrLight::RED;
    assert_eq!(6, red.wait());
    assert_eq!(1, TrLight::YELLOW.wait());
    assert_eq!(4, TrLight::GREEN.wait());
}
