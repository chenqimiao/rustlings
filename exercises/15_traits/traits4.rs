trait Licensed {
    fn licensing_info(&self) -> String {
        "Default license".to_string()
    }
}

struct SomeSoftware;
struct OtherSoftware;

impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}

// TODO: Fix the compiler error by only changing the signature of this function.
// dyn 其实多多态的一种语法，Box是指向堆上的指针，因为多态的表达会让编译器无法确定类型的大小，但作为函数参数（栈内）需要确定大小，所以需要使用Box来包裹动态类型。
fn compare_license_types(software1: Box<dyn Licensed>, software2: Box<dyn Licensed>) -> bool {
    software1.licensing_info() == software2.licensing_info()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_license_information() {
        assert!(compare_license_types(Box::new(SomeSoftware), Box::new(OtherSoftware)));
    }

    #[test]
    fn compare_license_information_backwards() {
        assert!(compare_license_types(Box::new(OtherSoftware), Box::new(SomeSoftware)));
    }
}
