macro_rules! impl_ops {
    (
        struct $name:ident<$t:ident> {
            $($field:ident: $ft:ident),+ $(,)?
        }
    ) => {
          impl<$t: Add<Output = $t>> Add for $name<$t> {
            type Output = Self;

            #[inline]
            fn add(self, other: Self) -> Self {
                Self {
                    $($field: self.$field + other.$field),+
                }
            }
        }

        impl<$t: Add<Output = $t> + Copy> Add<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn add(self, other: $t) -> Self {
                Self {
                    $($field: self.$field + other),+
                }
            }
        }

        impl<$t: AddAssign> AddAssign for $name<$t> {
            #[inline]
            fn add_assign(&mut self, other: Self) {
                $(self.$field += other.$field;)+
            }
        }

        impl<$t: AddAssign + Copy> AddAssign<$t> for $name<$t> {
            #[inline]
            fn add_assign(&mut self, other: $t) {
                $(self.$field += other;)+
            }
        }

        impl<$t: Sub<Output = $t>> Sub for $name<$t> {
            type Output = Self;

            #[inline]
            fn sub(self, other: Self) -> Self {
                Self {
                    $($field: self.$field - other.$field),+
                }
            }
        }

        impl<$t: Sub<Output = $t> + Copy> Sub<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn sub(self, other: $t) -> Self {
                Self {
                    $($field: self.$field - other),+
                }
            }
        }

        impl<$t: SubAssign> SubAssign for $name<$t> {
            #[inline]
            fn sub_assign(&mut self, other: Self) {
                $(self.$field -= other.$field;)+
            }
        }

        impl<$t: SubAssign + Copy> SubAssign<$t> for $name<$t> {
            #[inline]
            fn sub_assign(&mut self, other: $t) {
                $(self.$field -= other;)+
            }
        }

        impl<$t: Mul<Output = $t>> Mul for $name<$t> {
            type Output = Self;

            #[inline]
            fn mul(self, other: Self) -> Self {
                Self {
                    $($field: self.$field * other.$field),+
                }
            }
        }

        impl<$t: Mul<Output = $t> + Copy> Mul<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn mul(self, other: $t) -> Self {
                Self {
                    $($field: self.$field * other),+
                }
            }
        }

        impl<$t: MulAssign> MulAssign for $name<$t> {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                $(self.$field *= other.$field;)+
            }
        }

        impl<$t: MulAssign + Copy> MulAssign<$t> for $name<$t> {
            #[inline]
            fn mul_assign(&mut self, other: $t) {
                $(self.$field *= other;)+
            }
        }

        impl<$t: Div<Output = $t>> Div for $name<$t> {
            type Output = Self;

            #[inline]
            fn div(self, other: Self) -> Self {
                Self {
                    $($field: self.$field / other.$field),+
                }
            }
        }

        impl<$t: Div<Output = $t> + Copy> Div<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn div(self, other: $t) -> Self {
                Self {
                    $($field: self.$field / other),+
                }
            }
        }

        impl<$t: DivAssign> DivAssign for $name<$t> {
            #[inline]
            fn div_assign(&mut self, other: Self) {
                $(self.$field /= other.$field;)+
            }
        }

        impl<$t: DivAssign + Copy> DivAssign<$t> for $name<$t> {
            fn div_assign(&mut self, other: $t) {
                $(self.$field /= other;)+
            }
        }

        impl<$t: Rem<Output = $t>> Rem for $name<$t> {
            type Output = Self;

            #[inline]
            fn rem(self, other: Self) -> Self {
                Self {
                    $($field: self.$field % other.$field),+
                }
            }
        }

        impl<$t: Rem<Output = $t> + Copy> Rem<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn rem(self, other: $t) -> Self {
                Self {
                    $($field: self.$field % other),+
                }
            }
        }

        impl<$t: RemAssign> RemAssign for $name<$t> {
            #[inline]
            fn rem_assign(&mut self, other: Self) {
                $(self.$field %= other.$field;)+
            }
        }

        impl<$t: RemAssign + Copy> RemAssign<$t> for $name<$t> {
            #[inline]
            fn rem_assign(&mut self, other: $t) {
                $(self.$field %= other;)+
            }
        }

        impl<$t: BitAnd<Output = $t>> BitAnd for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitand(self, other: Self) -> Self {
                Self {
                    $($field: self.$field & other.$field),+
                }
            }
        }

        impl<$t: BitAnd<Output = $t> + Copy> BitAnd<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitand(self, other: $t) -> Self {
                Self {
                    $($field: self.$field & other),+
                }
            }
        }

        impl<$t: BitAndAssign> BitAndAssign for $name<$t> {
            #[inline]
            fn bitand_assign(&mut self, other: Self) {
                $(self.$field &= other.$field;)+
            }
        }

        impl<$t: BitAndAssign + Copy> BitAndAssign<$t> for $name<$t> {
            #[inline]
            fn bitand_assign(&mut self, other: $t) {
                $(self.$field &= other;)+
            }
        }

        impl<$t: BitOr<Output = $t>> BitOr for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitor(self, other: Self) -> Self {
                Self {
                    $($field: self.$field | other.$field),+
                }
            }
        }

        impl<$t: BitOr<Output = $t> + Copy> BitOr<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitor(self, other: $t) -> Self {
                Self {
                    $($field: self.$field | other),+
                }
            }
        }

        impl<$t: BitOrAssign> BitOrAssign for $name<$t> {
            #[inline]
            fn bitor_assign(&mut self, other: Self) {
                $(self.$field |= other.$field;)+
            }
        }

        impl<$t: BitOrAssign + Copy> BitOrAssign<$t> for $name<$t> {
            #[inline]
            fn bitor_assign(&mut self, other: $t) {
                $(self.$field |= other;)+
            }
        }

        impl<$t: BitXor<Output = $t>> BitXor for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitxor(self, other: Self) -> Self {
                Self {
                    $($field: self.$field ^ other.$field),+
                }
            }
        }

        impl<$t: BitXor<Output = $t> + Copy> BitXor<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn bitxor(self, other: $t) -> Self {
                Self {
                    $($field: self.$field ^ other),+
                }
            }
        }

        impl<$t: BitXorAssign> BitXorAssign for $name<$t> {
            #[inline]
            fn bitxor_assign(&mut self, other: Self) {
                $(self.$field ^= other.$field;)+
            }
        }

        impl<$t: BitXorAssign + Copy> BitXorAssign<$t> for $name<$t> {
            #[inline]
            fn bitxor_assign(&mut self, other: $t) {
                $(self.$field ^= other;)+
            }
        }

        impl<$t: Shl<Output = $t>> Shl for $name<$t> {
            type Output = Self;

            #[inline]
            fn shl(self, other: Self) -> Self {
                Self {
                    $($field: self.$field << other.$field),+
                }
            }
        }

        impl<$t: Shl<Output = $t> + Copy> Shl<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn shl(self, other: $t) -> Self {
                Self {
                    $($field: self.$field << other),+
                }
            }
        }

        impl<$t: ShlAssign> ShlAssign for $name<$t> {
            #[inline]
            fn shl_assign(&mut self, other: Self) {
                $(self.$field <<= other.$field;)+
            }
        }

        impl<$t: ShlAssign + Copy> ShlAssign<$t> for $name<$t> {
            #[inline]
            fn shl_assign(&mut self, other: $t) {
                $(self.$field <<= other;)+
            }
        }

        impl<$t: Shr<Output = $t>> Shr for $name<$t> {
            type Output = Self;

            #[inline]
            fn shr(self, other: Self) -> Self {
                Self {
                    $($field: self.$field >> other.$field),+
                }
            }
        }

        impl<$t: Shr<Output = $t> + Copy> Shr<$t> for $name<$t> {
            type Output = Self;

            #[inline]
            fn shr(self, other: $t) -> Self {
                Self {
                    $($field: self.$field >> other),+
                }
            }
        }

        impl<$t: ShrAssign> ShrAssign for $name<$t> {
            #[inline]
            fn shr_assign(&mut self, other: Self) {
                $(self.$field >>= other.$field;)+
            }
        }

        impl<$t: ShrAssign + Copy> ShrAssign<$t> for $name<$t> {
            #[inline]
            fn shr_assign(&mut self, other: $t) {
                $(self.$field >>= other;)+
            }
        }

        impl<$t: Neg<Output = $t>> Neg for $name<$t> {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self {
                    $($field: -self.$field),+
                }
            }
        }

        impl<$t: Not<Output = $t>> Not for $name<$t> {
            type Output = Self;

            #[inline]
            fn not(self) -> Self {
                Self {
                    $($field: !self.$field),+
                }
            }
        }

        impl<$t> Index<usize> for $name<$t> {
            type Output = $t;

            #[inline]
            fn index(&self, index: usize) -> &Self::Output {
                let n = 0;
                $(
                    if index == n {
                        return &self.$field;
                    }
                    let n = n + 1;
                )+
                let _ = n;
                panic!("Index out of bounds: {}", index);
            }
        }

        impl<$t> IndexMut<usize> for $name<$t> {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                let n = 0;
                $(
                    if index == n {
                        return &mut self.$field;
                    }
                    let n = n + 1;
                )+
                let _ = n;
                panic!("Index out of bounds: {}", index);
            }
        }
    };
}

pub(crate) use impl_ops;
