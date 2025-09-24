#include "SearchEngine.h"
#include "SIMD_Entropy.h"
#include "PrimeHilbertIndexer.h"

#include <filesystem>
#include <iostream>
#include <fstream>
#include <regex>

namespace fs = std::filesystem;

std::vector<SearchResult> SearchEngine::findMatchingFiles(
    const std::string& rootPath,
    const std::string& pattern,
    bool useEntropy
) {
    std::vector<SearchResult> results;
    std::regex regex_pattern(pattern, std::regex::icase);

    HilbertCoord hc = PrimeHilbertIndexer::hashFileToHilbert(result.path);
    std::cout << "Hilbert: (" << hc.x << ", " << hc.y << ", " << hc.z << ")\n";
    
    for (auto& p : fs::recursive_directory_iterator(rootPath, fs::directory_options::skip_permission_denied)) {
        try {
            if (!p.is_regular_file()) continue;
            std::string filepath = p.path().string();

            if (std::regex_search(filepath, regex_pattern)) {
                SearchResult result;
                result.path = filepath;
                result.entropyScore = useEntropy ? SIMD_Entropy::calculateFileEntropy(filepath) : 0.0;
                results.push_back(result);
            }
        } catch (...) {
            continue;
        }
    }

    return results;
}

void SearchEngine::printResults(const std::vector<SearchResult>& results) {
    for (const auto& r : results) {
        std::cout << r.path;
        if (r.entropyScore > 0.0)
            std::cout << " | Entropy: " << r.entropyScore;
        std::cout << "\n";
    }
}
