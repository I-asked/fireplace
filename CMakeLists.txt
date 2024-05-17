cmake_minimum_required(VERSION 3.25)

project(fireplace)

find_package(PkgConfig REQUIRED)
pkg_check_modules(MirAL miral REQUIRED IMPORTED_TARGET)

add_executable(fireplace
  kiosk_main.cpp
  kiosk_window_manager.cpp
)

target_link_libraries(fireplace
  PkgConfig::MirAL
)

install(TARGETS fireplace)

configure_file(
  ${CMAKE_CURRENT_SOURCE_DIR}/fireplace.service.in
  ${CMAKE_CURRENT_BINARY_DIR}/fireplace.service
  @ONLY
)

install(FILES 
  ${CMAKE_CURRENT_BINARY_DIR}/fireplace.service
  DESTINATION lib/systemd/user/
)
