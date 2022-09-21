#!/bin/bash

w4 png2src --template sprites.rs.mustache unit4.png unit5.png unit4-attack*.png unit4-gun*.png base.png > src/sprites.rs

rustfmt src/sprites.rs
