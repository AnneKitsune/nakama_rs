#!/bin/sh
bindgen wrapper.h -- -I nakama-cpp-sdk/include/ > src/bindings.rs
