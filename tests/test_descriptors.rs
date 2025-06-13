use std::collections::HashMap;

use rdkit::{Properties, ROMol};

#[test]
fn test_a_thing() {
    let mol = ROMol::from_smiles("c1ccccc1C(=O)NC").unwrap();
    let properties = Properties::new();
    let computed: HashMap<String, f64> = properties.compute_properties(&mol);
    assert_eq!(*computed.get("NumAtoms").unwrap(), 19.0);
}

#[test]
fn test_computing_properties() {
    let smiles = "CCOC(=O)C(C)(C)OC1=CC=C(C=C1)Cl.CO.C1=CC(=CC=C1C(=O)N[C@@H](CCC(=O)O)C(=O)O)NCC2=CN=C3C(=N2)C(=O)NC(=N3)N";
    let romol = ROMol::from_smiles(smiles).unwrap();

    let properties = Properties::new();

    let computed = properties.compute_properties(&romol);

    let expected = vec![
        ("NumAtoms", 88.0),
        ("chi1n", 14.854760794353165),
        ("chi2n", 6.31077807123148),
        ("kappa3", 12.661143750192025),
        ("NumAtomStereoCenters", 1.0),
        ("NumSaturatedRings", 0.0),
        ("CrippenMR", 182.82939999999977),
        ("NumAliphaticRings", 0.0),
        ("chi0v", 28.35419311110654),
        ("NumHeterocycles", 2.0),
        ("tpsa", 269.03999999999996),
        ("lipinskiHBA", 17.0),
        ("NumRings", 4.0),
        ("chi1v", 15.232725267362392),
        ("chi2v", 6.5627543865709645),
        ("amw", 716.1479999999998),
        ("NumAliphaticHeterocycles", 0.0),
        ("chi0n", 27.598264165088082),
        ("chi4n", 3.8988182639091344),
        ("NumSaturatedHeterocycles", 0.0),
        ("chi3v", 6.5627543865709645),
        ("chi3n", 6.31077807123148),
        ("FractionCSP3", 0.3125),
        ("NumBridgeheadAtoms", 0.0),
        ("kappa1", 40.36507158313297),
        ("kappa2", 19.253316231719992),
        ("Phi", 15.54322975812147),
        ("lipinskiHBD", 8.0),
        ("exactmw", 715.2368680959999),
        ("NumRotatableBonds", 13.0),
        ("NumAromaticHeterocycles", 2.0),
        ("CrippenClogP", 2.6242),
        ("NumHBD", 7.0),
        ("NumHeavyAtoms", 50.0),
        ("NumHBA", 13.0),
        ("NumAmideBonds", 1.0),
        ("NumAromaticRings", 4.0),
        ("NumUnspecifiedAtomStereoCenters", 0.0),
        ("chi4v", 4.024806421578876),
        ("NumHeteroatoms", 18.0),
        ("NumSpiroAtoms", 0.0),
        ("hallKierAlpha", -5.8100000000000005),
        ("labuteASA", 290.3869834026883),
    ];

    let mut expected = expected
        .iter()
        .map(|(k, v)| (k.to_string(), format!("{:.3}", *v)))
        .collect::<Vec<_>>();
    let mut computed = computed
        .into_iter()
        .map(|(k, v)| (k.to_string(), format!("{:.3}", v)))
        .collect::<Vec<_>>();

    expected.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
    computed.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

    assert_eq!(expected, computed);
}

#[test]
fn test_molecule_properties() {
    let mol = ROMol::from_smiles("CCOC(=O)C(C)(C)OC1=CC=C(C=C1)Cl.CO.C1=CC(=CC=C1C(=O)N[C@@H](CCC(=O)O)C(=O)O)NCC2=CN=C3C(=N2)C(=O)NC(=N3)N").unwrap();
    let props = mol.properties();

    assert_eq!(props.num_atoms, 88);
    assert_eq!(props.chi1n, 14.854760794353165);
    assert_eq!(props.chi2n, 6.31077807123148);
    assert_eq!(props.kappa3, 12.661143750192025);
    assert_eq!(props.num_atom_stereo_centers, 1);
    assert_eq!(props.num_saturated_rings, 0);
    assert_eq!(props.crippen_mr, 182.82939999999977);
    assert_eq!(props.num_aliphatic_rings, 0);
    assert_eq!(props.chi0v, 28.35419311110654);
    assert_eq!(props.num_heterocycles, 2);
    assert_eq!(props.tpsa, 269.03999999999996);
    assert_eq!(props.lipinski_hba, 17);
    assert_eq!(props.num_rings, 4);
    assert_eq!(props.chi1v, 15.232725267362392);
    assert_eq!(props.chi2v, 6.5627543865709645);
    assert_eq!(props.amw, 716.1479999999998);
    assert_eq!(props.num_aliphatic_heterocycles, 0);
    assert_eq!(props.chi0n, 27.598264165088082);
    assert_eq!(props.chi4n, 3.8988182639091344);
    assert_eq!(props.num_saturated_heterocycles, 0);
    assert_eq!(props.chi3v, 6.5627543865709645);
    assert_eq!(props.chi3n, 6.31077807123148);
    assert_eq!(props.fraction_c_sp3, 0.3125);
    assert_eq!(props.num_bridgehead_atoms, 0);
    assert_eq!(props.kappa1, 40.36507158313297);
    assert_eq!(props.kappa2, 19.253316231719992);
    assert_eq!(props.phi, 15.54322975812147);
    assert_eq!(props.lipinski_hbd, 8);
    assert_eq!(props.exact_mw, 715.2368680959999);
    assert_eq!(props.num_rotatable_bonds, 13);
    assert_eq!(props.num_aromatic_heterocycles, 2);
    assert_eq!(props.crippen_clog_p, 2.6242);
    assert_eq!(props.num_hbd, 7);
    assert_eq!(props.num_heavy_atoms, 50);
    assert_eq!(props.num_hba, 13);
    assert_eq!(props.num_amide_bonds, 1);
    assert_eq!(props.num_aromatic_rings, 4);
    assert_eq!(props.num_unspecified_atom_stereo_centers, 0);
    assert_eq!(props.chi4v, 4.024806421578876);
    assert_eq!(props.num_heteroatoms, 18);
    assert_eq!(props.num_spiro_atoms, 0);
    assert_eq!(props.hall_kier_alpha, -5.8100000000000005);
    assert_eq!(props.labute_asa, 290.3869834026883);
}