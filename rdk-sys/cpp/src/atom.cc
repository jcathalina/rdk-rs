#include "rust/cxx.h"
#include <GraphMol/Atom.h>
#include <GraphMol/PeriodicTable.h>

namespace RDKit {

    std::shared_ptr<Atom> atom_from_symbol(const std::string &symbol) {
        Atom atom = Atom(symbol);
        return std::make_shared<Atom>(atom);
    }

    rust::String get_symbol(std::shared_ptr<Atom> atom) {
        return atom->getSymbol();
    }
}