mod foo;

mod foo::Bar;

fn main() {
    foo::do_foo();
    Bar::hello();
}

