#![cfg(test)]

extern crate bincode;
#[macro_use]
extern crate serde_derive;

use bincode::Infinite;

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
enum Jkl {
    J,
    K,
    L,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
enum JklMno {
    J,
    K,
    L,
    M,
    N,
    O,
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
enum Abc<T> {
    A,
    B(i32),
    C(T),
}

#[repr(C)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
enum AbcXyz<T> {
    A,
    B(i32),
    C(T),
    X,
    Y(T),
    Z,
}

#[test]
fn direction_a() {
    let data = vec![
        Abc::C(Jkl::K),
        Abc::A,
        Abc::C(Jkl::J),
        Abc::B(5),
    ];

    let dump: Vec<u8> = bincode::serialize(&data, Infinite).unwrap();

    let data2: Vec<AbcXyz<JklMno>> = bincode::deserialize(&dump).unwrap();

    assert_eq!(
        data2,
        vec![
            AbcXyz::C(JklMno::K),
            AbcXyz::A,
            AbcXyz::C(JklMno::J),
            AbcXyz::B(5),
        ]
    );
}

#[test]
fn direction_b() {
    let data = vec![
        AbcXyz::C(Jkl::K),
        AbcXyz::A,
        AbcXyz::C(Jkl::J),
        AbcXyz::B(5),
    ];

    let dump: Vec<u8> = bincode::serialize(&data, Infinite).unwrap();

    let data2: Vec<Abc<JklMno>> = bincode::deserialize(&dump).unwrap();

    assert_eq!(
        data2,
        vec![
            Abc::C(JklMno::K),
            Abc::A,
            Abc::C(JklMno::J),
            Abc::B(5),
        ]
    );
}
