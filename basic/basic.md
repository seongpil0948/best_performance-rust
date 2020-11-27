* __Crate__ is set of source file
    - Libary Crate, Binary Crate(my project)

* Cargo.lock
    - make when initial excuted $ __cargo build__
    - and cargo check __Cargo.lock__ use version in lock when every build 

* $ __cargo doc__ is open Official Doc
    - __--open__ = Open th Doc of Lib you are using

* int type
    - u8 = unsigned = 부호 없는 = 8bit
    - i8 = signed = 8bit = 2의 보수로 표현 = 앞에 1비트 추가하는 거였나? 
        {-(2 ** n-1)} ~ {2 ** (n-1) -1 } = -128 ~ 127 까지의 값을 보관
    - isize/usize 는 64bit 컴퓨터면 64bit 32bit 면 32bit

* func
    - not attach semicolon after return val, 
        rust think construction(구문) it

* if state 
    - must return a bool
    - both if, else return same type

* loop
    - return in loop  = break variable1 * 2;
    - for i in a.iter() {}  // a is arr