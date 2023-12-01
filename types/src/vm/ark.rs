use super::MpcType;
use ark_bls12_381::Fr;

impl MpcType for Fr {}

#[cfg(test)]
mod test {
    use ark_bls12_381::Fr;

    #[test]
    fn sec_int_new() {
        let _secret_int = Fr::new(42_u64.into());
    }

    #[test]
    fn sec_int_add() {
        let secret_int_1 = Fr::new(42_u64.into());
        let secret_int_2 = Fr::new(2_u64.into());

        assert_eq!(secret_int_1 + secret_int_2, Fr::new(44_u64.into()));
    }

    #[test]
    fn sec_int_multiply() {
        let secret_int_1 = Fr::from(42_u64);
        let secret_int_2 = Fr::from(2_u64);

        assert_eq!((secret_int_1 * secret_int_2), Fr::from(84_u64));
    }

    #[test]
    fn sec_int_display() {
        let secret_int = Fr::from(42_u64);

        assert_eq!(
            secret_int.to_string(),
            "Fp256 \"(000000000000000000000000000000000000000000000000000000000000002A)\""
        );
    }
}
