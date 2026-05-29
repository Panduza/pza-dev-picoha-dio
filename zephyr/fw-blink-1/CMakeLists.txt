cmake_minimum_required(VERSION 3.20.0)
include($ENV{ZEPHYR_BASE}/cmake/app/boilerplate.cmake NO_POLICY_SCOPE)
project(fw-blink-1)

target_sources(app PRIVATE src/main.c)