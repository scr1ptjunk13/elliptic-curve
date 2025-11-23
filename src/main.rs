use elliptic_curve::{EllipticCurve, Point, ecdsa::ECDSA};
use num_bigint::BigUint;

fn main() {
    println!("=== ECDSA Demonstration ===\n");

    // Create a small elliptic curve for demonstration
    // Curve: y² = x³ + 2x + 2 (mod 17)
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

    // Order of the generator
    let order = BigUint::from(19u32);

    println!("Curve parameters:");
    println!("  y² = x³ + {}x + {} (mod {})", curve.a, curve.b, curve.p);
    println!("  Generator: {:?}", generator);
    println!("  Order: {}\n", order);

    // Create ECDSA instance
    let ecdsa = ECDSA::new(curve, generator, order);

    // Generate keypair
    println!("Generating keypair...");
    let keypair = ecdsa.generate_keypair();
    println!("  Private key: {}", keypair.private_key);
    println!("  Public key: {:?}\n", keypair.public_key);

    // Message to sign
    let message = b"Hello, ECDSA!";
    println!("Message: {:?}", String::from_utf8_lossy(message));

    // Sign the message
    println!("\nSigning message...");
    match ecdsa.sign(message, &keypair.private_key) {
        Ok(signature) => {
            println!("  Signature r: {}", signature.r);
            println!("  Signature s: {}", signature.s);

            // Verify the signature
            println!("\nVerifying signature with correct message...");
            let is_valid = ecdsa.verify(message, &signature, &keypair.public_key);
            println!("  Valid: {}", is_valid);

            // Try with wrong message
            let wrong_message = b"Wrong message!";
            println!("\nVerifying signature with wrong message...");
            let is_valid = ecdsa.verify(wrong_message, &signature, &keypair.public_key);
            println!("  Valid: {}", is_valid);

            // Demonstrate scalar multiplication
            println!("\n=== Elliptic Curve Operations ===");
            let point = Point::Coordinate(BigUint::from(5u32), BigUint::from(1u32));
            println!("Original point: {:?}", point);
            
            let doubled = ecdsa.curve.double(&point);
            println!("2P (doubled): {:?}", doubled);
            
            let tripled = ecdsa.curve.scalar_mult(&point, &BigUint::from(3u32));
            println!("3P: {:?}", tripled);
            
            let five_p = ecdsa.curve.scalar_mult(&point, &BigUint::from(5u32));
            println!("5P: {:?}", five_p);
        }
        Err(e) => {
            println!("Error signing: {}", e);
        }
    }

    println!("\n=== Demo Complete ===");
}