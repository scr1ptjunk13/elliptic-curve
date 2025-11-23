# Elliptic Curve Digital Signature Algorithm (ECDSA) in Rust

a rust cryptographic library implementing ECDSA from scratch

![ECDH](./ECDH.gif)

A comprehensive, educational implementation of ECDSA (Elliptic Curve Digital Signature Algorithm) from scratch in Rust. This project demonstrates the mathematical foundations of elliptic curve cryptography and digital signatures.

## 📚 Table of Contents

- [Overview](#overview)
- [Mathematical Background](#mathematical-background)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [How It Works](#how-it-works)
- [Security Considerations](#security-considerations)
- [Testing](#testing)
- [Educational Resources](#educational-resources)
- [Contributing](#contributing)
- [License](#license)

## 🔍 Overview

This project implements ECDSA, a cryptographic algorithm used for digital signatures based on elliptic curve mathematics. ECDSA is widely used in:

- **Bitcoin & Cryptocurrencies**: Signing transactions
- **TLS/SSL**: Securing web communications
- **SSH**: Authenticating connections
- **Code Signing**: Verifying software authenticity

### Why ECDSA?

- **Smaller Keys**: 256-bit ECDSA provides similar security to 3072-bit RSA
- **Faster Operations**: More efficient than RSA for signing
- **Modern Standard**: Widely adopted in contemporary cryptographic systems

## 📐 Mathematical Background

### Finite Fields

A finite field $\mathbb{F}_p$ consists of integers modulo a prime $p$ with operations:

- **Addition**: $(a + b) \bmod p$
- **Multiplication**: $(a \times b) \bmod p$
- **Division**: $a \times b^{-1} \bmod p$ where $b^{-1} = b^{p-2} \bmod p$ (Fermat's Little Theorem)

### Elliptic Curves

An elliptic curve over $\mathbb{F}_p$ is defined by the equation:

$$y^2 = x^3 + ax + b \pmod{p}$$

where $4a^3 + 27b^2 \neq 0 \pmod{p}$ (non-singular condition).

#### Point Addition

For points $P = (x_1, y_1)$ and $Q = (x_2, y_2)$:

**Case 1: Different Points** ($P \neq Q$)

$$s = \frac{y_2 - y_1}{x_2 - x_1} \bmod p$$

$$x_3 = s^2 - x_1 - x_2 \bmod p$$

$$y_3 = s(x_1 - x_3) - y_1 \bmod p$$

**Case 2: Point Doubling** ($P = Q$)

$$s = \frac{3x_1^2 + a}{2y_1} \bmod p$$

$$x_3 = s^2 - 2x_1 \bmod p$$

$$y_3 = s(x_1 - x_3) - y_1 \bmod p$$

#### Scalar Multiplication

Computing $k \cdot P$ uses the **double-and-add algorithm**:

```
Result = O (identity)
For each bit in k (from right to left):
    If bit is 1: Result = Result + P
    P = 2P (double)
```

Time complexity: $O(\log k)$

### ECDSA Algorithm

#### Key Generation

1. Choose random private key: $d \in [1, n-1]$ where $n$ is the curve order
2. Compute public key: $Q = d \cdot G$ where $G$ is the generator point

#### Signing

To sign message $m$ with private key $d$:

1. Compute hash: $z = \text{SHA256}(m) \bmod n$
2. Generate random $k \in [1, n-1]$
3. Compute point: $(x, y) = k \cdot G$
4. Compute: $r = x \bmod n$
5. Compute: $s = k^{-1}(z + rd) \bmod n$
6. Signature is $(r, s)$

#### Verification

To verify signature $(r, s)$ on message $m$ with public key $Q$:

1. Check: $r, s \in [1, n-1]$
2. Compute hash: $z = \text{SHA256}(m) \bmod n$
3. Compute: $w = s^{-1} \bmod n$
4. Compute: $u_1 = zw \bmod n$, $u_2 = rw \bmod n$
5. Compute point: $(x, y) = u_1 \cdot G + u_2 \cdot Q$
6. Verify: $r = x \bmod n$

## ✨ Features

- **Finite Field Arithmetic**: Addition, subtraction, multiplication, division over $\mathbb{F}_p$
- **Elliptic Curve Operations**: Point addition, doubling, scalar multiplication
- **ECDSA Implementation**: Complete key generation, signing, and verification
- **Comprehensive Testing**: 19 unit tests covering all major functionality
- **Educational Code**: Well-commented with clear mathematical foundations
- **Type Safety**: Leverages Rust's type system for correctness

## 🚀 Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/elliptic-curve.git
cd elliptic-curve

# Build the project
cargo build

# Run tests
cargo test

# Run the demo
cargo run
```

### Dependencies

```toml
[dependencies]
num-bigint = { version = "0.4", features = ["rand"] }
sha2 = "0.10"
rand = "0.8"
```

## 💻 Usage

### Basic Example

```rust
use elliptic_curve::{EllipticCurve, Point, ecdsa::ECDSA};
use num_bigint::BigUint;

fn main() {
    // Define an elliptic curve: y² = x³ + 2x + 2 (mod 17)
    let curve = EllipticCurve {
        a: BigUint::from(2u32),
        b: BigUint::from(2u32),
        p: BigUint::from(17u32),
    };

    // Generator point
    let generator = Point::Coordinate(
        BigUint::from(5u32),
        BigUint::from(1u32),
    );

    // Create ECDSA instance
    let order = BigUint::from(19u32);
    let ecdsa = ECDSA::new(curve, generator, order);

    // Generate keypair
    let keypair = ecdsa.generate_keypair();
    println!("Private key: {}", keypair.private_key);
    println!("Public key: {:?}", keypair.public_key);

    // Sign a message
    let message = b"Hello, ECDSA!";
    let signature = ecdsa.sign(message, &keypair.private_key).unwrap();
    println!("Signature: (r={}, s={})", signature.r, signature.s);

    // Verify the signature
    let is_valid = ecdsa.verify(message, &signature, &keypair.public_key);
    println!("Signature valid: {}", is_valid);
}
```

### Elliptic Curve Operations

```rust
use elliptic_curve::{EllipticCurve, Point};
use num_bigint::BigUint;

let curve = EllipticCurve {
    a: BigUint::from(2u32),
    b: BigUint::from(3u32),
    p: BigUint::from(11u32),
};

let point = Point::Coordinate(BigUint::from(0u32), BigUint::from(5u32));

// Point doubling
let doubled = curve.double(&point);

// Scalar multiplication
let result = curve.scalar_mult(&point, &BigUint::from(5u32));

// Point addition
let sum = curve.add(&point, &doubled);
```

## 📁 Project Structure

```
elliptic-curve/
├── src/
│   ├── lib.rs          # Core elliptic curve implementation
│   ├── ecdsa.rs        # ECDSA signing and verification
│   └── main.rs         # Demo application
├── Cargo.toml          # Project dependencies
└── README.md           # This file
```

### Module Overview

#### `lib.rs`

- **`FiniteField`**: Arithmetic operations in $\mathbb{F}_p$
- **`Point`**: Enum representing curve points (coordinates or identity)
- **`EllipticCurve`**: Core elliptic curve operations
  - `is_on_curve()`: Verify point validity
  - `add()`: Point addition
  - `double()`: Point doubling
  - `scalar_mult()`: Scalar multiplication

#### `ecdsa.rs`

- **`ECDSA`**: Main ECDSA implementation
  - `generate_keypair()`: Create public/private key pair
  - `sign()`: Generate digital signature
  - `verify()`: Verify signature authenticity
- **`ECDSAKeyPair`**: Container for key pairs
- **`ECDSASignature`**: Container for signatures $(r, s)$

## 🔧 How It Works

### 1. Finite Field Operations

The `FiniteField` struct implements modular arithmetic:

```rust
pub struct FiniteField {
    pub p: BigUint,  // Prime modulus
}

impl FiniteField {
    // Division using Fermat's Little Theorem
    // a/b = a × b^(-1) = a × b^(p-2) mod p
    pub fn div(&self, x: &BigUint, y: &BigUint) -> BigUint {
        let p_minus_2 = &self.p - BigUint::from(2u32);
        let y_inverse = y.modpow(&p_minus_2, &self.p);
        self.mul(x, &y_inverse)
    }
}
```

**Key Insight**: Fermat's Little Theorem states that for prime $p$ and $a \not\equiv 0$:

$$a^{p-1} \equiv 1 \pmod{p}$$

Therefore: $a^{p-2} \times a \equiv 1 \pmod{p}$, so $a^{-1} = a^{p-2} \bmod p$

### 2. Point Operations

Points on the curve can be:
- **Coordinate points**: $(x, y)$ satisfying the curve equation
- **Identity (point at infinity)**: The additive identity element $\mathcal{O}$

The `Point` enum handles both cases:

```rust
pub enum Point {
    Coordinate(BigUint, BigUint),
    Identity,
}
```

### 3. Scalar Multiplication Optimization

The double-and-add algorithm efficiently computes $k \cdot P$:

```rust
pub fn scalar_mult(&self, point: &Point, k: &BigUint) -> Point {
    let mut result = Point::Identity;
    let mut addend = point.clone();
    let mut scalar = k.clone();
    
    while scalar > BigUint::from(0u32) {
        if &scalar % BigUint::from(2u32) == BigUint::from(1u32) {
            result = self.add(&result, &addend);
        }
        addend = self.double(&addend);
        scalar /= BigUint::from(2u32);
    }
    
    result
}
```

**Example**: Computing $13 \cdot P$ where $13 = 1101_2$

```
13 = 1×8 + 1×4 + 0×2 + 1×1
13·P = 8P + 4P + P
```

Only 3 additions instead of 12!

### 4. Signature Generation

The signing process ensures:
- **Uniqueness**: Random $k$ makes each signature unique
- **Non-repudiation**: Only the private key holder can sign
- **Integrity**: Any message modification invalidates the signature

```rust
pub fn sign(&self, message: &[u8], private_key: &BigUint) 
    -> Result<ECDSASignature, &'static str> {
    
    let z = self.hash_message(message);
    
    loop {
        let k = generate_random();
        let point = self.curve.scalar_mult(&self.generator, &k);
        
        let r = extract_x_coordinate(point) % &self.order;
        if r == 0 { continue; }
        
        let s = k_inv * (z + r * private_key) % &self.order;
        if s == 0 { continue; }
        
        return Ok(ECDSASignature { r, s });
    }
}
```

### 5. Signature Verification

Verification works because:

$$u_1 \cdot G + u_2 \cdot Q = u_1 \cdot G + u_2 \cdot (d \cdot G) = (u_1 + u_2 d) \cdot G$$

Where:
- $u_1 = zw \bmod n$
- $u_2 = rw \bmod n$
- $w = s^{-1} \bmod n$

Substituting the signature equation $s = k^{-1}(z + rd)$:

$$u_1 + u_2 d = zs^{-1} + rs^{-1}d = s^{-1}(z + rd) = k$$

Therefore: $(u_1 + u_2 d) \cdot G = k \cdot G$, and the x-coordinate equals $r$.

## 🔒 Security Considerations

### ⚠️ Educational Purpose Only

**This implementation is for educational purposes and should NOT be used in production systems.**

### Known Limitations

1. **Small Test Curves**: Uses small primes (e.g., $p = 17$) for demonstration
   - Real systems use 256-bit curves (e.g., secp256k1, P-256)
   - Small curves are vulnerable to brute-force attacks

2. **Timing Attacks**: Not resistant to side-channel attacks
   - No constant-time operations
   - Vulnerable to timing analysis

3. **Random Number Generation**: Uses standard RNG
   - Production systems need cryptographically secure RNG
   - Weak randomness can leak private keys

4. **Hash Collisions**: With small order curves ($n = 19$), hash collisions are possible
   - Real curves use 256-bit orders where collisions are computationally infeasible

5. **No Nonce Reuse Protection**: Reusing $k$ reveals the private key
   - Production implementations use deterministic $k$ (RFC 6979)

### Production Recommendations

For real applications, use battle-tested libraries:

- **Rust**: `secp256k1`, `ring`, `ed25519-dalek`
- **Bitcoin/Ethereum**: Use official client libraries
- **General Purpose**: OpenSSL, libsodium

### The Nonce Reuse Attack

If you sign two different messages with the same $k$:

$$s_1 = k^{-1}(z_1 + rd) \bmod n$$
$$s_2 = k^{-1}(z_2 + rd) \bmod n$$

An attacker can compute:

$$k = \frac{z_1 - z_2}{s_1 - s_2} \bmod n$$

Then recover the private key:

$$d = \frac{sk - z}{r} \bmod n$$

**This happened in real life**: The PlayStation 3 signing key was compromised due to Sony reusing $k$.

## 🧪 Testing

### Run All Tests

```bash
cargo test
```

### Test Coverage

The project includes 19 comprehensive tests:

#### Finite Field Tests
- `test_add`: Addition modulo $p$
- `test_sub`: Subtraction modulo $p$
- `test_mul`: Multiplication modulo $p$
- `test_div`: Division using modular inverse
- `test_multiplicative_identity`: $x \times x^{-1} \equiv 1$
- `test_additive_identity`: $x + (-x) \equiv 0$

#### Elliptic Curve Tests
- `test_point_on_curve`: Point validation
- `test_ec_point_addition`: Addition of distinct points
- `test_ec_point_doubling`: Point doubling ($P + P$)
- `test_y_zero_special_case`: Points with $y = 0$ have order 2
- `test_scalar_multiplication`: $k \cdot P$ verification
- `test_group_properties`: Associativity and commutativity
- `test_secp256k1_like_curve`: Testing $y^2 = x^3 + 7$

#### ECDSA Tests
- `test_keygen`: Key pair generation
- `test_sign_verify`: Basic signing and verification
- `test_sign_verify_same_message_twice`: Different signatures for same message
- `test_invalid_signature`: Reject invalid signatures
- `test_signature_with_wrong_public_key`: Reject wrong public keys

### Test Output

```
running 19 tests
test tests::test_add ... ok
test tests::test_additive_identity ... ok
test ecdsa::tests::test_keygen ... ok
...
test result: ok. 19 passed; 0 failed; 0 ignored
```

## 📖 Educational Resources

### Books

- **"Understanding Cryptography" by Christof Paar**: Excellent introduction to elliptic curves
- **"Guide to Elliptic Curve Cryptography" by Hankerson et al.**: Comprehensive technical reference
- **"Serious Cryptography" by Jean-Philippe Aumasson**: Modern cryptography overview

### Online Resources

- [A (Relatively Easy To Understand) Primer on Elliptic Curve Cryptography](https://blog.cloudflare.com/a-relatively-easy-to-understand-primer-on-elliptic-curve-cryptography/)
- [Elliptic Curve Cryptography: a gentle introduction](https://andrea.corbellini.name/2015/05/17/elliptic-curve-cryptography-a-gentle-introduction/)
- [Bitcoin's Use of ECDSA](https://en.bitcoin.it/wiki/Elliptic_Curve_Digital_Signature_Algorithm)
- [RFC 6979: Deterministic ECDSA](https://tools.ietf.org/html/rfc6979)
- [NIST Curves](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.186-4.pdf)

### Mathematical Prerequisites

To fully understand this implementation, you should be familiar with:

1. **Modular Arithmetic**: Operations modulo a prime
2. **Group Theory**: Groups, cyclic groups, generators
3. **Abstract Algebra**: Fields, especially finite fields
4. **Number Theory**: Fermat's Little Theorem, discrete logarithm problem

### Learning Path

1. **Start Here**: Understand finite field arithmetic (`FiniteField` struct)
2. **Geometric Intuition**: Study elliptic curve point addition visually
3. **Scalar Multiplication**: Understand the double-and-add algorithm
4. **ECDSA Theory**: Learn why verification works mathematically
5. **Implementation**: Study the code with the mathematical background

## 🎯 Key Concepts

### The Discrete Logarithm Problem

ECDSA security relies on the computational hardness of:

**Given**: $Q = d \cdot G$ (public key)  
**Find**: $d$ (private key)

This is called the **Elliptic Curve Discrete Logarithm Problem (ECDLP)** and is believed to be computationally infeasible for large curves.

### Why Elliptic Curves?

1. **Smaller Keys**: 256-bit EC ≈ 3072-bit RSA security
2. **Efficient**: Faster operations than RSA
3. **Mobile-Friendly**: Lower power consumption
4. **Future-Proof**: Better resistance to quantum attacks than RSA (though still vulnerable)

### Common Curves

- **secp256k1**: Used by Bitcoin, Ethereum
  - $y^2 = x^3 + 7$ over a 256-bit field
  
- **P-256 (secp256r1)**: NIST standard
  - Used in TLS, government applications
  
- **Curve25519**: Modern alternative
  - Used in Signal, SSH, TLS 1.3

## 🤝 Contributing

Contributions are welcome! This is an educational project, so clarity and documentation are priorities.

### How to Contribute

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Update documentation
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Contribution Ideas

- Add support for standard curves (secp256k1, P-256)
- Implement deterministic $k$ generation (RFC 6979)
- Add more comprehensive examples
- Improve documentation
- Add visualization tools
- Performance optimizations
- Constant-time operations (for educational purposes)

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- The Rust community for excellent cryptographic libraries that inspired this implementation
- Cloudflare's blog posts on elliptic curve cryptography
- Andrea Corbellini's excellent ECC tutorial series
- The secp256k1 specification authors

## ⚡ Quick Reference

### Key Formulas

| Operation | Formula |
|-----------|---------|
| Field Addition | $(a + b) \bmod p$ |
| Field Multiplication | $(a \times b) \bmod p$ |
| Field Division | $a \times b^{p-2} \bmod p$ |
| Point Addition (different) | $s = \frac{y_2-y_1}{x_2-x_1}$, $x_3 = s^2-x_1-x_2$ |
| Point Doubling | $s = \frac{3x^2+a}{2y}$, $x_3 = s^2-2x$ |
| ECDSA Sign | $s = k^{-1}(z + rd) \bmod n$ |
| ECDSA Verify | Check if $(u_1 G + u_2 Q)_x \equiv r \pmod{n}$ |

### Common Commands

```bash
# Build
cargo build

# Run demo
cargo run

# Run tests
cargo test

# Run specific test
cargo test test_sign_verify

# Build documentation
cargo doc --open

# Check code
cargo clippy
```

---

**Made with ❤️ for learning cryptography**

*"In mathematics, you don't understand things. You just get used to them." - John von Neumann*