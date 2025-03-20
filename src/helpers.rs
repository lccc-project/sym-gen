pub trait TyOrDefault {
    type Type;
}

impl<T> TyOrDefault for (T,) {
    type Type = T;
}

impl<T, U> TyOrDefault for (T, U) {
    type Type = U;
}
