// run-pass
// edition:2021

struct Props {
    field_1: u32, //~ WARNING: field is never read: `field_1`
    field_2: u32, //~ WARNING: field is never read: `field_2`
}

fn main() {
    // Test 1
    let props_2 = Props { //~ WARNING: unused variable: `props_2`
        field_1: 1,
        field_2: 1,
    };

    let _ = || {
        let _: Props = props_2;
    };

    // Test 2
    let mut arr = [1, 3, 4, 5];

    let mref = &mut arr;

    let _c = || match arr {
        [_, _, _, _] => println!("A")
    };

    println!("{:#?}", mref);
}
