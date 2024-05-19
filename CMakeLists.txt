cmake_minimum_required(VERSION 3.25)

project(fireplace)

find_package(PkgConfig REQUIRED)
pkg_check_modules(MirAL miral REQUIRED IMPORTED_TARGET)

add_executable(fireplace
  kiosk_main.cpp
  kiosk_window_manager.cpp
)

ExternalProject_Add(mediator_rs
  SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/mediator"
  BUILD_IN_SOURCE On
  DOWNLOAD_COMMAND ""
  CONFIGURE_COMMAND ""
  COMMAND cargo build --offline $<$<NOT:$<CONFIG:Debug>>:--release>
  INSTALL_COMMAND ""
  LOG_BUILD On
)

add_library(IMPORTED mediator STATIC IMPORTED)
set_target_properties(mediator
  IMPORTED_LOCATION "${CMAKE_SOURCE_DIR}/mediator/target/release/libmediator.a"
  IMPORTED_LOCATION_Debug "${CMAKE_SOURCE_DIR}/mediator/target/debug/libmediator.a"
)

add_dependencies(mediator mediator_rs)

target_link_libraries(fireplace
  mediator
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
