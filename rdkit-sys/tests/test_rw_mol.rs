use cxx::{let_cxx_string, SharedPtr};
use rdkit_sys::{ro_mol_ffi::ROMol, rw_mol_ffi::RWMol};

#[test]
fn test_rw_mol_from_smarts() {
    cxx::let_cxx_string!(smarts = "[+1!h0!$([*]~[-1,-2,-3,-4]),-1!$([*]~[+1,+2,+3,+4])]");
    let rwmol = rdkit_sys::rw_mol_ffi::smarts_to_mol(&smarts).unwrap();
    let romol = rdkit_sys::rw_mol_ffi::rw_mol_to_ro_mol(rwmol);

    let smiles = rdkit_sys::ro_mol_ffi::mol_to_smiles(&romol);
    assert_eq!(smiles, "*".to_string());
}

#[test]
fn test_rw_mol_from_mol_block() {
    let mol_block = r#"1
  -OEChem-05172223082D

 31 30  0     1  0  0  0  0  0999 V2000
    2.8660    0.7500    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    2.8660   -2.2500    0.0000 O   0  5  0  0  0  0  0  0  0  0  0  0
    2.0000   -0.7500    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    3.7320    2.2500    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    5.4641    0.2500    0.0000 N   0  3  0  0  0  0  0  0  0  0  0  0
    4.5981    0.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    3.7320    0.2500    0.0000 C   0  0  3  0  0  0  0  0  0  0  0  0
    6.3301   -0.2500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    5.9641    1.1160    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    4.9641   -0.6160    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    3.7320   -0.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    2.8660   -1.2500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    2.8660    1.7500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    2.0000    2.2500    0.0000 C   0  0  0  0  0  0  0  0  0  0  0  0
    4.9966    1.2250    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.1996    1.2250    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    3.7320    0.8700    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.0201   -0.7869    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.8671   -0.5600    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.6401    0.2869    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.5010    0.8060    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    6.2741    1.6530    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    5.4272    1.4260    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.4272   -0.3060    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.6541   -1.1530    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    5.5010   -0.9260    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    3.9441   -1.3326    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    4.3426   -0.6423    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    2.3100    2.7869    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    1.4631    2.5600    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
    1.6900    1.7131    0.0000 H   0  0  0  0  0  0  0  0  0  0  0  0
  1  7  1  0  0  0  0
  1 13  1  0  0  0  0
  2 12  1  0  0  0  0
  3 12  2  0  0  0  0
  4 13  2  0  0  0  0
  5  6  1  0  0  0  0
  5  8  1  0  0  0  0
  5  9  1  0  0  0  0
  5 10  1  0  0  0  0
  6  7  1  0  0  0  0
  6 15  1  0  0  0  0
  6 16  1  0  0  0  0
  7 11  1  0  0  0  0
  7 17  1  0  0  0  0
  8 18  1  0  0  0  0
  8 19  1  0  0  0  0
  8 20  1  0  0  0  0
  9 21  1  0  0  0  0
  9 22  1  0  0  0  0
  9 23  1  0  0  0  0
 10 24  1  0  0  0  0
 10 25  1  0  0  0  0
 10 26  1  0  0  0  0
 11 12  1  0  0  0  0
 11 27  1  0  0  0  0
 11 28  1  0  0  0  0
 13 14  1  0  0  0  0
 14 29  1  0  0  0  0
 14 30  1  0  0  0  0
 14 31  1  0  0  0  0
M  CHG  2   2  -1   5   1
M  END
> <PUBCHEM_COMPOUND_CID>
1

> <PUBCHEM_COMPOUND_CANONICALIZED>
1

> <PUBCHEM_CACTVS_COMPLEXITY>
214

> <PUBCHEM_CACTVS_HBOND_ACCEPTOR>
4

> <PUBCHEM_CACTVS_HBOND_DONOR>
0

> <PUBCHEM_CACTVS_ROTATABLE_BOND>
5

> <PUBCHEM_CACTVS_SUBSKEYS>
AAADceByOAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAHgAAAAAACBThgAYCCAMABAAIAACQCAAAAAAAAAAAAAEIAAACABQAgAAHAAAFIAAQAAAkAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==

> <PUBCHEM_IUPAC_OPENEYE_NAME>
3-acetoxy-4-(trimethylammonio)butanoate

> <PUBCHEM_IUPAC_CAS_NAME>
3-acetyloxy-4-(trimethylammonio)butanoate

> <PUBCHEM_IUPAC_NAME_MARKUP>
3-acetyloxy-4-(trimethylazaniumyl)butanoate

> <PUBCHEM_IUPAC_NAME>
3-acetyloxy-4-(trimethylazaniumyl)butanoate

> <PUBCHEM_IUPAC_SYSTEMATIC_NAME>
3-acetyloxy-4-(trimethylazaniumyl)butanoate

> <PUBCHEM_IUPAC_TRADITIONAL_NAME>
3-acetoxy-4-(trimethylammonio)butyrate

> <PUBCHEM_IUPAC_INCHI>
InChI=1S/C9H17NO4/c1-7(11)14-8(5-9(12)13)6-10(2,3)4/h8H,5-6H2,1-4H3

> <PUBCHEM_IUPAC_INCHIKEY>
RDHQFKQIGNGIED-UHFFFAOYSA-N

> <PUBCHEM_XLOGP3_AA>
0.4

> <PUBCHEM_EXACT_MASS>
203.11575802

> <PUBCHEM_MOLECULAR_FORMULA>
C9H17NO4

> <PUBCHEM_MOLECULAR_WEIGHT>
203.24

> <PUBCHEM_OPENEYE_CAN_SMILES>
CC(=O)OC(CC(=O)[O-])C[N+](C)(C)C

> <PUBCHEM_OPENEYE_ISO_SMILES>
CC(=O)OC(CC(=O)[O-])C[N+](C)(C)C

> <PUBCHEM_CACTVS_TPSA>
66.4

> <PUBCHEM_MONOISOTOPIC_WEIGHT>
203.11575802

> <PUBCHEM_TOTAL_CHARGE>
0

> <PUBCHEM_HEAVY_ATOM_COUNT>
14

> <PUBCHEM_ATOM_DEF_STEREO_COUNT>
0

> <PUBCHEM_ATOM_UDEF_STEREO_COUNT>
1

> <PUBCHEM_BOND_DEF_STEREO_COUNT>
0

> <PUBCHEM_BOND_UDEF_STEREO_COUNT>
0

> <PUBCHEM_ISOTOPIC_ATOM_COUNT>
0

> <PUBCHEM_COMPONENT_COUNT>
1

> <PUBCHEM_CACTVS_TAUTO_COUNT>
1

> <PUBCHEM_COORDINATE_TYPE>
1
5
255

> <PUBCHEM_BONDANNOTATIONS>
7  11  3
"#;

    let_cxx_string!(mol_block = mol_block);

    let rw_mol = rdkit_sys::rw_mol_ffi::rw_mol_from_mol_block(&mol_block, false, false, false);
    let ro_mol = unsafe { std::mem::transmute::<SharedPtr<RWMol>, SharedPtr<ROMol>>(rw_mol) };

    let smiles = rdkit_sys::ro_mol_ffi::mol_to_smiles(&ro_mol);
    assert_eq!("[H]C([H])([H])C(=O)OC([H])(C([H])([H])C(=O)[O-])C([H])([H])[N+](C([H])([H])[H])(C([H])([H])[H])C([H])([H])[H]", &smiles);
}

#[test]
fn test_bad_mol_block() {
    let bad = r#"24258
  -OEChem-02132200252D

  5  4  0     0  0  0  0  0  0999 V2000
    2.8660    0.0000    0.0000 Cl  0  0  0  0  0  0  0  0  0  0  0  0
    3.7320    0.5000    0.0000 F   0  0  0  0  0  0  0  0  0  0  0  0
    2.0000   -0.5000    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    2.3660    0.8660    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
    3.3660   -0.8660    0.0000 O   0  0  0  0  0  0  0  0  0  0  0  0
  1  2  1  0  0  0  0
  1  3  2  0  0  0  0
  1  4  2  0  0  0  0
  1  5  2  0  0  0  0
M  END
> <PUBCHEM_COMPOUND_CID>
24258

> <PUBCHEM_COMPOUND_CANONICALIZED>
1

> <PUBCHEM_CACTVS_COMPLEXITY>
117

> <PUBCHEM_CACTVS_HBOND_ACCEPTOR>
4

> <PUBCHEM_CACTVS_HBOND_DONOR>
0

> <PUBCHEM_CACTVS_ROTATABLE_BOND>
0

> <PUBCHEM_CACTVS_SUBSKEYS>
AAADcQAAMQAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA==

> <PUBCHEM_IUPAC_OPENEYE_NAME>
perchloryl fluoride

> <PUBCHEM_IUPAC_CAS_NAME>
perchloryl fluoride

> <PUBCHEM_IUPAC_NAME_MARKUP>
perchloryl fluoride

> <PUBCHEM_IUPAC_NAME>
perchloryl fluoride

> <PUBCHEM_IUPAC_SYSTEMATIC_NAME>
perchloryl fluoride

> <PUBCHEM_IUPAC_TRADITIONAL_NAME>
perchloryl fluoride

> <PUBCHEM_IUPAC_INCHI>
InChI=1S/ClFO3/c2-1(3,4)5

> <PUBCHEM_IUPAC_INCHIKEY>
XHFXMNZYIKFCPN-UHFFFAOYSA-N

> <PUBCHEM_XLOGP3_AA>
3.3

> <PUBCHEM_EXACT_MASS>
101.9519997

> <PUBCHEM_MOLECULAR_FORMULA>
ClFO3

> <PUBCHEM_MOLECULAR_WEIGHT>
102.45

> <PUBCHEM_OPENEYE_CAN_SMILES>
O=Cl(=O)(=O)F

> <PUBCHEM_OPENEYE_ISO_SMILES>
O=Cl(=O)(=O)F

> <PUBCHEM_CACTVS_TPSA>
51.2

> <PUBCHEM_MONOISOTOPIC_WEIGHT>
101.9519997

> <PUBCHEM_TOTAL_CHARGE>
0

> <PUBCHEM_HEAVY_ATOM_COUNT>
5

> <PUBCHEM_ATOM_DEF_STEREO_COUNT>
0

> <PUBCHEM_ATOM_UDEF_STEREO_COUNT>
0

> <PUBCHEM_BOND_DEF_STEREO_COUNT>
0

> <PUBCHEM_BOND_UDEF_STEREO_COUNT>
0

> <PUBCHEM_ISOTOPIC_ATOM_COUNT>
0

> <PUBCHEM_COMPONENT_COUNT>
1

> <PUBCHEM_CACTVS_TAUTO_COUNT>
1

> <PUBCHEM_COORDINATE_TYPE>
1
5
255

$"#;
    let_cxx_string!(bad = bad);

    let mol = {
        let mut mol = rdkit_sys::rw_mol_ffi::rw_mol_from_mol_block(&bad, true, false, false);
        if mol.is_null() {
            mol = rdkit_sys::rw_mol_ffi::rw_mol_from_mol_block(&bad, false, false, false);
        }
        mol
    };
    assert!(!mol.is_null());
    // println!("{}", mol.as_smile());
}

#[test]
fn inchi_to_mol_test() {
    let inchi = "InChI=1S/H2O/h1H2";
    let_cxx_string!(inchi = inchi);
    let rw_mol = rdkit_sys::rw_mol_ffi::inchi_to_mol(&inchi, true, true).unwrap();
    assert!(!rw_mol.is_null());
}