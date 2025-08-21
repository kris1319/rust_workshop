#pragma once

#include <vector>
#include <unordered_map>

enum class IndexType {
    Tree,
    HashMap,
};

class Index {
public:
    using WordIndex = std::unordered_map<std::string, std::vector<size_t>>;

    Index(IndexType type): _type(type) {}

    int dictionary_size() const {
        return _index.size();
    }

    IndexType index_type() const {
        return _type;
    }

    void index(const std::string& name, const std::string& text) {
        size_t i = 0;
        while (i < text.size()) {
            // Skip non-alpha characters
            while (i < text.size() && !(std::isalpha(text[i]) || std::isdigit(text[i]))) ++i;
            size_t start = i;
            // Find end of word
            while (i < text.size() && (std::isalpha(text[i]) || std::isdigit(text[i]))) ++i;
            if (start < i) {
                std::string word = text.substr(start, i - start);
                _index[word][name].push_back(start);
            }
        }
    }

    WordIndex search(const std::string& word) const {
        auto it = _index.find(word);
        if (it != _index.end()) {
            return it->second;
        }
        return {};
    }

private:
    IndexType _type;
    std::unordered_map<std::string, WordIndex> _index;
};
