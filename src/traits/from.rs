
pub trait FromAngleAxis<Angle, Axis> {
    fn angle_axis(angle: Angle, axis: Axis) -> Self;
}

pub trait FromFromTo<From, To> {
    ///Rotation `r` such that `r.apl(from) == to`.
    fn from_to(from: From, to: To) -> Self;
}
