# variables
set(BUILD_TESTS "0")
set(BUILD_PERF_TESTS "0")
set(BUILD_opencv_python2 "0")
set(BUILD_opencv_python3 "0")
set(OpenCV_FIND_COMPONENTS "")
set(OpenCV_EXTRA_COMPONENTS "")
set(OPENCV_PYTHON_SKIP_DETECTION ON)

# add folders
add_subdirectory(raylib)
add_subdirectory(opencv)

# define interface
add_library(external INTERFACE)

# Set C++ standard for OpenCV if needed
set_target_properties(opencv_core PROPERTIES CXX_STANDARD 11)

# add libraries to interface
target_link_libraries(external INTERFACE raylib)
target_include_directories(external INTERFACE "raygui/src/")
target_link_libraries(external INTERFACE opencv_core opencv_imgproc opencv_highgui opencv_videoio)

