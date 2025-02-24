# Super simple Elliptic Curve Presentation. No imported libraries, wrappers, nothing.
# For educational purposes only.

# Public specs for Bitcoin's secp256k1 curve
Pcurve = 2**256 - 2**32 - 2**9 - 2**8 - 2**7 - 2**6 - 2**4 - 1  # Prime field
N = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141  # Order of the curve
Acurve = 0
Bcurve = 7  # Curve equation: y^2 = x^3 + Acurve * x + Bcurve
Gx = 55066263022277343669578718895168534326250603453777594175500187360389116729240
Gy = 32670510020758816978083085130507043184471273380659243275938904335757337482424
GPoint = (Gx, Gy)  # Generator point

# Replace with any private key
privKey = 0xA0DC65FFCA799873CBEA0AC274015B9526505DAAAED385155425F7337704883E

def modinv(a, n=Pcurve):  # Extended Euclidean Algorithm
    lm, hm = 1, 0
    low, high = a % n, n
    while low > 1:
        ratio = high // low  # Integer division
        nm, new = hm - lm * ratio, high - low * ratio
        hm, high = lm, low  # Update before overwriting
        lm, low = nm, new  # Update before overwriting
    return lm % n  # Ensure positive result

def modinv_test(a, n=Pcurve):  # Extended Euclidean Algorithm
    lm, hm = 1, 0
    low, high = a % n, n
    counter = 1
    while low > 1:
        ratio = high // low  # Integer division
        print(f"Step {counter}, hm: {hm}, lm: {lm}")
        nm, new = hm - lm * ratio, high - low * ratio
        hm, high = lm, low  # Update before overwriting
        lm, low = nm, new  # Update before overwriting
        print(f"Step {counter} ratio = {ratio}, nm = {nm}")
        counter += 1
    return lm % n  # Ensure positive result


def ECadd(a, b):  # Elliptic curve point addition
    LamAdd = ((b[1] - a[1]) * modinv(b[0] - a[0], Pcurve)) % Pcurve
    x = (LamAdd * LamAdd - a[0] - b[0]) % Pcurve
    y = (LamAdd * (a[0] - x) - a[1]) % Pcurve
    return (x, y)

def ECdouble(a):  # Elliptic curve point doubling
    Lam = ((3 * a[0] * a[0] + Acurve) * modinv(2 * a[1], Pcurve)) % Pcurve
    x = (Lam * Lam - 2 * a[0]) % Pcurve
    y = (Lam * (a[0] - x) - a[1]) % Pcurve
    return (x, y)

def EccMultiply(GenPoint, ScalarHex):  # Scalar multiplication (Double & Add)
    if ScalarHex == 0 or ScalarHex >= N:
        raise Exception("Invalid Scalar/Private Key")
    ScalarBin = bin(ScalarHex)[2:]  # Convert to binary string
    print(f"Binary scalar: {ScalarBin}")
    Q = GenPoint
    print("Point q: ", Q)
    for i in range(1, len(ScalarBin)):  # Double & add method
        Q = ECdouble(Q)
        print(f"Step {i} after double q = {Q}")
        if ScalarBin[i] == "1":
            Q = ECadd(Q, GenPoint)
            print(f"Step {i} after add q = {Q}")
    return Q

print("\n******* Public Key Generation *********\n")
PublicKey = EccMultiply(GPoint, privKey)

print("The private key:")
print(hex(privKey), "\n")

print("The uncompressed public key (not address):")
print(PublicKey, "\n")

print("The uncompressed public key (HEX):")
print(f"04{PublicKey[0]:064x}{PublicKey[1]:064x}\n")

print("The official Public Key - compressed:")
if PublicKey[1] % 2 == 1:  # If Y coordinate is odd
    print(f"03{PublicKey[0]:064x}")
else:  # If Y coordinate is even
    print(f"02{PublicKey[0]:064x}")
