cmake_minimum_required(VERSION 3.26)
project(libfoo)

set(CMAKE_CXX_STANDARD 20)

add_library(libfoo library.cpp)
include(GenerateExportHeader)
generate_export_header(libfoo)