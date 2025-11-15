#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // scenario_0();
    // scenario_1();
    // scenario_2();
    scenario_3();
}

fn scenario_3(){
    let [j1, j2] = &mut [Jump { value: 100 }, Jump { value: 50 }];

    let j1 = j1 as *mut Jump;
    let j2: *mut Jump = j2 as *mut Jump;

    println!("j1 address: {:?}, j2 address: {:?}", j1, j2);

    let [e1, e2] = &mut [Entity, Entity];

    let e1 = e1 as *mut Entity;
    let e2 = e2 as *mut Entity;

    println!("e1 address: {:?}, e2 address: {:?}", e1, e2);
}

fn scenario_2() {
    let [j1, j2] = &mut [Jump { value: 100 }, Jump { value: 50 }];

    let j1 = j1 as *mut Jump;
    let j2: *mut Jump = j2 as *mut Jump;

    println!("Size of Jump Array is {}", std::mem::size_of::<[Jump; 2]>());
    println!("Size of Jump struct is {}", std::mem::size_of::<Jump>());
    println!("j1 address: {:?}, j2 address: {:?}", j1, j2);

    let [e1, e2] = &mut [Entity, Entity];

    let e1 = e1 as *mut Entity;
    let e2 = e2 as *mut Entity;

    println!();
    println!(
        "Size of Entity Array is {}",
        std::mem::size_of::<[Entity; 2]>()
    );
    println!("Size of Entity struct is {}", std::mem::size_of::<Entity>());
    println!("e1 address: {:?}, e2 address: {:?}", e1, e2);
}

struct Entity;

fn scenario_1() {
    let mut jump = Jump { value: 100 };

    let ref_1 = &mut jump as *mut Jump;
    let ref_2 = &mut jump as *mut Jump;

    unsafe {
        println!("ref 1 address: {:?}, ref 2 address: {:?}", ref_1, ref_2);
        (*ref_2).value = 90;
        (*ref_1).value = 75;
        println!(
            "ref_1.value: {}, ref_2.value: {}",
            (*ref_1).value,
            (*ref_2).value
        );
    }
}

fn scenario_0() {
    let mut jump = Jump { value: 100 };

    let ref_1 = &mut jump;
    // cannot borrow `jump` as mutable more than once at a time...
    let ref_2 = &mut jump;

    // (*ref_1).value = 90; // Hatayı almak için yorum satırını açın
}

struct Jump {
    value: u32,
}
