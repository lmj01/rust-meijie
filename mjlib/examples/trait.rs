
use mjlib::mjtrait;

fn main() {
    let a = mjtrait::dyn_trait::who2(1);
    a.print_name();

    mjtrait::obj_trait::test();

    mjtrait::kind_trait::test();
}