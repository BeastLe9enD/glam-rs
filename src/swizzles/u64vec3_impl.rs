// Generated from swizzle_impl.rs.tera template. Edit the template, not the generated file.

use crate::{U64Vec2, U64Vec3, U64Vec4, Vec3Swizzles};

impl Vec3Swizzles for U64Vec3 {
    type Vec2 = U64Vec2;

    type Vec4 = U64Vec4;

    #[inline]
    fn xx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.x,
        }
    }

    #[inline]
    fn xy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    fn xz(self) -> U64Vec2 {
        U64Vec2 {
            x: self.x,
            y: self.z,
        }
    }

    #[inline]
    fn yx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.x,
        }
    }

    #[inline]
    fn yy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.y,
        }
    }

    #[inline]
    fn yz(self) -> U64Vec2 {
        U64Vec2 {
            x: self.y,
            y: self.z,
        }
    }

    #[inline]
    fn zx(self) -> U64Vec2 {
        U64Vec2 {
            x: self.z,
            y: self.x,
        }
    }

    #[inline]
    fn zy(self) -> U64Vec2 {
        U64Vec2 {
            x: self.z,
            y: self.y,
        }
    }

    #[inline]
    fn zz(self) -> U64Vec2 {
        U64Vec2 {
            x: self.z,
            y: self.z,
        }
    }

    #[inline]
    fn xxx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn xxy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn xxz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn xyx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn xyy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn xyz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn xzx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn xzy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn xzz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.x,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn yxx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn yxy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn yxz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn yyx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn yyy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn yyz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn yzx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn yzy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn yzz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.y,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn zxx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.x,
            z: self.x,
        }
    }

    #[inline]
    fn zxy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.x,
            z: self.y,
        }
    }

    #[inline]
    fn zxz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.x,
            z: self.z,
        }
    }

    #[inline]
    fn zyx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.y,
            z: self.x,
        }
    }

    #[inline]
    fn zyy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.y,
            z: self.y,
        }
    }

    #[inline]
    fn zyz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    fn zzx(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.z,
            z: self.x,
        }
    }

    #[inline]
    fn zzy(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.z,
            z: self.y,
        }
    }

    #[inline]
    fn zzz(self) -> U64Vec3 {
        U64Vec3 {
            x: self.z,
            y: self.z,
            z: self.z,
        }
    }

    #[inline]
    fn xxxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.x)
    }

    #[inline]
    fn xxxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.y)
    }

    #[inline]
    fn xxxz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.x, self.z)
    }

    #[inline]
    fn xxyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.x)
    }

    #[inline]
    fn xxyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.y)
    }

    #[inline]
    fn xxyz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.y, self.z)
    }

    #[inline]
    fn xxzx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.z, self.x)
    }

    #[inline]
    fn xxzy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.z, self.y)
    }

    #[inline]
    fn xxzz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.x, self.z, self.z)
    }

    #[inline]
    fn xyxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.x)
    }

    #[inline]
    fn xyxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.y)
    }

    #[inline]
    fn xyxz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.x, self.z)
    }

    #[inline]
    fn xyyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.x)
    }

    #[inline]
    fn xyyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.y)
    }

    #[inline]
    fn xyyz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.y, self.z)
    }

    #[inline]
    fn xyzx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.z, self.x)
    }

    #[inline]
    fn xyzy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.z, self.y)
    }

    #[inline]
    fn xyzz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.y, self.z, self.z)
    }

    #[inline]
    fn xzxx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.x, self.x)
    }

    #[inline]
    fn xzxy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.x, self.y)
    }

    #[inline]
    fn xzxz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.x, self.z)
    }

    #[inline]
    fn xzyx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.y, self.x)
    }

    #[inline]
    fn xzyy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.y, self.y)
    }

    #[inline]
    fn xzyz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.y, self.z)
    }

    #[inline]
    fn xzzx(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.z, self.x)
    }

    #[inline]
    fn xzzy(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.z, self.y)
    }

    #[inline]
    fn xzzz(self) -> U64Vec4 {
        U64Vec4::new(self.x, self.z, self.z, self.z)
    }

    #[inline]
    fn yxxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.x)
    }

    #[inline]
    fn yxxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.y)
    }

    #[inline]
    fn yxxz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.x, self.z)
    }

    #[inline]
    fn yxyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.x)
    }

    #[inline]
    fn yxyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.y)
    }

    #[inline]
    fn yxyz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.y, self.z)
    }

    #[inline]
    fn yxzx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.z, self.x)
    }

    #[inline]
    fn yxzy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.z, self.y)
    }

    #[inline]
    fn yxzz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.x, self.z, self.z)
    }

    #[inline]
    fn yyxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.x)
    }

    #[inline]
    fn yyxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.y)
    }

    #[inline]
    fn yyxz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.x, self.z)
    }

    #[inline]
    fn yyyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.x)
    }

    #[inline]
    fn yyyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.y)
    }

    #[inline]
    fn yyyz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.y, self.z)
    }

    #[inline]
    fn yyzx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.z, self.x)
    }

    #[inline]
    fn yyzy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.z, self.y)
    }

    #[inline]
    fn yyzz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.y, self.z, self.z)
    }

    #[inline]
    fn yzxx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.x, self.x)
    }

    #[inline]
    fn yzxy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.x, self.y)
    }

    #[inline]
    fn yzxz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.x, self.z)
    }

    #[inline]
    fn yzyx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.y, self.x)
    }

    #[inline]
    fn yzyy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.y, self.y)
    }

    #[inline]
    fn yzyz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.y, self.z)
    }

    #[inline]
    fn yzzx(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.z, self.x)
    }

    #[inline]
    fn yzzy(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.z, self.y)
    }

    #[inline]
    fn yzzz(self) -> U64Vec4 {
        U64Vec4::new(self.y, self.z, self.z, self.z)
    }

    #[inline]
    fn zxxx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.x, self.x)
    }

    #[inline]
    fn zxxy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.x, self.y)
    }

    #[inline]
    fn zxxz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.x, self.z)
    }

    #[inline]
    fn zxyx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.y, self.x)
    }

    #[inline]
    fn zxyy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.y, self.y)
    }

    #[inline]
    fn zxyz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.y, self.z)
    }

    #[inline]
    fn zxzx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.z, self.x)
    }

    #[inline]
    fn zxzy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.z, self.y)
    }

    #[inline]
    fn zxzz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.x, self.z, self.z)
    }

    #[inline]
    fn zyxx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.x, self.x)
    }

    #[inline]
    fn zyxy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.x, self.y)
    }

    #[inline]
    fn zyxz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.x, self.z)
    }

    #[inline]
    fn zyyx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.y, self.x)
    }

    #[inline]
    fn zyyy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.y, self.y)
    }

    #[inline]
    fn zyyz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.y, self.z)
    }

    #[inline]
    fn zyzx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.z, self.x)
    }

    #[inline]
    fn zyzy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.z, self.y)
    }

    #[inline]
    fn zyzz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.y, self.z, self.z)
    }

    #[inline]
    fn zzxx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.x, self.x)
    }

    #[inline]
    fn zzxy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.x, self.y)
    }

    #[inline]
    fn zzxz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.x, self.z)
    }

    #[inline]
    fn zzyx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.y, self.x)
    }

    #[inline]
    fn zzyy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.y, self.y)
    }

    #[inline]
    fn zzyz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.y, self.z)
    }

    #[inline]
    fn zzzx(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.z, self.x)
    }

    #[inline]
    fn zzzy(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.z, self.y)
    }

    #[inline]
    fn zzzz(self) -> U64Vec4 {
        U64Vec4::new(self.z, self.z, self.z, self.z)
    }
}
