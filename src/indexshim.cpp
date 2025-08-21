#include "rust_cxx/include/indexshim.h"

std::unique_ptr<Index> new_index(IndexType type) {
  return std::make_unique<Index>(type);
}

rust::Vec<rust::String> search_index(const Index& index, rust::Str word) {
    auto results = index.search((std::string)word);
    rust::Vec<rust::String> rust_results;
    for (const auto& [doc_name, positions] : results) {
        rust_results.push_back(doc_name);
    }
    return rust_results;
}

rust::Vec<IndexResult> search_index_with_positions(const Index& index, rust::Str word) {
    auto results = index.search((std::string)word);
    rust::Vec<IndexResult> rust_results;
    for (const auto& [doc_name, positions] : results) {
        rust::Vec<unsigned> rust_positions;
        for (size_t pos : positions) {
            rust_positions.push_back(static_cast<unsigned>(pos));
        }
        rust_results.push_back(IndexResult{doc_name, std::move(rust_positions)});
    }
    return rust_results;
}