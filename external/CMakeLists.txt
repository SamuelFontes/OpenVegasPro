add_subdirectory(raylib)

add_library(external INTERFACE)

target_link_libraries(external INTERFACE raylib)
target_include_directories(external INTERFACE "raygui/src/")

