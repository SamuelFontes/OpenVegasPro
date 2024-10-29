# variables
# Set the build options for OpenCV
set(BUILD_TESTS OFF CACHE BOOL "Disable OpenCV tests" FORCE)
set(BUILD_PERF_TESTS OFF CACHE BOOL "Disable OpenCV performance tests" FORCE)
set(BUILD_opencv_python2 OFF CACHE BOOL "Disable OpenCV Python 2 support" FORCE)
set(BUILD_opencv_python3 OFF CACHE BOOL "Disable OpenCV Python 3 support" FORCE)
set(OPENCV_PYTHON_SKIP_DETECTION ON)

# add folders
add_subdirectory(raylib)
add_subdirectory(opencv)

# define interface
add_library(external INTERFACE)

# Include directories for OpenCV
target_include_directories(external INTERFACE ${OpenCV_INCLUDE_DIRS})

# Add libraries to interface
target_link_libraries(external INTERFACE raylib)
target_include_directories(external INTERFACE "raygui/src/")
target_link_libraries(external INTERFACE opencv_videoio opencv_core)