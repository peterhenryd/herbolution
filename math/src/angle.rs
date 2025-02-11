pub trait Angle<T> {
    fn into_deg(self) -> Deg<T>;

    fn into_rad(self) -> Rad<T>;
}

#[derive(derive::Scalar, serde::Deserialize, serde::Serialize)]
pub struct Deg<T>(pub T);

impl<T: num::Float> Angle<T> for Deg<T> {
    fn into_deg(self) -> Deg<T> {
        self
    }

    fn into_rad(self) -> Rad<T> {
        Rad(self.0.to_radians())
    }
}

impl<T: num::Float> From<Rad<T>> for Deg<T> {
    fn from(rad: Rad<T>) -> Self {
        rad.into_deg()
    }
}

impl<T> From<T> for Deg<T> {
    fn from(value: T) -> Self {
        Deg(value)
    }
}

#[derive(derive::Scalar, serde::Deserialize, serde::Serialize)]
pub struct Rad<T>(pub T);

impl<T: num::Float> Angle<T> for Rad<T> {
    fn into_deg(self) -> Deg<T> {
        Deg(self.0.to_degrees())
    }

    fn into_rad(self) -> Rad<T> {
        self
    }
}

impl<T: num::Float> From<Deg<T>> for Rad<T> {
    fn from(deg: Deg<T>) -> Self {
        deg.into_rad()
    }
}

impl<T> From<T> for Rad<T> {
    fn from(value: T) -> Self {
        Rad(value)
    }
}