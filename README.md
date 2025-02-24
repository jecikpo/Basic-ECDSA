# Basic-ECDSA  
A basic **Elliptic Curve Digital Signature Algorithm (ECDSA)** implementation in **Rust** using the `BigInt` library. This project was created for educational purposes to practice **Elliptic Curve (EC) arithmetic** and deepen understanding of cryptographic concepts.  

## Features  
- **Pure Rust implementation** using `BigInt` for large integer arithmetic.  
- **Custom EC point operations** (addition, doubling, scalar multiplication).  
- **ECDSA key generation, signing, and verification** following the secp256k1 curve.  
- **No external cryptography libraries**—all operations are implemented manually for educational clarity.  

## Elliptic Curve Arithmetic

This implementation is based on the **secp256k1** elliptic curve, which is used in Bitcoin and other cryptocurrencies. The curve is defined by the equation:

$y^2=x^3+ax+b$

For **secp256k1**, the constants are:
- $\( a = 0 \)$
- $\( b = 7 \)$
- The prime modulus: $\( p = 2^{256} - 2^{32} - 2^9 - 2^8 - 2^7 - 2^6 - 2^4 -1 \)$
- Generator point $\( G = (G_x, G_y) \)$

### Point Addition

Given two points $\( P = (x_1, y_1) \)$ and $\( Q = (x_2, y_2) \)$, the addition formula is:

$\lambda = \frac{y_2 - y_1}{x_2 - x_1}$

$x_3 = \lambda^2 - x_1 - x_2$

$y_3 = \lambda (x_1 - x_3) - y_1$

### Point Doubling

When doubling a point $\( P = (x_1, y_1) \)$, the formulas simplify to:

$\lambda = \frac{3x_1^2 + a}{2y_1}$

$x_3 = \lambda^2 - 2x_1$

$y_3 = \lambda (x_1 - x_3) - y_1$

These operations form the basis of scalar multiplication, which is used for key generation and signing. We intentionally ommit the $\mod p$ for clarity.


## Getting Started  

### Prerequisites  
- Install **Rust** and **Cargo**:  
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Running the Project  
Clone the repository and run the implementation:  
```bash
git clone https://github.com/yourusername/Basic-ECDSA.git  
cd Basic-ECDSA  
cargo run  
```

### Running Tests  
To validate the implementation with unit tests:  
```bash
cargo test  
```

## Resources  

For a beginner-friendly introduction to **Elliptic Curve Cryptography (ECC)** and ECDSA, I highly recommend the following resources:  

- 📺 **Bitcoin 101 - ECC Part 4**: [Generating the Public Key (Python)](https://www.youtube.com/watch?v=iB3HcPgm_FI&ab_channel=CRI)  
- 📺 **Bitcoin 101 - ECC Part 5**: [Signing & Verifying a Message](https://youtu.be/U2bw_N6kQL8?si=HgtY4SnRl_SQznXD)  
- 📝 **Reference Code (Updated for Python 3)**: [Original ECC Scripts](https://github.com/wobine/blackboard101/blob/master/README.md)  

I have updated the original Python scripts from these videos to work with Python 3 and included them in this repository for reference.  

## Contributing  
This project is intended as a learning resource, but contributions and improvements are always welcome!  

---

If you have any questions, feel free to reach out! 🚀
