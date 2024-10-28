
pub trait Restricted<T>: Copy {
    fn restrict(value: T) -> Option<Self>;
    fn relax(self) -> T;
}