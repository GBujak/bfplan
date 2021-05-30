#!/bin/bash

set -xe

pandoc sprawozdanie.md -o sprawozdanie.pdf --citeproc
