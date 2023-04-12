#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/inchi.h>

namespace RDKit
{
    rust::String mol_to_inchi(std::shared_ptr<ROMol> mol)
    {
        ExtraInchiReturnValues ret_vals;
        // TODO: add options from const char* to this fn
        rust::String inchi = MolToInchi(*mol, ret_vals, "");
        return inchi;
    }

    std::shared_ptr<ROMol> inchi_to_mol(const std::string &inchi, bool sanitize, bool remove_hydrogens)
    {
        ExtraInchiReturnValues ret_vals;
        ROMol *mol = InchiToMol(inchi, ret_vals, sanitize, remove_hydrogens);
        return std::shared_ptr<ROMol>(mol);
    }

    rust::String inchi_to_inchi_key(const std::string &inchi)
    {
        return InchiToInchiKey(inchi);
    }
}
