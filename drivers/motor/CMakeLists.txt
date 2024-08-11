cmake_minimum_required(VERSION 3.22)

set(SCRS
    ${CMAKE_CURRENT_LIST_DIR}/motor.c
)

add_library(motor_driver INTERFACE)
target_sources(motor_driver INTERFACE ${SCRS})
target_include_directories(motor_driver INTERFACE 
    root
)
target_compile_definitions(motor_driver INTERFACE root)
target_link_libraries(motor_driver INTERFACE
    root
)
