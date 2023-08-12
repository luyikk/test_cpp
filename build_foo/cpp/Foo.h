
#include <iostream>
#include <string>

class Foo {
public:
    int size;
    std::string name;
    Foo() : size(1002),name("foo") {
    }
    Foo(const Foo& cpy) : size(cpy.size),name(cpy.name) {
    }
    ~Foo() {
        std::cout<<"drop"<<std::endl;
    }

    int get_size();
    std::string get_name();
};

