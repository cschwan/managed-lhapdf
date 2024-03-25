#ifndef FAKE_LHAPDF_HPP
#define FAKE_LHAPDF_HPP

// The sole purpose of this file is to make this crate compile on `https://docs.rs`, where the
// LHAPDF library isn't installed. There we only care about the crate being documented, so don't
// actually need to implement LHAPDF ourselves.
//
// As a added advantage this file documents every class and function we use from LHAPDF.

#include <cassert>
#include <cstddef>
#include <map>
#include <string>
#include <utility>
#include <vector>

namespace LHAPDF {

std::vector<std::string> const& availablePDFSets() {
    assert(false);
}

void setVerbosity(int) {
}

int verbosity() {
    return 0;
}

struct PDFUncertainty {
    double central;
    double errplus;
    double errminus;
    double errsymm;
    double scale;
    double errplus_pdf;
    double errminus_pdf;
    double errsymm_pdf;
    double errplus_par;
    double errminus_par;
    double errsymm_par;
    double err_par;
};

struct PDFSet {
    PDFSet() = default;

    PDFSet(std::string const&) {
    }

    bool has_key(std::string const&) const {
        return false;
    }

    std::string const& get_entry(std::string const&) const {
        assert(false);
    }

    std::string errorType() const {
        return "";
    }

    std::string name() const {
        return "";
    }

    std::size_t size() const {
        return 0;
    }

    int lhapdfID() const {
        return 0;
    }

    PDFUncertainty uncertainty(std::vector<double> const&, double, bool) const {
        return PDFUncertainty();
    }
};

struct PDF {
    double alphasQ2(double) const {
        return 0.0;
    }

    double xfxQ2(int, double, double) const {
        return 0.0;
    }

    int lhapdfID() const {
        return 0;
    }

    std::vector<int> const& flavors() const {
        return flavors_;
    }

    void setFlavors(std::vector<int> const& flavors) {
        flavors_ = flavors;
    }

    void setForcePositive(int) {
    }

    int forcePositive() const {
        return 0;
    }

    PDFSet set() const {
        return PDFSet();
    }

    double xMin() {
        return 0.0;
    }

    double xMax() {
        return 1.0;
    }

private:
    std::vector<int> flavors_;
};

std::map<int, std::string>& getPDFIndex() {
    static std::map<int, std::string> pdf_index;
    return pdf_index;
}

PDF* mkPDF(std::string const&, int) {
    return new PDF();
}

std::pair<std::string, int> lookupPDF(int) {
    return std::make_pair(std::string(), -1);
}

}

#endif
