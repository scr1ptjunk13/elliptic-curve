use num_bigint::BigUint;

// y^2 = x^3 + ax + b (mod p)
struct EllipticCurve {
    a: BigUint,
    b: BigUint,
    p: BigUint,
}

// (x, y)
struct Point {
    x: BigUint,
    y: BigUint,
}


impl EllipticCurve {
    fn add(x: &Point, y: &Point) -> Point {
        todo!()
    }    

    fn double(x: &Point) -> Point {
        todo!()
    }

    // k * x
    fn scalar_mult(x: &Point, k: &BigUint) -> Point {
        // double-and-add algorithm OR binary-method
        todo!()
    }
}

struct FiniteField {
    p: BigUint,
}

impl FiniteField {
    fn add(&self, x: &BigUint, y: &BigUint) -> BigUint {
        todo!()
    }

    fn sub(&self, x: &BigUint, y: &BigUint) -> BigUint {
        todo!()
    }
    
    fn mul(&self, x: &BigUint, y: &BigUint) -> BigUint {
        todo!()
    }

    fn div(&self, x: &BigUint, y: &BigUint) -> BigUint {
        todo!()
    }

    
}