#ifndef LIBFOO_LIBRARY_H
#define LIBFOO_LIBRARY_H

#include <iostream>
#include <string>

class CFoo {
public:
    int size;
    std::string name;
    CFoo() : size(2000),name("cmake foo") {
    }
    CFoo(const CFoo& cpy) : size(cpy.size),name(cpy.name) {
    }
    ~CFoo() {
        std::cout<<"CFoo drop"<<std::endl;
    }

    int get_size();

    std::string get_name();

};


#endif //LIBFOO_LIBRARY_H
