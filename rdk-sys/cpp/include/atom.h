#pragma once

#include "rust/cxx.h"
#include <GraphMol/Atom.h>
#include <GraphMol/PeriodicTable.h>

namespace RDKit {
    std::shared_ptr<Atom> atom_from_symbol(const std::string &symbol);
    rust::String get_symbol(std::shared_ptr<Atom> atom);
}