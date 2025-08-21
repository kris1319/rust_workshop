#include "rust_cxx/include/index.h"
#include "rust_cxx/src/main.rs.h"
#include "rust/cxx.h"

std::unique_ptr<Index> new_index(IndexType type);

rust::Vec<rust::String> search_index(const Index& index, rust::Str word);

struct IndexResult;

rust::Vec<IndexResult> search_index_with_positions(const Index& index, rust::Str word);