add_library(
    audio-dependencies
    STATIC
    alsa.c
    common.c
    pulse.c
)

target_link_libraries(audio-dependencies PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})