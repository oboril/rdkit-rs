use std::collections::HashMap;

use cxx::SharedPtr;

use crate::ROMol;

pub struct MoleculeProperties {
    pub exact_mw: f64,
    pub amw: f64,
    pub lipinski_hba: usize,
    pub lipinski_hbd: usize,
    pub num_rotatable_bonds: usize,
    pub num_hbd: usize,
    pub num_hba: usize,
    pub num_heavy_atoms: usize,
    pub num_atoms: usize,
    pub num_heteroatoms: usize,
    pub num_amide_bonds: usize,
    pub fraction_c_sp3: f64,
    pub num_rings: usize,
    pub num_aromatic_rings: usize,
    pub num_aliphatic_rings: usize,
    pub num_saturated_rings: usize,
    pub num_heterocycles: usize,
    pub num_aromatic_heterocycles: usize,
    pub num_saturated_heterocycles: usize,
    pub num_aliphatic_heterocycles: usize,
    pub num_spiro_atoms: usize,
    pub num_bridgehead_atoms: usize,
    pub num_atom_stereo_centers: usize,
    pub num_unspecified_atom_stereo_centers: usize,
    pub labute_asa: f64,
    pub tpsa: f64,
    pub crippen_clog_p: f64,
    pub crippen_mr: f64,
    pub chi0v: f64,
    pub chi1v: f64,
    pub chi2v: f64,
    pub chi3v: f64,
    pub chi4v: f64,
    pub chi0n: f64,
    pub chi1n: f64,
    pub chi2n: f64,
    pub chi3n: f64,
    pub chi4n: f64,
    pub hall_kier_alpha: f64,
    pub kappa1: f64,
    pub kappa2: f64,
    pub kappa3: f64,
    pub phi: f64,
}

impl MoleculeProperties {
    pub fn from_molecule(ro_mol: &ROMol) -> Self {
        let props = Properties::new();
        let values = rdkit_sys::descriptors_ffi::compute_properties(&props.ptr, &ro_mol.ptr);
        let values_slice = values.as_slice();
        assert_eq!(values.len(), 43, "expected 43 descriptors, got {}", values.len());


        Self {
            exact_mw: values_slice[0],
            amw: values_slice[1],
            lipinski_hba: values_slice[2].round() as usize,
            lipinski_hbd: values_slice[3].round() as usize,
            num_rotatable_bonds: values_slice[4].round() as usize,
            num_hbd: values_slice[5].round() as usize,
            num_hba: values_slice[6].round() as usize,
            num_heavy_atoms: values_slice[7].round() as usize,
            num_atoms: values_slice[8].round() as usize,
            num_heteroatoms: values_slice[9].round() as usize,
            num_amide_bonds: values_slice[10].round() as usize,
            fraction_c_sp3: values_slice[11],
            num_rings: values_slice[12].round() as usize,
            num_aromatic_rings: values_slice[13].round() as usize,
            num_aliphatic_rings: values_slice[14].round() as usize,
            num_saturated_rings: values_slice[15].round() as usize,
            num_heterocycles: values_slice[16].round() as usize,
            num_aromatic_heterocycles: values_slice[17].round() as usize,
            num_saturated_heterocycles: values_slice[18].round() as usize,
            num_aliphatic_heterocycles: values_slice[19].round() as usize,
            num_spiro_atoms: values_slice[20].round() as usize,
            num_bridgehead_atoms: values_slice[21].round() as usize,
            num_atom_stereo_centers: values_slice[22].round() as usize,
            num_unspecified_atom_stereo_centers: values_slice[23].round() as usize,
            labute_asa: values_slice[24],
            tpsa: values_slice[25],
            crippen_clog_p: values_slice[26],
            crippen_mr: values_slice[27],
            chi0v: values_slice[28],
            chi1v: values_slice[29],
            chi2v: values_slice[30],
            chi3v: values_slice[31],
            chi4v: values_slice[32],
            chi0n: values_slice[33],
            chi1n: values_slice[34],
            chi2n: values_slice[35],
            chi3n: values_slice[36],
            chi4n: values_slice[37],
            hall_kier_alpha: values_slice[38],
            kappa1: values_slice[39],
            kappa2: values_slice[40],
            kappa3: values_slice[41],
            phi: values_slice[42],
        }
    }
}

pub struct Properties {
    ptr: SharedPtr<rdkit_sys::descriptors_ffi::Properties>,
}

impl Default for Properties {
    fn default() -> Self {
        Properties::new()
    }
}

impl Properties {
    pub fn new() -> Self {
        Properties {
            ptr: rdkit_sys::descriptors_ffi::new_properties(),
        }
    }

    pub fn compute_properties(&self, ro_mol: &ROMol) -> HashMap<String, f64> {
        let names = rdkit_sys::descriptors_ffi::get_property_names(&self.ptr);
        let computed = rdkit_sys::descriptors_ffi::compute_properties(&self.ptr, &ro_mol.ptr);

        assert!(names.len() != 0);
        assert!(computed.len() == names.len());

        names
            .into_iter()
            .zip(computed.as_slice())
            .map(|(k, v)| (k.to_string(), *v))
            .collect()
    }
}
