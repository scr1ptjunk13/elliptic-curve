use crate::{EllipticCurve, Point, FiniteField};
use num_bigint::{BigUint, RandBigInt};
use sha2::{Sha256, Digest};
use rand::thread_rng;

#[derive(Clone, Debug)]
pub struct ECDSAKeyPair {
    pub private_key: BigUint,
    pub public_key: Point,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ECDSASignature {
    pub r: BigUint,
    pub s: BigUint,
}

pub struct ECDSA {
    pub curve: EllipticCurve,
    pub generator: Point,
    pub order: BigUint,
}

impl ECDSA {
    pub fn new(curve: EllipticCurve, generator: Point, order: BigUint) -> Self {
        ECDSA {
            curve,
            generator,
            order,
        }
    }

    // Generate private key: random in [1, n-1]
    pub fn generate_private_key(&self) -> BigUint {
        let mut rng = thread_rng();
        rng.gen_biguint_range(&BigUint::from(1u32), &self.order)
    }

    // Generate public key: Q = d * G
    pub fn generate_public_key(&self, private_key: &BigUint) -> Point {
        self.curve.scalar_mult(&self.generator, private_key)
    }

    // Generate keypair
    pub fn generate_keypair(&self) -> ECDSAKeyPair {
        let private_key = self.generate_private_key();
        let public_key = self.generate_public_key(&private_key);
        ECDSAKeyPair {
            private_key,
            public_key,
        }
    }

    // Hash message with SHA-256
    fn hash_message(&self, message: &[u8]) -> BigUint {
        let mut hasher = Sha256::new();
        hasher.update(message);
        let hash = hasher.finalize();
        let hash_int = BigUint::from_bytes_be(&hash);
        hash_int % &self.order
    }

    // Sign message
    // s = k^(-1) * (z + r * d) mod n
    pub fn sign(&self, message: &[u8], private_key: &BigUint) -> Result<ECDSASignature, &'static str> {
        let mut rng = thread_rng();
        let field = FiniteField { p: self.order.clone() };
        let z = self.hash_message(message);

        loop {
            // Generate random k
            let k = rng.gen_biguint_range(&BigUint::from(1u32), &self.order);
            
            // Compute R = k * G
            let point = self.curve.scalar_mult(&self.generator, &k);
            
            let r = match point {
                Point::Coordinate(x, _) => x % &self.order,
                Point::Identity => continue,
            };

            if r == BigUint::from(0u32) {
                continue;
            }

            // Compute s = k^(-1) * (z + r * d) mod n
            let r_d = field.mul(&r, private_key);
            let z_r_d = field.add(&z, &r_d);
            let k_inv = field.div(&BigUint::from(1u32), &k);
            let s = field.mul(&k_inv, &z_r_d);

            if s == BigUint::from(0u32) {
                continue;
            }

            return Ok(ECDSASignature { r, s });
        }
    }

    // Verify signature
    // Check if r == x_p mod n where (x_p, y_p) = u1*G + u2*Q
    pub fn verify(&self, message: &[u8], signature: &ECDSASignature, public_key: &Point) -> bool {
        let field = FiniteField { p: self.order.clone() };
        
        // Check r and s in valid range
        if signature.r == BigUint::from(0u32) || signature.r >= self.order ||
           signature.s == BigUint::from(0u32) || signature.s >= self.order {
            return false;
        }

        let z = self.hash_message(message);

        // Compute w = s^(-1) mod n
        let w = field.div(&BigUint::from(1u32), &signature.s);

        // Compute u1 = z * w mod n, u2 = r * w mod n
        let u1 = field.mul(&z, &w);
        let u2 = field.mul(&signature.r, &w);

        // Compute point P = u1*G + u2*Q
        let u1_g = self.curve.scalar_mult(&self.generator, &u1);
        let u2_q = self.curve.scalar_mult(public_key, &u2);
        let point = self.curve.add(&u1_g, &u2_q);

        // Verify r == x_p mod n
        match point {
            Point::Coordinate(x, _) => (x % &self.order) == signature.r,
            Point::Identity => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_ecdsa() -> ECDSA {
        let curve = EllipticCurve {
            a: BigUint::from(2u32),
            b: BigUint::from(2u32),
            p: BigUint::from(17u32),
        };
        
        let generator = Point::Coordinate(
            BigUint::from(5u32),
            BigUint::from(1u32)
        );
        
        let order = BigUint::from(19u32);
        
        ECDSA::new(curve, generator, order)
    }

    #[test]
    fn test_keygen() {
        let ecdsa = setup_ecdsa();
        let keypair = ecdsa.generate_keypair();
        
        assert!(keypair.private_key > BigUint::from(0u32));
        assert!(keypair.private_key < ecdsa.order);
        assert!(ecdsa.curve.is_on_curve(&keypair.public_key));
    }

    #[test]
    fn test_sign_verify() {
        let ecdsa = setup_ecdsa();
        let keypair = ecdsa.generate_keypair();
        let message = b"test message";
        
        let signature = ecdsa.sign(message, &keypair.private_key).unwrap();
        assert!(ecdsa.verify(message, &signature, &keypair.public_key));
        
        let wrong_message = b"wrong message";
        assert!(!ecdsa.verify(wrong_message, &signature, &keypair.public_key));
    }

    #[test]
    fn test_invalid_signature() {
        let ecdsa = setup_ecdsa();
        let keypair = ecdsa.generate_keypair();
        let message = b"test";
        
        let invalid_sig = ECDSASignature {
            r: BigUint::from(0u32),
            s: BigUint::from(1u32),
        };
        
        assert!(!ecdsa.verify(message, &invalid_sig, &keypair.public_key));
    }
}
