use lambdaworks_math::elliptic_curve::short_weierstrass::curves::bls12_381::curve::BLS12381Curve;
use lambdaworks_math::elliptic_curve::traits::IsEllipticCurve;
use lambdaworks_math::cyclic_group::IsGroup;
use lambdaworks_math::unsigned_integer::element::U256;
use lambdaworks_math::traits::ByteConversion;
use lambdaworks_math::traits::AsBytes;

fn main() {
    let g = BLS12381Curve::generator();
    let priv_key = U256::from_hex_unchecked("6C616D6264617370");
    let pub_key = g.operate_with_self(priv_key);
    let pub_key_256 = U256::from_bytes_be(&pub_key.as_bytes()).expect("failed");
    println!("public key {:?}", pub_key_256.to_hex());
}
