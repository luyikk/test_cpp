#include "library.h"

int CFoo::get_size() {
    return this->size;
}

std::string CFoo::get_name() {
    return this->name;
}

Result CFoo::GetResult() {
    auto result = Result();
    result.data.emplace_back(10,11);
    return result;
}
