#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>

namespace RDKit {
    rust::String mol_to_inchi(std::shared_ptr<ROMol> mol);
    std::shared_ptr<ROMol> inchi_to_mol(const std::string &inchi, bool sanitize = true, bool remove_hydrogens = true);
    rust::String inchi_to_inchi_key(const std::string &inchi);
}