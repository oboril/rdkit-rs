use cxx::SharedPtr;
use rdkit_sys::substruct_match_ffi::new_substruct_match_parameters;

pub struct SubstructMatchParameters {
    pub ptr: SharedPtr<rdkit_sys::substruct_match_ffi::SubstructMatchParameters>,
}

impl Default for SubstructMatchParameters {
    fn default() -> Self {
        SubstructMatchParameters::new()
    }
}

#[allow(dead_code)]
impl SubstructMatchParameters {
    pub fn new() -> Self {
        let ptr = new_substruct_match_parameters();

        SubstructMatchParameters { ptr }
    }

    pub fn get_use_chirality(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_use_chirality(&self.ptr)
    }

    pub fn get_use_enhanced_stereo(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_use_enhanced_stereo(&self.ptr)
    }

    pub fn get_aromatic_matches_conjugated(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_aromatic_matches_conjugated(&self.ptr)
    }

    pub fn get_use_query_query_matches(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_use_query_query_matches(&self.ptr)
    }

    pub fn get_use_generic_matchers(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_use_generic_matchers(&self.ptr)
    }

    pub fn get_recursion_possible(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_recursion_possible(&self.ptr)
    }

    pub fn get_uniquify(&self) -> bool {
        rdkit_sys::substruct_match_ffi::get_uniquify(&self.ptr)
    }

    pub fn use_chirality(mut self, what: bool) -> SubstructMatchParameters{
        rdkit_sys::substruct_match_ffi::set_use_chirality(&mut self.ptr, what);
        self
    }

    pub fn use_enhanced_stereo(mut self, what: bool) -> SubstructMatchParameters{
        rdkit_sys::substruct_match_ffi::set_use_enhanced_stereo(&mut self.ptr, what);
        self
    }

    pub fn aromatic_matches_conjugated(mut self, what: bool) -> SubstructMatchParameters{
        rdkit_sys::substruct_match_ffi::set_aromatic_matches_conjugated(&mut self.ptr, what);
        self
    }

    pub fn use_query_query_matches(mut self, what: bool) -> SubstructMatchParameters{
        rdkit_sys::substruct_match_ffi::set_use_query_query_matches(&mut self.ptr, what);
        self
    }

    pub fn use_generic_matchers(mut self, what: bool) -> SubstructMatchParameters{
        rdkit_sys::substruct_match_ffi::set_use_generic_matchers(&mut self.ptr, what);
        self
    }

    pub fn recursion_possible(mut self, what: bool) -> SubstructMatchParameters{
        rdkit_sys::substruct_match_ffi::set_recursion_possible(&mut self.ptr, what);
        self
    }

    pub fn uniquify(mut self, what: bool) -> SubstructMatchParameters{
        rdkit_sys::substruct_match_ffi::set_uniquify(&mut self.ptr, what);
        self
    }

    pub fn max_matches(mut self, value: u32) -> SubstructMatchParameters {
        rdkit_sys::substruct_match_ffi::set_max_matches(&mut self.ptr, value);
        self
    }
}
