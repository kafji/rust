// run-rustfix

#![deny(disjoint_capture_drop_reorder)]

#[derive(Debug)]
struct Foo(i32);
impl Drop for Foo {
    fn drop(&mut self) {
        println!("{:?} dropped", self.0);
    }
}

struct ConstainsDropField(Foo, Foo);

// Test that lint is triggered if a path that implements Drop is not captured by move
fn test_precise_analysis_drop_paths_not_captured_by_move() {
    let t = ConstainsDropField(Foo(10), Foo(20));

    let c = || {
    //~^ ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| HELP: add a dummy let to cause `t` to be fully captured
        let _t = t.0;
        let _t = &t.1;
    };

    c();
}

struct S;
impl Drop for S {
    fn drop(&mut self) {
    }
}

struct T(S, S);
struct U(T, T);

// Test precise analysis for the lint works with paths longer than one.
fn test_precise_analysis_long_path_missing() {
    let u = U(T(S, S), T(S, S));

    let c = || {
    //~^ ERROR: drop order affected for closure because of `capture_disjoint_fields`
    //~| HELP: add a dummy let to cause `u` to be fully captured
        let _x = u.0.0;
        let _x = u.0.1;
        let _x = u.1.0;
    };

    c();
}

fn main() {
    test_precise_analysis_drop_paths_not_captured_by_move();
    test_precise_analysis_long_path_missing();
}
