cmake_minimum_required(VERSION 3.22)

set(SCRS
    ${CMAKE_CURRENT_LIST_DIR}/motor_controller.c
)

add_library(motor_controller_module INTERFACE)
target_sources(motor_controller_module INTERFACE ${SCRS})
target_include_directories(motor_controller_module INTERFACE 
    root
)
target_compile_definitions(motor_controller_module INTERFACE root)
target_link_libraries(motor_controller_module INTERFACE
    root
)
