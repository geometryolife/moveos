// module rooch_examples
module rooch_examples::test {
    // Test1:
    // struct Data {}
    //
    // struct Box has drop {
    //     value: u64
    // }
    //
    // fun new_box(v: u64) {
    //     Box { value: v };
    // }

    // Test2:
    // struct Data has drop {}
    //
    // struct Box<T> has drop {
    //     value: T
    // }
    //
    // fun new_box(v: Data) {
    //     Box { value: v };
    // }

    // Test3:
    // struct Data {}
    //
    // struct Box<T> has key {
    //     value: T
    // }
    //
    // fun new_box<T>(v: Data): Box<Data> {
    //     Box { value: v }
    // }

    // Test4:
    // Define a Data type
    struct Data has copy, drop {
        v: u64
    }

    // Define a Box type
    struct Box<T> has drop {
        v: T
    }

    // Create an instance of type Data
    fun new_data(value: u64): Data {
        Data { v: value }
    }

    // Create an instance of type Box<T>
    public fun new_box<T>(value: T): Box<T> {
        Box { v: value }
    }

    // Get the value inside the Box<T>
    public fun get_box_value<T: copy>(box: &Box<T>): T {
        box.v
    }

    // Unit Test:
    #[test]
    // #[expected_failure]
    fun test() {
        let data = new_data(123);
        let box = new_box<Data>(data);
        assert!(get_box_value<Data>(&box).v == 123, 1000);
        // assert!(get_box_value(&box) == Data{v:1234}, 10001)
    }
}

// module rooch_examples::test2 {
//     use rooch_examples::test::{new_box, get_box_value};
//
//     const NOT_EQUAL: u64 = 1;
//
//     struct Data2 has copy, drop {
//         v: u64
//     }
//
//     #[test]
//     #[expected_failure]
//     fun run() {
//         let data2 = Data2 { v: 789 };
//         let box = new_box<Data2>(data2);
//         assert!(get_box_value(&box).v == 7891, NOT_EQUAL);
//     }
// }