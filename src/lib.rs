use num_bigint::BigUint;

pub mod ecdsa;

// y^2 = x^3 + ax + b (mod p)
#[derive(Clone)]
pub struct EllipticCurve {
    pub a: BigUint,
    pub b: BigUint,
    pub p: BigUint,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Point {
    Coordinate(BigUint, BigUint),  // Regular point (x, y)
    Identity,                      // Point at infinity (identity element)
}


impl EllipticCurve {
    // Check if a point is on the elliptic curve: y² = x³ + ax + b (mod p)
    pub fn is_on_curve(&self, point: &Point) -> bool {
        match point {
            Point::Identity => true,  // Point at infinity is always on the curve
            Point::Coordinate(x, y) => {
                let field = FiniteField { p: self.p.clone() };
                
                // Left side: y²
                let y_squared = field.mul(y, y);
                
                // Right side: x³ + ax + b
                let x_squared = field.mul(x, x);
                let x_cubed = field.mul(&x_squared, x);
                let ax = field.mul(&self.a, x);
                let right_side = field.add(&field.add(&x_cubed, &ax), &self.b);
                
                y_squared == right_side
            }
        }
    }

    // Elliptic curve point addition: P + Q
    pub fn add(&self, p: &Point, q: &Point) -> Point {
        match (p, q) {
            // Identity cases: P + O = P, O + P = P
            (Point::Identity, _) => q.clone(),
            (_, Point::Identity) => p.clone(),
            
            (Point::Coordinate(x1, y1), Point::Coordinate(x2, y2)) => {
                let field = FiniteField { p: self.p.clone() };
                
                // Case 1: Same x-coordinate
                if x1 == x2 {
                    if y1 == y2 {
                        // Same point: use doubling
                        self.double(p)
                    } else {
                        // Opposite points: P + (-P) = O
                        Point::Identity
                    }
                } else {
                    // Case 2: Different points
                    // Slope: s = (y2 - y1) / (x2 - x1)
                    let dy = field.sub(y2, y1);
                    let dx = field.sub(x2, x1);
                    let slope = field.div(&dy, &dx);
                    
                    // x3 = s² - x1 - x2
                    let slope_squared = field.mul(&slope, &slope);
                    let x3 = field.sub(&field.sub(&slope_squared, x1), x2);
                    
                    // y3 = s(x1 - x3) - y1
                    let dx1_x3 = field.sub(x1, &x3);
                    let y3 = field.sub(&field.mul(&slope, &dx1_x3), y1);
                    
                    Point::Coordinate(x3, y3)
                }
            }
        }
    }

    // Point doubling: 2P
    pub fn double(&self, p: &Point) -> Point {
        match p {
            Point::Identity => Point::Identity,
            Point::Coordinate(x, y) => {
                let field = FiniteField { p: self.p.clone() };
                
                // Check if y = 0 (point has order 2)
                if *y == BigUint::from(0u32) {
                    return Point::Identity;
                }
                
                // Slope: s = (3x² + a) / (2y)
                let three = BigUint::from(3u32);
                let two = BigUint::from(2u32);
                
                let x_squared = field.mul(x, x);
                let three_x_squared = field.mul(&three, &x_squared);
                let numerator = field.add(&three_x_squared, &self.a);
                let denominator = field.mul(&two, y);
                let slope = field.div(&numerator, &denominator);
                
                // x3 = s² - 2x
                let slope_squared = field.mul(&slope, &slope);
                let two_x = field.mul(&two, x);
                let x3 = field.sub(&slope_squared, &two_x);
                
                // y3 = s(x - x3) - y
                let dx_x3 = field.sub(x, &x3);
                let y3 = field.sub(&field.mul(&slope, &dx_x3), y);
                
                Point::Coordinate(x3, y3)
            }
        }
    }

    // Scalar multiplication: k * P (double-and-add algorithm)
    pub fn scalar_mult(&self, point: &Point, k: &BigUint) -> Point {
        // Handle edge cases
        if *k == BigUint::from(0u32) {
            return Point::Identity;
        }
        if *k == BigUint::from(1u32) {
            return point.clone();
        }
        
        let mut result = Point::Identity;
        let mut addend = point.clone();
        let mut scalar = k.clone();
        
        // Double-and-add algorithm
        while scalar > BigUint::from(0u32) {
            // If current bit is 1, add current power of point
            if &scalar % BigUint::from(2u32) == BigUint::from(1u32) {
                result = self.add(&result, &addend);
            }
            
            // Double the addend and halve the scalar
            addend = self.double(&addend);
            scalar /= BigUint::from(2u32);
        }
        
        result
    }
}

pub struct FiniteField {
    pub p: BigUint,
}

impl FiniteField {
    pub fn add(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x + y) % &self.p
    }

    // addition by additive inverse (subtraction)
    pub fn sub(&self, x: &BigUint, y: &BigUint) -> BigUint {
        // Subtraction: x - y = x + (-y) mod p
        // Additive inverse: -y = (p - y) mod p
        let neg_y = (&self.p - y) % &self.p;
        self.add(x, &neg_y)
    }
    
    pub fn mul(&self, x: &BigUint, y: &BigUint) -> BigUint {
        (x * y) % &self.p
    }

    // multiplication by multiplicative inverse (division)
    pub fn div(&self, x: &BigUint, y: &BigUint) -> BigUint {
        // division in finite field: x / y = x * y^(-1) mod p

        if *y == BigUint::from(0u32) {
            panic!("?division by zero in finite field?");
        }
        
        // Using Fermat's little theorem: y^(-1) = y^(p-2) mod p (when p is prime)
        let p_minus_2 = &self.p - BigUint::from(2u32);
        let y_inverse = y.modpow(&p_minus_2, &self.p);
        self.mul(x, &y_inverse)
    }

    // a^(p-1) ≡ 1 (mod p)
    // a^(p-2) × a ≡ 1 (mod p) i.e a^(p-2) is the multiplicative inverse of a

    // x ÷ y = x × y^(-1) = x × y^(p-2) (mod p)

    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(3u32);

        let res = BigUint::from(9u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.add(&x, &y), res);
    }
    
    #[test]
    fn test_sub() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(3u32);

        let res = BigUint::from(3u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.sub(&x, &y), res);
    }

    #[test]
    fn test_mul() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(3u32);

        let res = BigUint::from(7u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.mul(&x, &y), res);
    }

    #[test]
    fn test_div() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(3u32);

        let res = BigUint::from(2u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.div(&x, &y), res);
    }

    //fail test case 
    #[test]
    #[should_panic]
    fn test_div_fail() {
        let x = BigUint::from(17u32);
        let y = BigUint::from(0u32);

        let field = FiniteField { p: BigUint::from(11u32) };
        assert_eq!(field.div(&x, &y), BigUint::from(0u32));
    }

    #[test]
    fn test_multiplicative_identity() {
        let x = BigUint::from(4u32);
        let field = FiniteField { p: BigUint::from(11u32) };
        
        // Test: x * x^(-1) = 1
        // 4^(-1) mod 11 = 3, so 4 * 3 = 12 ≡ 1 mod 11
        let one = BigUint::from(1u32);
        let x_inverse = field.div(&one, &x);  // 1/x = x^(-1)

        assert_eq!(field.mul(&x, &x_inverse), one);
    }

    #[test]
    fn test_additive_identity() {
        let x = BigUint::from(7u32);
        let field = FiniteField { p: BigUint::from(11u32) };
        
        // Test: x + (-x) = 0
        // -7 mod 11 = 4, so 7 + 4 = 11 ≡ 0 mod 11
        let zero = BigUint::from(0u32);
        let neg_x = field.sub(&zero, &x);  // 0 - x = -x
        assert_eq!(field.add(&x, &neg_x), zero);
    }

    #[test]
    fn test_point_on_curve() {
        // Test curve: y² = x³ + 2x + 3 (mod 11)
        let curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(3u32),
            p: BigUint::from(11u32),
        };

        // Test point at infinity
        assert!(curve.is_on_curve(&Point::Identity));

        // Test valid point: (5, 1) should be on the curve
        // Check: 1² = 5³ + 2×5 + 3 = 125 + 10 + 3 = 138 ≡ 6 (mod 11)
        // Actually: 1² = 1, so let's find a point that works
        // For x = 0: y² = 0 + 0 + 3 = 3 (mod 11), so y = ±√3
        // √3 mod 11 = 5 (since 5² = 25 ≡ 3 mod 11)
        let valid_point = Point::Coordinate(BigUint::from(0u32), BigUint::from(5u32));
        assert!(curve.is_on_curve(&valid_point));

        // Test invalid point
        let invalid_point = Point::Coordinate(BigUint::from(1u32), BigUint::from(1u32));
        assert!(!curve.is_on_curve(&invalid_point));
    }

    #[test]
    fn test_ec_point_addition() {
        // Test curve: y² = x³ + 2x + 3 (mod 11)
        let curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(3u32),
            p: BigUint::from(11u32),
        };

        // Test P + O = P (identity)
        let p = Point::Coordinate(BigUint::from(0u32), BigUint::from(5u32));
        let result = curve.add(&p, &Point::Identity);
        assert_eq!(result, p);

        // Test O + P = P (identity)
        let result = curve.add(&Point::Identity, &p);
        assert_eq!(result, p);

        // Test P + P = 2P (doubling case)
        let doubled = curve.double(&p);
        let result = curve.add(&p, &p);
        assert_eq!(result, doubled);

        // Test P + (-P) = O (opposite points)
        let neg_p = Point::Coordinate(BigUint::from(0u32), BigUint::from(6u32)); // -5 mod 11 = 6
        let result = curve.add(&p, &neg_p);
        assert_eq!(result, Point::Identity);
    }

    #[test]
    fn test_ec_point_doubling() {
        let curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(3u32),
            p: BigUint::from(11u32),
        };

        // Test 2O = O
        let result = curve.double(&Point::Identity);
        assert_eq!(result, Point::Identity);

        // Test doubling regular point
        let p = Point::Coordinate(BigUint::from(0u32), BigUint::from(5u32));
        let doubled = curve.double(&p);
        
        // Verify the doubled point is on the curve
        assert!(curve.is_on_curve(&doubled));
    }

    #[test]
    // h/w [homework] : test y = 0 special case
    fn test_y_zero_special_case() {
        let curve = EllipticCurve {
            a: BigUint::from(0u32),
            b: BigUint::from(7u32),
            p: BigUint::from(11u32),
        };

        // Find a point with y = 0: x³ + 7 ≡ 0 (mod 11)
        // x³ ≡ -7 ≡ 4 (mod 11)
        // Need to find x where x³ ≡ 4 (mod 11)
        // Testing: 2³ = 8, 3³ = 27 ≡ 5, 4³ = 64 ≡ 9, 5³ = 125 ≡ 4 ✓
        let point_y_zero = Point::Coordinate(BigUint::from(5u32), BigUint::from(0u32));
        
        // Verify point is on curve
        assert!(curve.is_on_curve(&point_y_zero));
        
        // Test: 2P should equal Identity when y = 0
        let result = curve.double(&point_y_zero);
        assert_eq!(result, Point::Identity);
    }

    #[test]
    fn test_scalar_multiplication() {
        let curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(3u32),
            p: BigUint::from(11u32),
        };

        let p = Point::Coordinate(BigUint::from(0u32), BigUint::from(5u32));

        // Test 0 * P = O
        let result = curve.scalar_mult(&p, &BigUint::from(0u32));
        assert_eq!(result, Point::Identity);

        // Test 1 * P = P
        let result = curve.scalar_mult(&p, &BigUint::from(1u32));
        assert_eq!(result, p);

        // Test 2 * P = P + P
        let result = curve.scalar_mult(&p, &BigUint::from(2u32));
        let expected = curve.add(&p, &p);
        assert_eq!(result, expected);

        // Test 3 * P = 2P + P
        let result = curve.scalar_mult(&p, &BigUint::from(3u32));
        let two_p = curve.scalar_mult(&p, &BigUint::from(2u32));
        let expected = curve.add(&two_p, &p);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_secp256k1_like_curve() {
        // Simplified version of secp256k1: y² = x³ + 7 (mod small_prime)
        // Using small prime for testing: p = 23
        let curve = EllipticCurve {
            a: BigUint::from(0u32),  // secp256k1 has a = 0
            b: BigUint::from(7u32),  // secp256k1 has b = 7
            p: BigUint::from(23u32), // Small prime for testing
        };

        // Find a valid point on y² = x³ + 7 (mod 23)
        // For x = 1: y² = 1 + 7 = 8 (mod 23)
        // Need to check if 8 is a quadratic residue mod 23
        // 8 is not a perfect square mod 23, try x = 2
        // For x = 2: y² = 8 + 7 = 15 (mod 23)
        // For x = 3: y² = 27 + 7 = 34 ≡ 11 (mod 23)
        // For x = 4: y² = 64 + 7 = 71 ≡ 2 (mod 23)
        // For x = 5: y² = 125 + 7 = 132 ≡ 17 (mod 23)
        // For x = 6: y² = 216 + 7 = 223 ≡ 16 = 4² (mod 23) ✓
        
        let point = Point::Coordinate(BigUint::from(6u32), BigUint::from(4u32));
        assert!(curve.is_on_curve(&point));

        // Test some operations
        let doubled = curve.double(&point);
        assert!(curve.is_on_curve(&doubled));

        let tripled = curve.scalar_mult(&point, &BigUint::from(3u32));
        assert!(curve.is_on_curve(&tripled));
    }

    #[test]
    fn test_group_properties() {
        let curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(3u32),
            p: BigUint::from(11u32),
        };

        let p1 = Point::Coordinate(BigUint::from(0u32), BigUint::from(5u32));
        let p2 = Point::Coordinate(BigUint::from(0u32), BigUint::from(6u32)); // -p1

        // Test associativity: (P + Q) + R = P + (Q + R)
        let q = curve.double(&p1); // Some other point
        let r = curve.add(&p1, &p2); // Should be Identity
        
        let left = curve.add(&curve.add(&p1, &q), &r);
        let right = curve.add(&p1, &curve.add(&q, &r));
        assert_eq!(left, right);

        // Test commutativity: P + Q = Q + P
        let sum1 = curve.add(&p1, &q);
        let sum2 = curve.add(&q, &p1);
        assert_eq!(sum1, sum2);

        // Test inverse: P + (-P) = O
        let sum = curve.add(&p1, &p2);
        assert_eq!(sum, Point::Identity);
    }

}
    


//modpow(exponent,modulus)