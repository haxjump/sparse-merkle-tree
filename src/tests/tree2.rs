use crate::{blake3_hasher::Blake3Hasher, error::Error, MerkleProof, VsSmt2, *};
use rand::prelude::{Rng, SliceRandom};

type Xid = [u8; 16];

const XID: Xid = [0; 16];
const XID1: Xid = [1; 16];
const XID2: Xid = [2; 16];

#[allow(clippy::upper_case_acronyms)]
type SMT = VsSmt2<Xid, H256>;

#[test]
fn test_default_root() {
    let tree = SMT::default();
    assert_eq!(tree.root(&XID), H256::zero());
}

#[test]
fn test_default_tree() {
    let tree = SMT::default();
    assert_eq!(tree.get(&XID, &H256::zero()).unwrap(), None);
    let proof = tree
        .merkle_proof(&XID, vec![H256::zero()])
        .expect("merkle proof");
    let root = proof
        .compute_root::<Blake3Hasher>(
            vec![(H256::zero(), H256::zero())]
                .into_iter()
                .map(|(k, v)| (k, Some(v)))
                .collect(),
        )
        .expect("root");
    assert_eq!(root, tree.root(&XID));
    let proof = tree
        .merkle_proof(&XID, vec![H256::zero()])
        .expect("merkle proof");
    let root2 = proof
        .compute_root::<Blake3Hasher>(
            vec![(H256::zero(), [42u8; 32].into())]
                .into_iter()
                .map(|(k, v)| (k, Some(v)))
                .collect(),
        )
        .expect("root");
    assert_ne!(root2, tree.root(&XID));
}

#[test]
fn test_default_merkle_proof() {
    let proof = MerkleProof::new(Default::default(), Default::default());
    let result = proof.compute_root::<Blake3Hasher>(
        vec![([42u8; 32].into(), [42u8; 32].into())]
            .into_iter()
            .map(|(k, v)| (k, Some(v)))
            .collect(),
    );
    assert_eq!(
        result.unwrap_err(),
        Error::IncorrectNumberOfLeaves {
            expected: 0,
            actual: 1
        }
    );

    // FIXME: makes room for leaves
    // let proof = MerkleProof::new(vec![Vec::new()], Default::default());
    // let root = proof
    //     .compute_root::<Blake3Hasher>(vec![([42u8; 32].into(), [42u8; 32].into())])
    //     .expect("compute root");
    // assert_ne!(root, H256::zero());
}

#[test]
fn test_merkle_root() {
    fn new_blake3() -> blake3::Hasher {
        Default::default()
    }

    let mut tree = SMT::default();
    for (i, word) in "The quick brown fox jumps over the lazy dog"
        .split_whitespace()
        .enumerate()
    {
        let key: H256 = {
            let mut buf = [0u8; 32];
            let mut hasher = new_blake3();
            hasher.update(&(i as u32).to_le_bytes());
            buf.copy_from_slice(hasher.finalize().as_bytes());
            buf.into()
        };
        let value: H256 = {
            let mut buf = [0u8; 32];
            let mut hasher = new_blake3();
            hasher.update(word.as_bytes());
            buf.copy_from_slice(hasher.finalize().as_bytes());
            buf.into()
        };
        tree.update(&XID, key, value).expect("update");
        tree.update(&XID1, key, value).expect("update");
        tree.update(&XID2, key, value).expect("update");
    }

    let expected_root: H256 = [
        121, 132, 252, 110, 162, 162, 63, 100, 12, 112, 190, 230, 177, 100, 54, 80, 95, 152, 72,
        29, 158, 97, 84, 117, 107, 2, 153, 97, 36, 38, 123, 84,
    ]
    .into();

    assert_eq!(tree.root(&XID), expected_root);
    assert_eq!(tree.root(&XID1), expected_root);
    assert_eq!(tree.root(&XID2), expected_root);

    tree.remove_x(&XID).unwrap();
    assert_eq!(tree.root(&XID), H256::zero());
    assert_eq!(tree.root(&XID1), expected_root);
    assert_eq!(tree.root(&XID2), expected_root);
    tree.remove_x(&XID1).unwrap();
    assert_eq!(tree.root(&XID), H256::zero());
    assert_eq!(tree.root(&XID1), H256::zero());
    assert_eq!(tree.root(&XID2), expected_root);
    tree.remove_x(&XID2).unwrap();
    assert_eq!(tree.root(&XID), H256::zero());
    assert_eq!(tree.root(&XID1), H256::zero());
    assert_eq!(tree.root(&XID2), H256::zero());
}

#[test]
fn test_zero_value_donot_change_root() {
    let mut tree = SMT::default();
    let key = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]
    .into();
    let value = H256::zero();
    tree.update(&XID, key, value).unwrap();
    assert_eq!(tree.root(&XID), H256::zero());
}

#[test]
fn test_zero_value_donot_change_store() {
    let mut tree = SMT::default();
    let key = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();
    let value = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]
    .into();
    tree.update(&XID, key, value).unwrap();
    assert_ne!(tree.root(&XID), H256::zero());
    let root = tree.root(&XID);

    // insert a zero value leaf
    let key = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]
    .into();
    let value = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();
    tree.update(&XID, key, value).unwrap();
    assert_eq!(tree.root(&XID), root);
}

#[test]
fn test_delete_a_leaf() {
    let mut tree = SMT::default();
    let key = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();
    let value = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]
    .into();
    tree.update(&XID, key, value).unwrap();
    assert_ne!(tree.root(&XID), H256::zero());
    let root = tree.root(&XID);

    // insert a leaf
    let key = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]
    .into();
    let value = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1,
    ]
    .into();
    tree.update(&XID, key, value).unwrap();
    assert_ne!(tree.root(&XID), root);

    // delete a leaf
    tree.update(&XID, key, H256::zero()).unwrap();
    assert_eq!(tree.root(&XID), root);
}

#[test]
fn test_sibling_key_get() {
    {
        let mut tree = SMT::default();
        let key = H256::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        let value = H256::from([1u8; 32]);
        tree.update(&XID, key, value).expect("update");

        let sibling_key = H256::from([
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        // get non exists sibling key should return zero value;
        assert_eq!(None, tree.get(&XID, &sibling_key).unwrap());
    }

    {
        let mut tree = SMT::default();
        let key = H256::from([
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        let value = H256::from([1u8; 32]);
        tree.update(&XID, key, value).expect("update");

        let sibling_key = H256::from([
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ]);
        let sibling_value = H256::from([2u8; 32]);
        tree.update(&XID, sibling_key, sibling_value)
            .expect("update");
        // get sibling key should return corresponding value
        assert_eq!(value, tree.get(&XID, &key).unwrap().unwrap());
        assert_eq!(
            sibling_value,
            tree.get(&XID, &sibling_key).unwrap().unwrap()
        );
    }
}

fn new_smt(pairs: Vec<(H256, H256)>) -> SMT {
    let mut smt = SMT::default();
    for (key, value) in pairs {
        smt.update(&XID, key, value).unwrap();
    }
    smt
}

fn parse_h256(s: &str) -> H256 {
    let data = hex::decode(s).unwrap();
    let mut inner = [0u8; 32];
    inner.copy_from_slice(&data);
    H256::from(inner)
}

#[test]
fn test_v0_2_broken_sample() {
    let keys = vec![
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000002",
        "0000000000000000000000000000000000000000000000000000000000000003",
        "0000000000000000000000000000000000000000000000000000000000000004",
        "0000000000000000000000000000000000000000000000000000000000000005",
        "0000000000000000000000000000000000000000000000000000000000000006",
        "000000000000000000000000000000000000000000000000000000000000000e",
        "f652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d3f",
        "f652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0d40",
        "5eff886ea0ce6ca488a3d6e336d6c0f75f46d19b42c06ce5ee98e42c96d256c7",
        "6d5257204ebe7d88fd91ae87941cb2dd9d8062b64ae5a2bd2d28ec40b9fbf6df",
    ]
    .into_iter()
    .map(parse_h256);

    let values = vec![
        "000000000000000000000000c8328aabcd9b9e8e64fbc566c4385c3bdeb219d7",
        "000000000000000000000001c8328aabcd9b9e8e64fbc566c4385c3bdeb219d7",
        "0000384000001c2000000e1000000708000002580000012c000000780000003c",
        "000000000000000000093a80000546000002a300000151800000e10000007080",
        "000000000000000000000000000000000000000000000000000000000000000f",
        "0000000000000000000000000000000000000000000000000000000000000001",
        "00000000000000000000000000000000000000000000000000071afd498d0000",
        "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000001",
        "0000000000000000000000000000000000000000000000000000000000000000",
    ]
    .into_iter()
    .map(parse_h256);

    let mut pairs = keys.zip(values).collect::<Vec<_>>();
    let smt = new_smt(pairs.clone());
    let base_root = smt.root(&XID);

    // insert in random order
    let mut rng = rand::thread_rng();
    for _i in 0..10 {
        pairs.shuffle(&mut rng);
        let smt = new_smt(pairs.clone());
        let current_root = smt.root(&XID);
        assert_eq!(base_root, current_root);
    }
}

#[test]
fn test_v0_3_broken_sample() {
    let k1 = [
        0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let v1 = [
        108, 153, 9, 238, 15, 28, 173, 182, 146, 77, 52, 203, 162, 151, 125, 76, 55, 176, 192, 104,
        170, 5, 193, 174, 137, 255, 169, 176, 132, 64, 199, 115,
    ];
    let k2 = [
        1, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let v2 = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let k3 = [
        1, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    let v3 = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];

    let mut smt = SMT::default();
    // inserted keys shouldn't interfere with each other
    assert_ne!(k1, k2);
    assert_ne!(k2, k3);
    assert_ne!(k1, k3);
    smt.update(&XID, k1.into(), v1.into()).unwrap();
    smt.update(&XID, k2.into(), v2.into()).unwrap();
    smt.update(&XID, k3.into(), v3.into()).unwrap();
    assert_eq!(smt.get(&XID, &k1.into()).unwrap().unwrap(), v1.into());
}

#[test]
fn test_replay_to_pass_proof() {
    let key1: H256 = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();
    let key2: H256 = [
        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();
    let key3: H256 = [
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();
    let key4: H256 = [
        4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();

    let existing: H256 = [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();
    let non_existing: H256 = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ]
    .into();
    let other_value: H256 = [
        0, 0, 0xff, 0, 0, 0, 0xff, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0xff,
    ]
    .into();
    let pairs = vec![
        (key1, existing),
        (key2, non_existing),
        (key3, non_existing),
        (key4, non_existing),
    ];
    let smt = new_smt(pairs);
    let leaf_a_bl = vec![(key1, H256::zero())];
    let leaf_c = vec![(key3, non_existing)];
    let leaf_other = vec![(key3, other_value)];
    let proofc = smt
        .merkle_proof(&XID, leaf_c.clone().into_iter().map(|(k, _)| k).collect())
        .expect("gen proof");
    let compiled_proof = proofc.clone().compile(vec![key3]).expect("compile proof");

    println!("verify ok case");
    assert!(proofc
        .clone()
        .verify::<Blake3Hasher>(
            smt.root(&XID),
            leaf_c.into_iter().map(|(k, v)| (k, Some(v))).collect()
        )
        .expect("verify"));
    println!("verify not ok case");
    assert!(!proofc
        .clone()
        .verify::<Blake3Hasher>(
            smt.root(&XID),
            leaf_other.into_iter().map(|(k, v)| (k, Some(v))).collect()
        )
        .expect("verify"));

    println!("merkle proof, leaf is faked");
    assert!(!proofc
        .verify::<Blake3Hasher>(
            smt.root(&XID),
            leaf_a_bl
                .clone()
                .into_iter()
                .map(|(k, v)| (k, Some(v)))
                .collect()
        )
        .expect("verify"));

    println!("compiled merkle proof, leaf is faked");
    assert!(!compiled_proof
        .verify::<Blake3Hasher>(
            smt.root(&XID),
            leaf_a_bl.into_iter().map(|(k, v)| (k, Some(v))).collect()
        )
        .expect("verify compiled proof"));
}

#[test]
fn test_sibling_leaf() {
    fn gen_rand_h256() -> H256 {
        let mut rng = rand::thread_rng();
        let rand_data: [u8; 32] = rng.gen();
        H256::from(rand_data)
    }
    let rand_key = gen_rand_h256();
    let mut sibling_key = rand_key;
    if rand_key.is_right(0) {
        sibling_key.clear_bit(0);
    } else {
        sibling_key.set_bit(0);
    }
    let pairs = vec![(rand_key, gen_rand_h256()), (sibling_key, gen_rand_h256())];
    let keys = vec![rand_key, sibling_key];
    let smt = new_smt(pairs.clone());
    let proof = smt.merkle_proof(&XID, keys).expect("gen proof");
    assert!(proof
        .verify::<Blake3Hasher>(
            smt.root(&XID),
            pairs.into_iter().map(|(k, v)| (k, Some(v))).collect()
        )
        .expect("verify"));
}

#[test]
fn test_max_stack_size() {
    fn gen_h256(height: u8) -> H256 {
        // The key path is first go right `256 - height` times then go left `height` times.
        let mut key = H256::zero();
        for h in height..=255 {
            key.set_bit(h);
        }
        key
    }
    let mut pairs: Vec<_> = (0..=255)
        .map(|height| (gen_h256(height), gen_h256(1)))
        .collect();
    // Most left key
    pairs.push((H256::zero(), gen_h256(1)));
    {
        // A pair of sibling keys in between
        let mut left_key = H256::zero();
        for h in 12..56 {
            left_key.set_bit(h);
        }
        let mut right_key = left_key;
        right_key.set_bit(0);
        pairs.push((left_key, gen_h256(1)));
        pairs.push((right_key, gen_h256(1)));
    }

    let keys: Vec<_> = pairs.iter().map(|(key, _)| *key).collect();
    let smt = new_smt(pairs.clone());
    let proof = smt.merkle_proof(&XID, keys.clone()).expect("gen proof");
    let compiled_proof = proof.compile(keys).expect("compile proof");
    assert!(compiled_proof
        .verify::<Blake3Hasher>(
            smt.root(&XID),
            pairs.into_iter().map(|(k, v)| (k, Some(v))).collect()
        )
        .expect("verify"));
}
