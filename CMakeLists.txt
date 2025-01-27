cmake_minimum_required(VERSION 3.10)

project(NES_Emulator)

# Set C++ standard
set(CMAKE_CXX_STANDARD 17)

# Add the main executable
add_executable(nes-emulator src/main.cpp src/bus.cpp src/cpu.cpp)

target_compile_options(nes-emulator PRIVATE -Wall -Wextra -fsanitize=address,undefined)

target_link_libraries(nes-emulator PRIVATE -fsanitize=address -fsanitize=undefined)

# Optional target for Criterion tests
option(BUILD_CPU_TESTS "Build CPU tests with Criterion" OFF)

if(BUILD_CPU_TESTS)
    # Add a separate executable for tests
    add_executable(cpu-tests src/main.cpp src/bus.cpp src/cpu.cpp)

    # Define macro for Criterion tests
    target_compile_definitions(cpu-tests PRIVATE CRITERION_INCLUDE_PLS)

    # Find Criterion library
    find_library(CRITERION_LIB criterion REQUIRED)
    find_path(CRITERION_INCLUDE_DIR criterion/criterion.h REQUIRED)

    # Include Criterion headers and link the library
    target_include_directories(cpu-tests PRIVATE ${CRITERION_INCLUDE_DIR})
    target_link_libraries(cpu-tests PRIVATE ${CRITERION_LIB})
endif()
