use std::{vec, f32::consts::PI};

/*
 * @Author: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.&com
 * @Date: 2022-07-16 17:12:16
 * @LastEditors: Gxp-Ning 77679755+Gxp-Ning@users.noreply.github.com
 * @LastEditTime: 2022-07-17 00:46:23
 * @FilePath: \course4\src\main.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use random_number::random;

fn main() {
    //第一题
    random_color();
    //第二题
    /* 定义一个容量为70000的vec，每个值的范围在0..70000
    大概率不会溢出，但会存在这种情况 */
    let set = vec![0; 70000];
    let sum = sum_of_set(set);    
    match sum {
        Some(num) => {
            println!("和={}", num)
        },
        None => {
            println!("None")
        }
    }
    //第三题
    //通过随机数控制图形类型
    let rand = random!(1..=2);
    if rand == 1 {
        let grap = Circle {
            radius: 3.0
        };
        println!("面积={}", area(grap));
    } else {
        let grap = Square {
            side: 4.0
        };
        println!("面积={}", area(grap));
    }
}
/*****************第一题************************

 利用random随机在012中随机生成一个数字 matchTrafficLight中的相应颜色
    存在问题：在match时，似乎时按照i32进行的，导致需要一个"_" */
    
//交通灯
pub  enum TrafficLight {
    Red,
    Yellow,
    Green,
}



fn random_color() {
    let color :u8 = random!(..=2);
    let light = 
    match color {
        0 => TrafficLight::Red,
        1 => TrafficLight::Green,
        2 => TrafficLight::Yellow,  
        _ => TrafficLight::Green,
    };
    println!("交通灯时间={:?}", light.time());
}

impl TrafficLight {
    pub fn time (&self) -> u8 {
            match self {
                Self::Red => 60,
                Self::Yellow => 50,
                Self::Green => 40,
            }
        }
}

/******************第二题************************/

fn sum_of_set(mut set :Vec<u32>) -> Option<u32>{
    /* 定义一个容量为70000的vec，每个值的范围在0..70000
    大概率不会溢出，但会存在这种情况 */
    let mut len = set.len();
    let mut sum = 0;
    let max = u32::MAX;
    let min = u32::MIN;
    while len > 0 {
        set[len-1] = random!(..=70000);
        len -= 1;
    }
    for i in set.iter() {
        sum += i;
    }
    if sum >= max || sum <= min {
        None
    } else {
        Some(sum)
    }
    
}

/******************第三题************************/

pub struct  Square {
    side :f32,
}

pub struct Circle {
    radius :f32,
}

pub trait Area {
    fn graphic_area(&self) -> f32;
}

impl Area for Square {
    fn graphic_area(&self) -> f32 {
        self.side * self.side
    }
}

impl Area for Circle {
    fn graphic_area(&self) -> f32 {
        self.radius * self .radius * PI
    }
}

fn area<T: Area>(graphic :T) -> f32 {
        graphic.graphic_area()
}